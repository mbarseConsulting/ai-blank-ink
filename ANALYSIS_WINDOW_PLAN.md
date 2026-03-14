## Ink Intelligence Desk – Second Window Plan

This document describes the second desktop window dedicated to **analysis, reports, and structure**, complementary to the main “Writing Surface” window.

---

## 1. Purpose & Relationship to Main Window

- **Main window** (already implemented):
  - Focused on writing and revision.
  - Central CodeMirror editor, inline patch suggestions (red/green blocks), file tree on the left, skill launcher on the right.
- **Second window – Ink Intelligence Desk** (this document):
  - Focused on **reading, understanding, and organizing feedback** from skills.
  - Hosts long-form reports (qa-reader, qa-originality, qa-prose, qa-characters, qa-consistency) and structural views (outline-ink, arch-ink).
  - Does **not** edit text directly; all text changes still go through the main window via patches.

The mental model is “manuscript on one desk, analytical dossier on another”.

---

## 2. Layout – The Dossier

High-level layout (desktop-only):

- **Far left – Skill Rail**
  - Slim vertical bar with grouped skill families:
    - Lecture: `qa-reader`, `qa-originality`.
    - Langue: `qa-prose`, `edit-ai-fr`, `edit-final`.
    - Structure: `outline-ink`, `arch-ink`.
  - Icon + short label; selecting a family filters the run history and available reports.

- **Left sidebar – Run History**
  - Vertical timeline of `SkillRun`s for the currently active document:
    - Example entries:
      - `10:15 – qa-reader – Rev 4`
      - `10:22 – qa-originality – Voice focus`
    - Clicking an entry sets the **active run** in the Dossier.
  - Optional filters:
    - By severity (info/suggestion/warning/error).
    - By date range.

- **Center – Dossier View (Report Reader)**
  - Wide reading surface, optimized for long text:
    - Line length ≈ 65–75 characters.
    - Comfortable serif font (e.g. “editorial” style).
  - Shows the main body of the selected report:
    - `qa-reader`: narrative analysis of hooks, pacing, tension.
    - `qa-prose`: craft findings.
    - `qa-originality`: voice/originality commentary.
    - `arch-ink` / `outline-ink`: structural descriptions, acts, scenes.
  - Header includes:
    - Document title.
    - Skill name.
    - Run timestamp and version indicator.

- **Right sidebar – Report Map**
  - Contextual “table of contents” for the active report:
    - Headings extracted from the report (e.g. “Scène 1 – Librairie”, “Scène 2 – Abandon des masques”).
    - Anchors to jump inside the Dossier.
  - For structural skills:
    - Acts / beats / chapters tree.
    - Quick navigation to specific structural elements.

---

## 3. Interaction Model

### 3.1. Cross-window Sync

Second window and main window communicate via Tauri events and shared `SkillRun` data:

- When un skill “réel” est exécuté dans la fenêtre principale (ex. `qa-reader` avec une demande utilisateur):
  - La fenêtre principale émet un événement (actuellement `skill-run`) avec:
    - `documentPath`
    - `skillName`
    - `SkillRunModel` (including diagnostics + patches, even if patches are not displayed here).
- The analysis window listens and:
  - Updates its Run History for this document.
  - Optionally auto-selects the latest run for the current skill.

### 3.2. “Flashback” – Run Versioning

Each `SkillRun` is a snapshot:

- UI pattern at top of the Dossier:
  - `LaGalerie-complet.md > qa-reader > [Version 3 (Latest) ▾]`
- Selecting an older version:
  - Reloads the corresponding report body.
  - Keeps main window text as-is (no rollback implied).

### 3.3. Click-through Findings

From the Dossier:

- Clicking a highlighted finding (e.g. a `qa-prose` comment or structural warning):
  - Sends a navigation event to the main window.
  - Main window centers the affected paragraph / range and briefly highlights it.

This uses the existing offset / range information where available.

---

## 4. Visual Language

- **Theme contrast**
  - Main window: dark, focused IDE-like environment.
  - Analysis window: slightly lighter “editorial dashboard” feel; still dark-friendly, but more “paper/print” in typography.

- **Typography**
  - Body text in reports uses a readable serif font (within technical constraints).
  - Headings and UI chrome keep the same sans-serif as the main app for consistency.

- **Color axes (aligned with skills)**
  - Structure (`arch-ink`, `outline-ink`): deep indigo / blue accents.
  - Craft (`qa-prose`, `edit-*`): green accents.
  - Voice / originality (`qa-originality`): amber / warm accents.
  - Reading experience (`qa-reader`): neutral / teal accents.

These colors are used for tags, badges, run history markers, and section headings.

---

## 5. Data & Components (Frontend)

### 5.1. Implémentation actuelle

