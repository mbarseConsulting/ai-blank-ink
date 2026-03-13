import { CommonModule } from '@angular/common';
import {
  ChangeDetectorRef,
  Component,
  HostListener,
  OnInit,
  ViewChild,
  inject,
} from '@angular/core';
import { FormsModule } from '@angular/forms';
import { invoke } from '@tauri-apps/api/core';
import { emit } from '@tauri-apps/api/event';
import {
  DocumentModel,
  DocumentNode,
  PatchModel,
  SkillName,
  SkillRunModel,
} from './models';
import { DocumentEditorComponent } from './document-editor.component';
import { SkillsPanelComponent } from './skills-panel.component';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    DocumentEditorComponent,
    SkillsPanelComponent,
  ],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css'],
})
export class App implements OnInit {
  private cdr = inject(ChangeDetectorRef);

  /** Arbre de documents, rempli par le scan Tauri de stories/. */
  documentTree: DocumentNode[] = [];

  /** IDs des dossiers repliés. Vide = tous ouverts. */
  collapsedFolderIds = new Set<string>();

  /** En cours de chargement de l'arbre. */
  treeLoading = false;

  ngOnInit(): void {
    this.loadStoriesTree();
  }

  refreshStoriesTree(): void {
    this.loadStoriesTree();
  }

  toggleFolder(node: DocumentNode): void {
    if (this.collapsedFolderIds.has(node.id)) {
      this.collapsedFolderIds.delete(node.id);
    } else {
      this.collapsedFolderIds.add(node.id);
    }
    this.collapsedFolderIds = new Set(this.collapsedFolderIds);
  }

  isFolderExpanded(node: DocumentNode): boolean {
    return !this.collapsedFolderIds.has(node.id);
  }

  async loadStoriesTree(): Promise<void> {
    this.treeLoading = true;
    this.cdr.detectChanges();
    try {
      const tree = await invoke<DocumentNode[]>('list_stories_tree');
      this.documentTree = tree ?? [];
    } catch (error) {
      console.error('Failed to load stories tree', error);
      this.documentTree = [];
    } finally {
      this.treeLoading = false;
      this.cdr.detectChanges();
    }
  }

  availableSkills: { name: SkillName; label: string; description: string }[] = [
    {
      name: 'calibrate-ink',
      label: 'Calibrer (genre)',
      description: 'Fixer genre et conventions de lecture.',
    },
    {
      name: 'arch-ink',
      label: 'Structure',
      description: 'Analyse des actes, arcs, climax.',
    },
    {
      name: 'qa-reader',
      label: 'QA lecture',
      description: 'Hooks, tension, rythme, engagement.',
    },
    {
      name: 'qa-originality',
      label: 'Originalité',
      description: 'Clichés vs singularité de la voix.',
    },
    {
      name: 'edit-ai-fr',
      label: 'Langue FR',
      description: 'Nettoyage artefacts IA + français.',
    },
  ];

  currentDocument: DocumentModel = {
    id: 'untitled',
    title: 'Aucun document ouvert',
    status: 'draft',
    content: '',
    history: [],
  };

  pendingPatches: PatchModel[] = [];
  selectedSkillName: SkillName | null = null;
  promptText = '';
  /** Message bref après sauvegarde (ex. "Enregistré", "Erreur"). */
  saveIndicator = '';
  /** Dernier diagnostic textuel (ex. qa-reader) affiché dans le panneau droit. */
  lastDiagnosticMessage: string | null = null;
  /** Question de suivi pour le panneau de diagnostic. */
  diagnosticFollowupText: string = '';

  @ViewChild('documentEditor', { static: false })
  editorComponent?: DocumentEditorComponent;

  get currentDocumentSafe(): DocumentModel {
    return this.currentDocument;
  }

  get selectedSkillDefinition():
    | { name: SkillName; label: string; description: string }
    | null {
    if (!this.selectedSkillName) return null;
    return this.availableSkills.find((s) => s.name === this.selectedSkillName) ?? null;
  }

  openAnalysisDesk(): void {
    if (!this.currentDocument.path) return;
    const currentPath = this.currentDocument.path;
    invoke('open_analysis_window', { path: currentPath })
      .then(() => {
        // Petite attente pour laisser le temps à la fenêtre d'analyse
        // de se charger et d'attacher ses listeners.
        setTimeout(() => {
          console.log('Emitting analysis-init for', currentPath);
          emit('analysis-init', {
            documentPath: currentPath,
          }).catch((err) => console.error('Failed to emit analysis-init', err));
        }, 800);
      })
      .catch((err) => console.error('Failed to open analysis window', err));
  }

  async onNodeClick(node: DocumentNode, event: MouseEvent): Promise<void> {
    event.stopPropagation();

    if (node.kind === 'folder') {
      // Later: gérer l'état ouvert/fermé. Pour l’instant, clic sur dossier ne fait rien.
      this.toggleFolder(node);
      return;
    }

    try {
      const content = await invoke<string>('open_document', { path: node.path });

      this.currentDocument = {
        id: node.id,
        title: node.label,
        status: 'draft',
        path: node.path,
        content,
        history: [],
      };
      // Clear any suggestions from the previous document.
      this.pendingPatches = [];
    } catch (error) {
      // Future: afficher un toast / message d’erreur.
      console.error('Failed to open document', error);
    }
  }

