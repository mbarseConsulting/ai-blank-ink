import { CommonModule } from '@angular/common';
import {
  ChangeDetectorRef,
  Component,
  NgZone,
  OnDestroy,
  OnInit,
  inject,
} from '@angular/core';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { SkillRunModel, SkillName } from './models';
import { AppConfigService } from './app-config.service';

@Component({
  selector: 'app-analysis-shell',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './analysis-shell.component.html',
  styleUrls: ['./analysis-shell.component.css'],
})
export class AnalysisShellComponent implements OnInit, OnDestroy {
  private zone = inject(NgZone);
  private cdr = inject(ChangeDetectorRef);
  private sanitizer = inject(DomSanitizer);
  private configService = inject(AppConfigService);

  activeDocumentPath: string | null = null;
  activeSkillName: SkillName | null = null;
  activeRun: SkillRunModel | null = null;

  /** Historique des runs, indexé par "path::skillName". */
  private runsByKey: Record<string, SkillRunModel[]> = {};
  private activeKey: string | null = null;
  activeIndex: number = 0;
  /** Onglets ouverts pour le document courant (skills ayant déjà un historique). */
  openSkills: SkillName[] = [];

  private unlisten: UnlistenFn | null = null;
  private unlistenInit: UnlistenFn | null = null;

  async ngOnInit(): Promise<void> {
    console.log('AnalysisShellComponent ngOnInit – setting up listener for skill-run');
    this.unlisten = await listen<{
      documentPath: string;
      skillName: SkillName;
      run: SkillRunModel;
    }>('skill-run', (event) => {
      const payload = event.payload;
      // S'assurer que l'update se fait dans la zone Angular
      this.zone.run(async () => {
        console.log('Analysis desk received skill-run event', {
          documentPath: payload.documentPath,
          skillName: payload.skillName,
          diagnosticsCount: payload.run.diagnostics?.length ?? 0,
        });

        this.activeDocumentPath = payload.documentPath;
        this.activeSkillName = payload.skillName;

        const key = this.makeKey(payload.documentPath, payload.skillName);
        // Charger l'historique existant si on ne l'a pas encore.
        if (!this.runsByKey[key]) {
          try {
            const existing = await invoke<SkillRunModel[]>('load_skill_run_history', {
              path: payload.documentPath,
              skillName: payload.skillName,
            });
            this.runsByKey[key] = existing ?? [];
          } catch (error) {
            console.error('Failed to load skill run history', error);
            this.runsByKey[key] = [];
          }
        }

        this.runsByKey[key].push(payload.run);
        if (!this.openSkills.includes(payload.skillName)) {
          this.openSkills.push(payload.skillName);
        }
        this.activeKey = key;
        this.activeIndex = this.runsByKey[key].length - 1;
        this.activeRun = payload.run;

        // Sauvegarder l'historique complet pour ce couple (doc, skill).
        try {
          await invoke('save_skill_run_history', {
            path: payload.documentPath,
            skillName: payload.skillName,
            runs: this.runsByKey[key],
          });
        } catch (error) {
          console.error('Failed to save skill run history', error);
        }

        this.cdr.detectChanges();
      });
    });

    // Écouter le contexte initial envoyé depuis la fenêtre principale
    this.unlistenInit = await listen<{ documentPath: string }>('analysis-init', async (event) => {
      const { documentPath } = event.payload;
      this.zone.run(async () => {
        console.log('Analysis desk received analysis-init', { documentPath });
        this.activeDocumentPath = documentPath;
        this.openSkills = [];
        const supportedSkills = this.configService.get().skills.supportedForAnalysisDesk as SkillName[];

        for (const skill of supportedSkills) {
          const key = this.makeKey(documentPath, skill);
          try {
            const existing = await invoke<SkillRunModel[]>('load_skill_run_history', {
              path: documentPath,
              skillName: skill,
            });
            const runs = existing ?? [];
            this.runsByKey[key] = runs;
            if (runs.length > 0) {
              this.openSkills.push(skill);
            }
          } catch (error) {
            console.error('Failed to load history on analysis-init for', skill, error);
            this.runsByKey[key] = [];
          }
        }

        const defaultOrder = this.configService.get().skills.defaultOrder as SkillName[];
        let chosen: SkillName | null = null;
        for (const skill of defaultOrder) {
          if (this.openSkills.includes(skill)) {
            chosen = skill;
            break;
          }
        }

        if (chosen) {
          this.activeSkillName = chosen;
          const key = this.makeKey(documentPath, chosen);
          this.setActiveFromKey(key);
        } else {
          this.activeSkillName = null;
          this.activeKey = null;
          this.activeIndex = 0;
          this.activeRun = null;
        }

        this.cdr.detectChanges();
      });
    });
  }

