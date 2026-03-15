import { CommonModule } from '@angular/common';
import {
  AfterViewInit,
  Component,
  ElementRef,
  EventEmitter,
  Input,
  OnChanges,
  OnDestroy,
  Output,
  SimpleChanges,
  ViewChild,
} from '@angular/core';
import { FormsModule } from '@angular/forms';
import { basicSetup } from 'codemirror';
import { EditorState, Range } from '@codemirror/state';
import {
  Decoration,
  DecorationSet,
  EditorView,
  WidgetType,
} from '@codemirror/view';
import { markdown } from '@codemirror/lang-markdown';
import { DocumentModel, PatchModel } from './models';

@Component({
  selector: 'app-document-editor',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './document-editor.component.html',
  styleUrls: ['./document-editor.component.css'],
})
export class DocumentEditorComponent
  implements AfterViewInit, OnDestroy, OnChanges
{
  @Input() document!: DocumentModel;
  @Output() documentContentChange = new EventEmitter<string>();
  @Input() patches: PatchModel[] = [];
  @Output() acceptPatch = new EventEmitter<PatchModel>();
  @Output() rejectPatch = new EventEmitter<PatchModel>();

  /** Font family id currently selected. */
  fontId: string = 'georgia';
  /** Font size token currently selected. */
  fontSize: string = 'medium';

  fontOptions = [
    { id: 'georgia', label: 'Georgia' },
    { id: 'times', label: 'Times New Roman' },
    { id: 'garamond', label: 'Garamond' },
    { id: 'palatino', label: 'Palatino' },
    { id: 'arial', label: 'Arial' },
    { id: 'helvetica', label: 'Helvetica' },
    { id: 'inter', label: 'Inter' },
    { id: 'roboto', label: 'Roboto' },
    { id: 'jetbrains', label: 'JetBrains Mono' },
    { id: 'consolas', label: 'Consolas' },
  ];

  sizeOptions = [
    { id: 'small', label: 'Small' },
    { id: 'medium', label: 'Medium' },
    { id: 'large', label: 'Large' },
  ];

  @ViewChild('editorHost', { static: true })
  editorHost!: ElementRef<HTMLDivElement>;

  private view: EditorView | null = null;
  private updatingFromInput = false;
  private patchActionListener = (event: Event) => {
    const anyEvent = event as CustomEvent<{ id: string; action: 'accept' | 'reject' }>;
    const detail = anyEvent.detail;
    if (!detail) return;
    const patch = this.patches.find((p) => p.id === detail.id);
    if (!patch) return;
    if (detail.action === 'accept') {
      this.onAcceptPatchClick(patch);
    } else if (detail.action === 'reject') {
      this.onRejectPatchClick(patch);
    }
  };

  ngAfterViewInit(): void {
    const state = this.createEditorState(this.document?.content ?? '');
    this.view = new EditorView({
      state,
      parent: this.editorHost.nativeElement,
    });
    window.addEventListener('patchAction', this.patchActionListener as EventListener);
  }

  ngOnChanges(changes: SimpleChanges): void {
    if (!this.view) return;
    const docChanged = !!changes['document'] && this.document;
    const patchesChanged = !!changes['patches'];

    if (docChanged || patchesChanged) {
      const newText = this.document.content ?? '';
      const currentText = this.view.state.doc.toString();
      if (newText !== currentText) {
        this.updatingFromInput = true;
        const state = this.createEditorState(newText);
        this.view.setState(state);
        this.updatingFromInput = false;
      } else if (patchesChanged) {
        // Rebuild only extensions (patch decorations) while keeping current text.
        const state = this.createEditorState(currentText);
        this.view.setState(state);
      }
    }
  }

  ngOnDestroy(): void {
    window.removeEventListener('patchAction', this.patchActionListener as EventListener);
    this.view?.destroy();
  }

  onAcceptPatchClick(patch: PatchModel) {
    // Apply the patch directly in CodeMirror so offsets always line up,
    // then let the parent know to clear it from the pending list.
    if (this.view) {
      const tr = this.view.state.update({
        changes: {
          from: patch.startOffset,
          to: patch.endOffset,
          insert: patch.afterText,
        },
      });
      this.view.dispatch(tr);
    }
    this.acceptPatch.emit(patch);
  }

  onRejectPatchClick(patch: PatchModel) {
    this.rejectPatch.emit(patch);
  }

  private createEditorState(doc: string): EditorState {
    const proseTheme = EditorView.theme(
      {
        '&': {
          backgroundColor: '#020617',
          color: '#e5e7eb',
        },
        '.cm-content': {
          maxWidth: '100%',
          margin: '0',
          lineHeight: '1.6',
          padding: '1.5rem 1.5rem 1rem',
        },
        '.cm-scroller': {
          overflow: 'auto',
        },
      },
      { dark: true }
    );

    const patchDecorations = this.buildPatchDecorations(doc);

    return EditorState.create({
      doc,
      extensions: [
        basicSetup,
        markdown(),
        EditorView.lineWrapping,
        proseTheme,
        EditorView.decorations.of(patchDecorations as DecorationSet),
        EditorView.updateListener.of((update) => {
          if (update.docChanged && !this.updatingFromInput) {
            const text = update.state.doc.toString();
            this.documentContentChange.emit(text);
          }
        }),
      ],
    });
  }

  private buildPatchDecorations(doc: string): DecorationSet {
    const ranges: Range<Decoration>[] = [];

    for (const patch of this.patches ?? []) {
      const from = patch.startOffset;
      const to = patch.endOffset;
      if (
        typeof from !== 'number' ||
        typeof to !== 'number' ||
        !Number.isFinite(from) ||
        !Number.isFinite(to) ||
        from < 0 ||
        to < from
      ) {
        continue;
      }

      const currentTextAtRange = doc.slice(from, to);
      if (currentTextAtRange !== patch.beforeText) {
        console.warn(
          `Patch ${patch.id} is stale or invalid: document text at range [${from},${to}] does not match beforeText. Skipping.`
        );
        continue;
      }

      ranges.push(
        Decoration.mark({
          class: 'cm-patch-before',
        }).range(from, to)
      );

      ranges.push(
        Decoration.widget({
          widget: new PatchSuggestionWidget(patch.id, patch.afterText),
          block: true,
        }).range(to)
      );
    }

    if (!ranges.length) {
      return Decoration.none;
    }

    return Decoration.set(ranges, true);
  }
}