  selectSkill(name: SkillName): void {
    this.selectedSkillName = name;
  }

  async runSelectedSkill(): Promise<void> {
    if (!this.selectedSkillName || !this.currentDocument.path) return;

    try {
      if (this.selectedSkillName === 'qa-reader') {
        // Premier clic : expliquer ce que fait le skill et comment formuler la demande,
        // sans appeler immédiatement l'API.
        this.lastDiagnosticMessage =
          "QA lecture (/qa-reader) peut analyser l’expérience de lecture de ton texte.\n\n" +
          "- Hooks : est-ce que le début donne envie de continuer ?\n" +
          "- Rythme : alternance lenteur / accélération, passages trop statiques.\n" +
          "- Tension : où la tension monte, où elle retombe trop.\n" +
          "- Engagement : où le lecteur risque de décrocher.\n\n" +
          "Dans le champ ci-dessous, précise ce que tu veux :\n" +
          'Exemples :\n' +
          '- \"Concentre-toi sur le rythme de la scène 1.\"\n' +
          '- \"Dis-moi où la tension retombe dans la scène 2.\"\n' +
          '- \"Analyse uniquement l’ouverture et dis-moi si le hook fonctionne.\"';
        this.diagnosticFollowupText = '';
        this.cdr.detectChanges();
        return;
      }

      // Pour les autres skills, on reste sur le mock Tauri qui renvoie un patch.
      const run = await invoke<SkillRunModel>('run_skill_mock', {
        skillName: this.selectedSkillName,
        path: this.currentDocument.path,
        mode: 'analysis',
      });
      if (!run || !run.patches?.length) return;
      this.pendingPatches = [...this.pendingPatches, ...run.patches];
      this.cdr.detectChanges();
    } catch (error) {
      console.error('Failed to run skill', error);
    }
  }

  acceptPatch(patch: PatchModel): void {
    this.pendingPatches = this.pendingPatches.filter((p) => p.id !== patch.id);
    this.cdr.detectChanges();
  }

  rejectPatch(patch: PatchModel): void {
    this.pendingPatches = this.pendingPatches.filter((p) => p.id !== patch.id);
    this.cdr.detectChanges();
  }

  async sendDiagnosticFollowup(): Promise<void> {
    const text = this.diagnosticFollowupText.trim();
    if (!text || !this.currentDocument.path || this.selectedSkillName !== 'qa-reader') {
      return;
    }
    try {
      const run = await invoke<SkillRunModel>('run_skill', {
        skillName: this.selectedSkillName,
        path: this.currentDocument.path,
        mode: 'analysis',
        followUp: text,
        // On envoie toujours le texte complet + l’instruction détaillée;
        // pas besoin de renvoyer le message précédent en contexte pour le premier vrai run.
        previousMessage: null,
      });
      console.log('qa-reader run completed, emitting skill-run event', {
        path: this.currentDocument.path,
        skill: this.selectedSkillName,
        diagnosticsCount: run.diagnostics?.length ?? 0,
      });
      const first = run.diagnostics?.[0];
      this.lastDiagnosticMessage = first?.message ?? null;
      this.diagnosticFollowupText = '';
      // Diffuser le SkillRun complet vers la fenêtre d'analyse (si ouverte).
      emit('skill-run', {
        documentPath: this.currentDocument.path,
        skillName: this.selectedSkillName,
        run,
      })
        .then(() => {
          console.log('skill-run event emitted');
        })
        .catch((err) => console.error('Failed to emit skill-run event', err));
      this.cdr.detectChanges();
    } catch (error) {
      console.error('Failed to continue diagnostic conversation', error);
    }
  }

  @HostListener('document:keydown', ['$event'])
  onKeyDown(event: KeyboardEvent): void {
    if ((event.ctrlKey || event.metaKey) && event.key === 's') {
      event.preventDefault();
      this.saveCurrentDocument();
    }
  }

  async saveCurrentDocument(): Promise<void> {
    if (!this.currentDocument.path) {
      return;
    }

    try {
      await invoke('save_document', {
        path: this.currentDocument.path,
        content: this.currentDocument.content,
      });
      this.saveIndicator = 'Enregistré';
      this.cdr.detectChanges();
      setTimeout(() => {
        this.saveIndicator = '';
        this.cdr.detectChanges();
      }, 2000);
    } catch (error) {
      console.error('Failed to save document', error);
      this.saveIndicator = 'Erreur';
      this.cdr.detectChanges();
      setTimeout(() => {
        this.saveIndicator = '';
        this.cdr.detectChanges();
      }, 2000);
    }
  }

  sendPrompt(): void {
    if (!this.promptText.trim()) return;
    // Future: route to Tauri command / subagent
    this.promptText = '';
  }
}
