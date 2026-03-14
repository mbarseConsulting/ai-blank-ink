export type SkillName =
  | 'calibrate-ink'
  | 'arch-ink'
  | 'write-ink'
  | 'cowrite-ink'
  | 'qa-reader'
  | 'qa-originality'
  | 'qa-prose'
  | 'qa-characters'
  | 'qa-consistency'
  | 'edit-ai-fr';

export type PatchStatus = 'pending' | 'accepted' | 'rejected';

export type DocumentNodeKind = 'folder' | 'document';

export interface DocumentNode {
  id: string;
  label: string;
  kind: DocumentNodeKind;
  /** Absolute or project-relative path; future Tauri commands can use this. */
  path: string;
  children?: DocumentNode[];
}

export interface DocumentSnapshot {
  timestamp: string;
  label: string;
  content: string;
}

export interface DocumentModel {
  id: string;
  title: string;
  content: string;
  status: 'draft' | 'in_review' | 'finalized';
   /** Optional absolute path on disk for persistence. */
  path?: string;
  history: DocumentSnapshot[];
}

export interface PatchModel {
  id: string;
  skillRunId: string;
  startOffset: number;
  endOffset: number;
  beforeText: string;
  afterText: string;
  status: PatchStatus;
  axis: string;
  note?: string;
}

export interface DiagnosticModel {
  id: string;
  axis: string;
  message: string;
  severity: 'info' | 'suggestion' | 'warning' | 'error';
  startOffset?: number;
  endOffset?: number;
}

export interface SkillRunModel {
  id: string;
  documentId: string;
  skillName: SkillName;
  mode: 'analysis' | 'rewrite' | 'inline-fix';
  scope: 'full' | 'selection';
  createdAt: string;
  diagnostics: DiagnosticModel[];
  patches: PatchModel[];
}

