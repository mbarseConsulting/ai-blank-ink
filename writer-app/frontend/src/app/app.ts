import { CommonModule } from '@angular/common';
import { ChangeDetectorRef, Component, HostListener, inject, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { invoke } from '@tauri-apps/api/core';
import { DocumentModel, DocumentNode, PatchModel, SkillName } from './models';
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

  get currentDocumentSafe(): DocumentModel {
    return this.currentDocument;
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
    } catch (error) {
      // Future: afficher un toast / message d’erreur.
      console.error('Failed to open document', error);
    }
  }

  selectSkill(name: SkillName): void {
    this.selectedSkillName = name;
  }

  runSelectedSkill(): void {
    if (!this.selectedSkillName) return;

    const beforeSnippet = this.currentDocument.content.split('\n').slice(5, 9).join('\n');
    const mockPatch: PatchModel = {
      id: `patch-${Date.now()}`,
      skillRunId: `run-${Date.now()}`,
      startOffset: 0,
      endOffset: beforeSnippet.length,
      beforeText: beforeSnippet,
      afterText: beforeSnippet.replace(
        'Les rayonnages montaient haut',
        '[QA lecture] Les rayonnages montaient haut'
      ),
      status: 'pending',
      axis: `${this.selectedSkillName}:mock`,
      note: 'Mock qa-reader suggestion on the opening paragraph.',
    };

    this.pendingPatches = [mockPatch];
  }

  acceptPatch(patch: PatchModel): void {
    // naive apply based on start/end offsets on current content
    const before = this.currentDocument.content.slice(0, patch.startOffset);
    const after = this.currentDocument.content.slice(patch.endOffset);
    this.currentDocument = {
      ...this.currentDocument,
      content: before + patch.afterText + after,
    };
    this.pendingPatches = this.pendingPatches.filter((p) => p.id !== patch.id);
  }

  rejectPatch(patch: PatchModel): void {
    this.pendingPatches = this.pendingPatches.filter((p) => p.id !== patch.id);
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