  ngOnDestroy(): void {
    if (this.unlisten) {
      this.unlisten();
      this.unlisten = null;
    }
    if (this.unlistenInit) {
      this.unlistenInit();
      this.unlistenInit = null;
    }
  }

  private makeKey(path: string, skill: SkillName): string {
    return `${path}::${skill}`;
  }

  get renderedReportHtml(): SafeHtml | null {
    const msg = this.activeRun?.diagnostics?.[0]?.message;
    if (!msg) return null;
    try {
      const html = this.simpleMarkdownToHtml(msg);
      return this.sanitizer.bypassSecurityTrustHtml(html);
    } catch {
      // Fallback si le parsing markdown plante (ex. contenu inattendu)
      const escaped = msg
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/\n/g, '<br>');
      return this.sanitizer.bypassSecurityTrustHtml('<p>' + escaped + '</p>');
    }
  }

  private simpleMarkdownToHtml(text: string): string {
    let out = text
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
    out = out.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>');
    out = out.replace(/\*(.+?)\*/g, '<em>$1</em>');
    out = out.replace(/^[-*] (.+)$/gm, '<li>$1</li>');
    out = out.replace(/(<li>.*<\/li>\n?)+/gs, (m) => `<ul>${m}</ul>`);
    out = out.replace(/\n\n/g, '</p><p>');
    out = out.replace(/\n/g, '<br>');
    return '<p>' + out + '</p>';
  }

  get currentRuns(): SkillRunModel[] {
    if (!this.activeDocumentPath || !this.activeSkillName) {
      return [];
    }
    const key = this.makeKey(this.activeDocumentPath, this.activeSkillName);
    return this.runsByKey[key] ?? [];
  }

  get analysisDeskTitle(): string {
    return this.configService.get().i18n.analysisDeskTitle;
  }
  get analysisHistoryTitle(): string {
    return this.configService.get().i18n.analysisHistoryTitle;
  }
  get noReportPlaceholder(): string {
    return this.configService.get().i18n.noReportPlaceholder;
  }
  get noReportHint(): string {
    return this.configService.get().i18n.noReportHint;
  }
  get noDocSubtitle(): string {
    return this.configService.get().i18n.noDocSubtitle;
  }

  selectRun(index: number): void {
    const runs = this.currentRuns;
    if (!runs[index]) return;
    this.activeIndex = index;
    this.activeRun = runs[index];
    this.cdr.detectChanges();
  }

  selectSkillTab(skill: SkillName): void {
    if (!this.activeDocumentPath) return;
    this.activeSkillName = skill;
    const key = this.makeKey(this.activeDocumentPath, skill);
    // Charger l'historique si nécessaire.
    if (!this.runsByKey[key]) {
      invoke<SkillRunModel[]>('load_skill_run_history', {
        path: this.activeDocumentPath,
        skillName: skill,
      })
        .then((existing) => {
          this.runsByKey[key] = existing ?? [];
          this.setActiveFromKey(key);
        })
        .catch((error) => {
          console.error('Failed to load history when selecting tab', error);
          this.runsByKey[key] = [];
          this.setActiveFromKey(key);
        });
    } else {
      this.setActiveFromKey(key);
    }
  }

  closeSkillTab(skill: SkillName, event: MouseEvent): void {
    event.stopPropagation();
    this.openSkills = this.openSkills.filter((s) => s !== skill);

    if (this.activeSkillName === skill) {
      // Choisir un autre onglet actif si possible.
      const nextSkill = this.openSkills[0] ?? null;
      this.activeSkillName = nextSkill;
      if (nextSkill && this.activeDocumentPath) {
        const key = this.makeKey(this.activeDocumentPath, nextSkill);
        this.setActiveFromKey(key);
      } else {
        this.activeKey = null;
        this.activeIndex = 0;
        this.activeRun = null;
      }
    }
    this.cdr.detectChanges();
  }

  private setActiveFromKey(key: string): void {
    this.activeKey = key;
    const runs = this.runsByKey[key] ?? [];
    if (runs.length > 0) {
      this.activeIndex = runs.length - 1;
      this.activeRun = runs[this.activeIndex];
    } else {
      this.activeIndex = 0;
      this.activeRun = null;
    }
    this.cdr.detectChanges();
  }
}

