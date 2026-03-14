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
      label: 'calibrate-ink',
      description: 'Fixer genre et conventions de lecture (calibration).',
    },
    {
      name: 'qa-prose',
      label: 'qa-prose',
      description: 'POV, show/tell, description, dialogue (ligne à ligne).',
    },
    {
      name: 'arch-ink',
      label: 'arch-ink',
      description: 'Analyse structurelle (acts, arcs, climax).',
    },
    {
      name: 'qa-reader',
      label: 'qa-reader',
      description: 'Hooks, tension, rythme, engagement (expérience de lecture).',
    },
    {
      name: 'qa-originality',
      label: 'qa-originality',
      description: 'Clichés vs singularité de la voix et des idées.',
    },
    {
      name: 'qa-characters',
      label: 'qa-characters',
      description: 'Psychologie, relations, crédibilité des personnages.',
    },
    {
      name: 'qa-consistency',
      label: 'qa-consistency',
      description: 'Cohérence factuelle : objets, chronologie, lore, arcs.',
    },
    {
      name: 'edit-ai-fr',
      label: 'edit-ai-fr',
      description: 'Nettoyage artefacts IA + corrections de français.',
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
      // Ouvrir / rafraîchir le bureau d'analyse pour ce document.
      this.openAnalysisDesk();
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

      if (this.selectedSkillName === 'qa-originality') {
        this.lastDiagnosticMessage =
          "Originalité (/qa-originality) peut analyser la singularité créative de ton texte.\n\n" +
          "- Voix : personnalité de la narration, ton, point de vue.\n" +
          "- Clichés : formulations attendues, images vues et revues.\n" +
          "- Idées : concept de la scène / nouvelle par rapport à des tropes connus.\n\n" +
          "Dans le champ ci-dessous, précise ce que tu veux :\n" +
          'Exemples :\n' +
          '- \"Dis-moi si cette scène d’ouverture ressemble trop à un trope connu.\"\n' +
          '- \"Analyse la voix de Julia : est-elle assez singulière ?\"\n' +
          '- \"Repère les endroits où je retombe dans des clichés de thriller érotique.\"';
        this.diagnosticFollowupText = '';
        this.cdr.detectChanges();
        return;
      }

      if (this.selectedSkillName === 'qa-prose') {
        this.lastDiagnosticMessage =
          "QA prose (/qa-prose) peut analyser la qualité de la phrase et du paragraphe.\n\n" +
          "- POV : cohérence du point de vue, distance au personnage.\n" +
          "- Show vs tell : là où tu expliques au lieu de faire vivre.\n" +
          "- Description : clarté sensorielle, surcharge, clichés.\n" +
          "- Dialogue : naturel, sous-texte, rythme.\n\n" +
          "Dans le champ ci-dessous, précise ce que tu veux :\n" +
          'Exemples :\n' +
          '- \"Analyse uniquement les dialogues de cette scène.\"\n' +
          '- \"Montre-moi où je suis trop explicatif / en mode tell.\"\n' +
          '- \"Repère les phrases les plus lourdes et propose des pistes d’allègement.\"';
        this.diagnosticFollowupText = '';
        this.cdr.detectChanges();
        return;
      }

      if (this.selectedSkillName === 'qa-characters') {
        this.lastDiagnosticMessage =
          "Personnages (/qa-characters) analyse la crédibilité et la cohérence psychologique des personnages.\n\n" +
          "- Psychologie : motivations, arcs émotionnels, vraisemblance.\n" +
          "- Relations : dynamiques interpersonnelles, chimie, conflits.\n" +
          "- Crédibilité : réactions plausibles, cohérence des choix.\n\n" +
          "Dans le champ ci-dessous, précise ce que tu veux :\n" +
          "Exemples :\n" +
          '- "Analyse Julia : ses motivations sont-elles claires ?"\n' +
          '- "Est-ce que la relation avec l\'inconnu sonne vraie ?"\n' +
          '- "Repère les moments où les personnages agissent de façon peu crédible."';
        this.diagnosticFollowupText = '';
        this.cdr.detectChanges();
        return;
      }

      if (this.selectedSkillName === 'qa-consistency') {
        this.lastDiagnosticMessage =
          "Cohérence (/qa-consistency) vérifie la continuité factuelle du récit.\n\n" +
          "- Objets : présence, localisation, usage cohérent.\n" +
          "- Chronologie : ordre des événements, ellipses, contradictions.\n" +
          "- Lore / univers : règles établies, cohérence interne.\n" +
          "- Arcs narratifs : promesses tenues ou oubliées.\n\n" +
          "Dans le champ ci-dessous, précise ce que tu veux :\n" +
          "Exemples :\n" +
          '- "Vérifie la chronologie de la scène 1 à 3."\n' +
          '- "Y a-t-il des incohérences sur le carton noir ou les lieux ?"\n' +
          '- "Repère les détails qui ne collent pas avec ce qui a été établi."';
        this.diagnosticFollowupText = '';
        this.cdr.detectChanges();
        return;
      }

      if (this.selectedSkillName === 'edit-ai-fr') {
        this.lastDiagnosticMessage =
          "Langue FR (/edit-ai-fr) peut proposer des corrections détaillées de la langue et nettoyer les artefacts IA.\n\n" +
          "- Orthographe, grammaire, accords.\n" +
          "- Lourdeurs de phrase et répétitions mécaniques.\n" +
          "- Tics de langage IA et formulations trop génériques.\n\n" +
          "Dans le champ ci-dessous, précise ce que tu veux :\n" +
          'Exemples :\n' +
          '- \"Nettoie toute la scène mais garde mon style.\"\\n' +
          '- \"Corrige surtout les dialogues.\"\\n' +
          '- \"Propose seulement des corrections là où le français est vraiment bancal.\"';
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

  onDiagnosticsKeydown(event: KeyboardEvent): void {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      this.sendDiagnosticFollowup();
    }
  }

  async sendDiagnosticFollowup(): Promise<void> {
    const text = this.diagnosticFollowupText.trim();
    if (!text || !this.currentDocument.path || !this.selectedSkillName) {
      return;
    }
    if (
      this.selectedSkillName !== 'qa-reader' &&
      this.selectedSkillName !== 'qa-originality' &&
      this.selectedSkillName !== 'qa-prose' &&
      this.selectedSkillName !== 'qa-characters' &&
      this.selectedSkillName !== 'qa-consistency' &&
      this.selectedSkillName !== 'edit-ai-fr'
    ) {
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
      console.log('skill run completed, emitting skill-run event', {
        path: this.currentDocument.path,
        skill: this.selectedSkillName,
        diagnosticsCount: run.diagnostics?.length ?? 0,
      });
      const first = run.diagnostics?.[0];
      this.lastDiagnosticMessage = first?.message ?? null;
      // Si edit-ai-fr renvoie des patches, les ajouter aux suggestions inline.
      if (this.selectedSkillName === 'edit-ai-fr' && run.patches?.length) {
        this.pendingPatches = [...this.pendingPatches, ...run.patches];
      }
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