class PatchSuggestionWidget extends WidgetType {
  constructor(
    private patchId: string,
    private afterText: string
  ) {
    super();
  }

  override eq(other: WidgetType): boolean {
    return (
      other instanceof PatchSuggestionWidget &&
      other.patchId === this.patchId &&
      other.afterText === this.afterText
    );
  }

  toDOM(): HTMLElement {
    const container = document.createElement('div');
    container.className = 'cm-patch-suggestion';

    const text = document.createElement('pre');
    text.className = 'cm-patch-suggestion-text';
    text.textContent = this.afterText;

    const actions = document.createElement('div');
    actions.className = 'cm-patch-suggestion-actions';

    const cancelBtn = document.createElement('button');
    cancelBtn.type = 'button';
    cancelBtn.className = 'cm-patch-btn cm-patch-btn-cancel';
    cancelBtn.textContent = 'Cancel';
    cancelBtn.onclick = (ev) => {
      ev.preventDefault();
      window.dispatchEvent(
        new CustomEvent('patchAction', {
          detail: { id: this.patchId, action: 'reject' },
        })
      );
    };

    const applyBtn = document.createElement('button');
    applyBtn.type = 'button';
    applyBtn.className = 'cm-patch-btn cm-patch-btn-apply';
    applyBtn.textContent = 'Apply suggestion';
    applyBtn.onclick = (ev) => {
      ev.preventDefault();
      window.dispatchEvent(
        new CustomEvent('patchAction', {
          detail: { id: this.patchId, action: 'accept' },
        })
      );
    };

    actions.appendChild(cancelBtn);
    actions.appendChild(applyBtn);

    container.appendChild(text);
    container.appendChild(actions);

    return container;
  }
}