- `AnalysisShellComponent` (window root, **déjà implémenté**) :
  - State réel :
    - `activeDocumentPath: string | null`
    - `activeSkillName: SkillName | null`
    - `activeRun: SkillRunModel | null`
    - `runsByKey: Record<string, SkillRunModel[]>` où la clé = `documentPath::skillName`
    - `activeIndex: number` (index du run sélectionné pour le skill actif)
    - `openSkills: SkillName[]` (liste des skills ayant un onglet ouvert pour le document courant)
  - Intégration Tauri :
    - Écoute `analysis-init` :
      - Reçoit `{ documentPath }` depuis la fenêtre principale.
      - Charge l’historique pour tous les skills supportés (`qa-reader`, `qa-originality`, `qa-prose`, `edit-ai-fr`) depuis `.analysis-history/`.
      - Initialise `openSkills` uniquement avec les skills ayant de l’historique.
      - Choisit un skill actif par défaut (ordre : `qa-reader` → `qa-originality` → `qa-prose` → `edit-ai-fr`) et sélectionne le run le plus récent.
    - Écoute `skill-run` :
      - Ajoute le `SkillRunModel` à `runsByKey` pour la paire `(documentPath, skillName)`.
      - Ajoute le skill aux `openSkills` si besoin.
      - Sauvegarde immédiatement la nouvelle liste de runs sur disque via `save_skill_run_history`.
  - Techniques Angular :
    - Utilise `NgZone` + `ChangeDetectorRef` pour garantir que les mises à jour venant de Tauri rafraîchissent bien l’UI.

- Tabs par skill (Chrome-like) :
  - UI implémentée dans le template d’`AnalysisShellComponent` :
    - `openSkills` → liste d’onglets dynamiques (`qa-reader`, `qa-originality`, `qa-prose`, `edit-ai-fr`).
    - Un onglet n’apparaît que s’il existe de l’historique pour ce skill **ou** après un premier run.
    - Un bouton de fermeture par onglet permet de retirer un skill de `openSkills` (changement automatique de l’onglet actif si nécessaire).
  - Style :
    - Inspiré des onglets de navigateur (coins arrondis, onglet actif surélevé, bouton de fermeture discret).

- Run history (colonne gauche) :
  - Liste les `currentRuns` = historique du skill actif pour `activeDocumentPath`.
  - Met en avant le run sélectionné (`activeIndex`).
  - `selectRun(index)` change simplement `activeRun` et `activeIndex`.

- Report reader (zone centrale) :
  - Affiche `activeRun.diagnostics[0].message` comme corps principal du rapport (rapport “éditorial”).
  - **Rendu Markdown** : gras `**texte**`, italique `*texte*`, listes à puces (`-` ou `*`), retours à la ligne conservés. Parsing maison (pas de lib externe), échappement XSS (`<`, `>`, `&`). Styles dédiés pour `strong`, `em`, listes, paragraphes.
  - Pour `edit-ai-fr`, ce message inclut également un bloc listant les suggestions détectées sous forme :
    - `ORIGINAL / REPLACEMENT / EXPLANATION` (données issues du JSON structuré renvoyé par Gemini, et réinjectées par le backend).

- Persistance :
  - `load_skill_run_history(documentPath, skillName)` et `save_skill_run_history(documentPath, skillName, runs[])` exposés par Tauri.
  - Fichiers stockés dans `writer-app/.analysis-history/` (ignorés par Git).
  - Au lancement de la fenêtre d’analyse, l’état en mémoire est reconstruit depuis ces fichiers.

### 5.2. Composants prévus / à affiner

High-level Angular component ideas (names still tentative, certains aspects déjà présents dans `AnalysisShellComponent`) :

- `SkillRailComponent`
  - Props: `families`, `activeFamily`.
  - Emits: `familyChange`.

- `RunHistoryComponent`
  - Props: `activeFamily`, `runs`, `activeRunId`.
  - Emits: `runSelected`.

- `DossierViewComponent`
  - Props: `run: SkillRunModel | null`.
  - Displays:
    - Header (doc, skill, timestamp, version selector).
    - Body (report text).
    - Optional: diagnostic list view.

- `ReportMapComponent`
  - Props: `run`, parsed headings / structure.
  - Emits section navigation events for scrolling inside the Dossier.

Initially, we can render reports as plain text/markdown using a generic renderer, then evolve toward more specialized visualizations.

---

## 6. Future Enhancements

- **Skill-specific visualizations**
  - `qa-reader`: tension graph / emotional arc sparkline per chapter.
  - `qa-originality`: originality score badges next to paragraphs.
  - `arch-ink`: act/beat map, milestones.

- **Filtering & comparison**
  - Compare two runs for the same skill on the same document.
  - Filter findings by axis / severity.

- **Session timeline**
  - Chronological view across skills:
    - `calibrate-ink → arch-ink → qa-reader → edit-ai-fr`.

These can be added incrementally once the base Dossier window and event pipeline are stable.

