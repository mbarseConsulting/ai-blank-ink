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
import { EditorState } from '@codemirror/state';
import { EditorView } from '@codemirror/view';
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
  @Input() promptText = '';
  @Output() promptTextChange = new EventEmitter<string>();
  @Output() sendPromptRequested = new EventEmitter<void>();
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

  ngAfterViewInit(): void {
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

    const state = EditorState.create({
      doc: this.document?.content ?? '',
      extensions: [
        basicSetup,
        markdown(),
        EditorView.lineWrapping,
        proseTheme,
        EditorView.updateListener.of((update) => {
          if (update.docChanged && !this.updatingFromInput) {
            const text = update.state.doc.toString();
            this.documentContentChange.emit(text);
          }
        }),
      ],
    });

    this.view = new EditorView({
      state,
      parent: this.editorHost.nativeElement,
    });
  }

  ngOnChanges(changes: SimpleChanges): void {
    if (!this.view) return;
    if (changes['document'] && this.document) {
      const newText = this.document.content ?? '';
      const currentText = this.view.state.doc.toString();
      if (newText !== currentText) {
        this.updatingFromInput = true;
        this.view.dispatch({
          changes: { from: 0, to: currentText.length, insert: newText },
        });
        this.updatingFromInput = false;
      }
    }
  }

  ngOnDestroy(): void {
    this.view?.destroy();
  }

  onPromptChange(value: any): void {
    this.promptTextChange.emit(String(value));
  }

  onSendPrompt(): void {
    this.sendPromptRequested.emit();
  }

  onAcceptPatchClick(patch: PatchModel) {
    this.acceptPatch.emit(patch);
  }

  onRejectPatchClick(patch: PatchModel) {
    this.rejectPatch.emit(patch);
  }
}

