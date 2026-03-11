# AI Blank Ink – Interface d’atelier texte

## 1. Objectif

Créer une interface dédiée à l’écriture et à la révision de textes de fiction, qui orchestre les skills existants (AI Write Ink, AI Forge Ink, AI Crafter Toolkit) et présente leurs retours sous forme de propositions diffables/mergeables.

---

## 2. Modèle de données

### 2.1. Document

- `id`
- `title`
- `content` (texte courant)
- `status` (`draft` | `in_review` | `finalized`)
- `history[]` – snapshots :
  - `timestamp`
  - `content`
  - `label` (ex. « avant edit-ai-fr batch #1 »)

### 2.2. SkillRun

- `id`
- `documentId`
- `skillName` (`qa-reader`, `qa-originality`, `edit-ai-fr`, etc.)
- `mode` (`analysis`, `rewrite`, `inline-fix`, …)
- `scope` (`full` | `selection`)
- `inputSnapshot` (contenu de départ)
- `result` :
  - `diagnostics[]`
  - `patches[]`
- `createdAt`

### 2.3. Patch / Proposal

- `id`
- `skillRunId`
- `range` – soit offsets, soit lignes :
  - `startOffset`, `endOffset` **ou**
  - `startLine`, `endLine`
- `beforeText`
- `afterText`
- `status` (`pending` | `accepted` | `rejected`)
- `axis` (ex. `ai-fr:forbidden-locutions`, `qa-reader:scene-tension`, `qa-originality:formulation-freshness`)
- `note` (commentaire du skill)

---

## 3. UX / Layout

### 3.1. Écran principal

- **Colonne centrale** : éditeur de texte
  - Texte du document
  - Surlignage des zones avec propositions (`pending patches`)
  - Numéros de ligne / minimap

- **Barre latérale gauche** : navigation projet
  - Liste des textes (`Document`)
  - État global (icônes) : brouillon, en relecture, final

- **Barre latérale droite** : panneau de skills
  - Boutons par skill :
    - `Calibrer (calibrate-ink)`
    - `Structure (arch-ink)`
    - `Prose (write-ink)` – génération de texte
    - `QA lecture (qa-reader)`
    - `Originalité (qa-originality)`
    - `Langue FR (edit-ai-fr)`
  - Pour chaque skill :
    - Bouton `Analyser`
    - Dernier statut (OK / findings / error)
    - Compteur de propositions (ex. `5 suggestions`, `2 critiques`)

- **Bandeau bas** : zone de prompt libre
  - Champ de texte pour dialoguer avec l’agent
  - Sélecteur de scope :
    - `Texte entier`
    - `Sélection`
    - `Diagnostic en cours`

### 3.2. Vue “merge request”

Pour un `SkillRun` donné :

- **Liste des propositions**
  - Chaque item = un `Patch` :
    - Type / axis (`langue`, `formulation`, `tension`, …)
    - Extrait du texte concerné
    - Boutons : `Voir diff`, `Accepter`, `Rejeter`

- **Diff détaillé**
  - Vue comparaison `before / after` à la MR (rouge/vert)
  - Actions :
    - `Accepter cette proposition`
    - `Rejeter cette proposition`
    - `Accepter toutes les propositions de cet axis` (optionnel)

---

## 4. Refined UX / Component Architecture

Dans cette vision, **le texte est le code source**, les **skills sont des compilateurs / linters**, et les **patches sont des pull requests**. L’UI doit refléter cette métaphore.

### 4.1. Tri-pane layout

- **Pane centre – Primary Forge**
  - Éditeur Markdown/texte plein écran (Monaco ou ProseMirror).
  - Gutter markers pour :
    - erreurs structurelles (`arch-ink`),
    - problèmes de langue / “AI-smell” (`edit-ai-fr`),
    - opportunités créatives (`spark`, etc.).
  - Clic sur un marker → focus sur le diagnostic correspondant dans le panneau droit.

- **Pane gauche – Skill Registry**
  - Tree view groupée par :
    - Write Ink,
    - Forge Ink,
    - Crafter.
  - Section “Favorites / Recents”.
  - Palette de commande (`Ctrl+K`) pour lancer un skill rapidement.

- **Pane droit – Intelligence Layer**
  - Panneau tabulé :
    - `Diagnostics` — liste des findings groupés par axis/skill.
    - `Patches` — vue diff/PR pour les propositions de corrections.
    - `Artifacts` — worldbibles, fiches persos, résultats de scan, etc.
  - Barre de statut en bas :
    - skill(s) en cours,
    - progression `steps-ink`,
    - badge de calibration (genre/ton/failure mode).

### 4.2. Composants Angular & état

#### État global (signals)

```ts
export interface AppState {
  currentFile: Signal<string | null>;             // chemin ou handle
  document: Signal<DocumentModel | null>;         // contenu + métadonnées
  activeSkill: Signal<SkillDefinition | null>;
  skillRuns: Signal<SkillRunModel[]>;
  pendingPatches: Signal<PatchModel[]>;
  isProcessing: Signal<boolean>;
}
```

#### Découpage par features

- `features/editor` :
  - `EditorShellComponent` — surface d’écriture.
  - `GutterMarkersService` — intégration QA <-> éditeur.
  - `CalibrationOverlayComponent` — badge `[GENRE: ...]`.
- `features/skill-panel` :
  - `SkillPanelComponent` — tree, favoris, palette.
- `features/diagnostics` :
  - `DiagnosticsListComponent` — findings groupés par axis, skill, sévérité.
- `features/patches` :
  - `PatchListComponent`, `PatchDetailComponent` — vue diff + accept/reject.
- `features/forge` :
  - vues pour worldbuilding et scan (world bibles, timelines, etc.).

#### Patch = Pull Request

- Modèle enrichi :

```ts
export interface PatchModel {
  id: string;
  skillRunId: string;
  sourceSkillId: string;
  axis: string;
  severity: 'info' | 'suggestion' | 'warning' | 'error';
  startOffset: number;
  endOffset: number;
  beforeText: string;
  afterText: string;
  status: 'pending' | 'accepted' | 'rejected';
  note?: string;
}
```

- UX :
  - Groupement par skill run, axis, sévérité.
  - Boutons `[Accept]`, `[Reject]`, `[Iterate]` (ré-exécuter le skill sur le contexte du patch).

### 4.3. Moments UX clés

- **Gutter Insights**
  - Markers colorés :
    - rouge : structure critique,
    - ambre : langue / artefact IA,
    - violet : opportunité créative.
  - Clic = navigation bidirectionnelle éditeur ↔ diagnostic.

- **Calibration Overlay**
  - Barre non modale en haut de l’éditeur :
    - `[GENRE: CYBERPUNK | TONE: GRIM | FAIL: NO EXPOSITION DUMPS]`.
  - Calquée sur la sortie de `calibrate-ink`.

- **Multi-Block Stepper (`steps-ink`)**
  - Visualisation de `Block 1/5`, `2/5`, etc.
  - Chaque block a ses propres diagnostics et patches, filtrables dans le panneau droit.

---

## 5. Architecture app desktop (Tauri + frontend web)

### 5.1. Choix techniques

- **Shell desktop** : [Tauri](https://tauri.app/) (binaire léger, bon pour un outil perso).
- **Frontend web** : Tauri accepte n’importe quel framework (React, Vue, Svelte, Angular, vanilla).  
  → Pour ce projet on part sur **Angular** (que tu préfères).

Organisation de haut niveau :

- `src-tauri/` :
  - Code Rust (commandes Tauri) pour :
    - Accès fichiers (`stories/`, `deps/ai-write-ink/...`)
    - Lecture des `SKILL.md` et `agent-*.md`
    - Appel à l’API du modèle (Claude/OpenAI…) en HTTP
    - Orchestration `SkillRun` → `diagnostics` + `patches`
- `frontend/` (Angular) :
  - Composants UI (éditeur, panneau de skills, vue diff)
  - Gestion de l’état (`Document`, `SkillRun`, `Patch`)
  - Appels aux commandes Tauri (`invoke`) pour lancer les skills, lire/écrire les fichiers, etc.

### 5.2. Gestion de la clé API (sans base de données)

- **Source** : pas de stockage serveur, uniquement **local**.
- Options possibles (à préciser à l’implémentation) :
  - Variable d’environnement `AI_BLANK_INK_API_KEY` lue par Tauri/Rust.
  - Ou fichier de config local (ex. `~/.ai-blank-ink/config.json`) lu/écrit via une commande Tauri.
- Angular ne voit jamais la clé directement si tu préfères la garder dans la partie Rust :
  - Le frontend demande juste `runSkill(skillName, filePath, range)` à Tauri.
  - Tauri s’occupe de :
    - Charger le texte
    - Charger les skills
    - Lire la clé API locale
    - Appeler l’API du modèle
    - Renvoyer un `SkillRun` structuré vers Angular.

---

## 6. Pipeline backend (agnostique à l’IDE / appli)

### 6.1. Architecture générale

- **Backend indépendant de l’IDE** (ex. : API HTTP en Node/Express ou Python/FastAPI).
- Le backend :
  - Lit les fichiers de skills depuis le repo (`deps/ai-write-ink/skills/...`).
  - Construit les prompts en injectant `SKILL.md` + `agent-*.md` + le texte.
  - Appelle une API de modèle (Claude, OpenAI, autre) **directement**, pas l’API de Cursor.
  - Retourne au front un objet structuré : `SkillRun` avec `diagnostics[]` et `patches[]`.

Ainsi, l’atelier peut tourner dans n’importe quelle interface :

- Web app standalone
- Plugin VS Code / autre IDE
- Client CLI

Cursor devient **une option de front-end**, pas un prérequis.

### 6.2. Workflow d’un appel de skill

1. Le front envoie à l’API :
   - `documentId`
   - `content` (ou un hash + ref côté serveur)
   - `skillName`
   - `scope` (full / selection + coordonnées)
2. Le backend :
   - Charge `SKILL.md` et `agent-*.md` depuis les repos de skills.
   - Construit un prompt (système + instructions du skill + texte à analyser).
   - Appelle le modèle via son API (Claude/Anthropic ou autre).
   - Parse la réponse dans un format structuré (`diagnostics`, `patches`).
3. L’API répond au front :
   - `SkillRun` + `patches[]` en status `pending`.
4. Le front affiche les résultats (liste + diff), sans modifier le texte tant que l’utilisateur n’accepte pas.

### 6.3. Spécificités par skill

- `calibrate-ink` : retourne un bloc de calibration (genre, conventions, failure mode). Pas de patchs, juste du contexte.
- `arch-ink` : retourne des diagnostics structuraux (acts, midpoint, promise/delivery). Pas de modifications directes du texte.
- `qa-reader` & `qa-originality` : surtout des `diagnostics[]` avec `axis`, `location`, `comment`. Possibilité ultérieure de générer des patchs ponctuels.
- `edit-ai-fr` : principal générateur de `patches[]` (corrections langue, répétitions, artefacts IA).

---

## 7. Application des patchs et versionning

### 7.1. Application

- Lorsqu’un patch est **accepté** :
  - Le backend applique le diff sur `Document.content`.
  - Met à jour `Patch.status = accepted`.
  - Retourne le texte mis à jour au front.

- Lorsqu’un patch est **rejeté** :
  - `Patch.status = rejected`.
  - Il n’est plus proposé.

### 7.2. Snapshots

- Avant l’application d’un lot de patchs (par ex. « Appliquer 20 corrections edit-ai-fr ») :
  - Créer un snapshot dans `history[]`.
- Offrir dans l’UI :
  - Vue diff entre snapshot et version actuelle.
  - Rollback possible (restaurer un snapshot).

---

## 8. Points UX supplémentaires

- **Filtres par axis** : afficher/accepter seulement certaines catégories de propositions (`ai-fr`, `qa-reader`, etc.).
- **Mode lecture seule critique** : désactiver l’édition libre pour se concentrer sur la lecture + commentaires.
- **Timeline des skill-runs** : petite chronologie montrant l’ordre des appels de skills (calibrate → arch → qa-reader → edit-ai-fr, etc.).

---

## 9. État actuel de l’implémentation (Angular + Tauri)

### 9.1. Shell desktop & fichiers

- **Tauri 2** initialisé dans `writer-app/src-tauri/` avec :
  - `open_document(path: String) -> String` – lecture d’un `.md` depuis le repo (ex. `stories/LaGalerie/scene-1.md`).
  - `save_document(path: String, content: String) -> ()` – écriture directe sur le disque.
  - `list_stories_tree() -> DocumentNode[]` – scan récursif de `stories/` et retour d’un arbre `[folder|document]` (chemins relatifs).

- **Frontend Angular** dans `writer-app/frontend/` :
  - Arbre à gauche alimenté par `list_stories_tree` (dossiers + fichiers `.md`).
  - Clic sur un fichier → `open_document` → charge le texte dans `currentDocument`.
  - Bouton `Enregistrer` + `Ctrl+S` → `save_document` sur le fichier courant, avec indicateur visuel de sauvegarde.

### 9.2. Éditeur central (CodeMirror 6)

- Remplacement du `<textarea>` par **CodeMirror 6** configuré pour la prose :
  - `basicSetup` + `markdown()` + `lineWrapping`.
  - Thème sombre « page de roman » (fonds, interligne, padding).
  - Barre d’outils locale :
    - Choix de **police** (Georgia, Times, Garamond, Inter, etc.).
    - Choix de **taille** (Small / Medium / Large).

- La zone centrale occupe tout l’espace disponible, avec scroll vertical propre, prompt fixé en bas.

### 9.3. Patches inline (mock)

- **Modèle `PatchModel`** déjà aligné avec le plan (offsets + `beforeText` / `afterText` + `axis` + `note`).
- Intégration native dans CodeMirror :
  - Chaque patch est rendu comme :
    - Un `Decoration.mark` sur la plage ciblée (`cm-patch-before`) → fond rouge semi‑transparent (texte actuel).
    - Un `Decoration.widget` juste après la plage → bloc vert (`cm-patch-suggestion`) avec :
      - texte suggéré,
      - boutons `Apply suggestion` / `Cancel`.
  - Styles appliqués via `styles.css` (`.cm-patch-before`, `.cm-patch-suggestion`, etc.).

- **Application des suggestions** :
  - Le bouton **Apply** déclenche une transaction CodeMirror :
    - `changes: { from: startOffset, to: endOffset, insert: afterText }`.
  - L’éditeur émet ensuite `documentContentChange` vers Angular → `currentDocument.content` est synchronisé.
  - Côté `App`, `acceptPatch` se contente de retirer le patch de `pendingPatches`.

### 9.4. Mocks de skills

- **QA lecture (`qa-reader`)** :
  - `runSelectedSkill` appelle `createMockPatch('qa-reader')` côté éditeur.
  - Détection exacte du bloc :
    - `Elle tendit la main vers …` jusqu’à `— Permettez.` via regex sur `EditorState.doc`.
    - Offsets validés via `doc.sliceString(from, to)` pour éviter les décalages.
  - `afterText` = bloc identique + ` [TEST]` à la fin pour visualiser l’application.

- **Originalité (`qa-originality`)** :
  - `runSelectedSkill` détecte ce skill et appelle `createMockOriginalityPatches()` :
    - Crée plusieurs `PatchModel` sur des phrases arbitraires (ex. : `Les rayonnages montaient haut…`, `Elle regarda autour d'elle…`).
    - Chaque suggestion ajoute aussi ` [TEST]` en fin de bloc.
  - Résultat : plusieurs suggestions simultanées, chacune avec son bloc rouge + bloc vert + boutons.

- Changement de fichier dans le panneau gauche :
  - `pendingPatches` est vidé → pas de suggestions résiduelles quand on change de texte.

---

## 10. Prochaines étapes d’implémentation

1. **Passer du mock aux vrais skills**
   - Créer des commandes Tauri :
     - `run_skill(skillName: String, path: String, range: Option<Range>) -> SkillRunModel`.
   - Lire les `SKILL.md` / `agent-*.md` depuis `deps/ai-write-ink`, `deps/ai-forge-ink`, etc.
   - Construire les prompts, appeler l’API modèle et renvoyer de vrais `diagnostics[]` + `patches[]`.
   - Côté Angular :
     - Remplacer `createMockPatch` / `createMockOriginalityPatches` par les données renvoyées par Tauri.

2. **Gestion avancée des patches**
   - Grouper les suggestions par `SkillRun` + `axis` dans le panneau droit.
   - Ajouter des actions batch :
     - `Accepter toutes les propositions de cet axis`.
     - `Rejeter toutes les propositions de ce SkillRun`.
   - Journaliser les états (`history[]`) à chaque batch d’acceptation.

3. **Diagnostics & gutter**
   - Introduire un panneau `Diagnostics` branché sur `DiagnosticModel` (déjà défini dans `models.ts`).
   - Mapper les diagnostics sur :
     - surlignages légers (pas de widget),
     - markers dans la gouttière (Gutter Insights).
   - Clic sur un diagnostic → scroll + focus dans l’éditeur.

4. **Persistance & multi-documents**
   - Ajouter un mini-cache de `SkillRun` par document (mémoire ou fichier `.json` à côté du `.md`).
   - Gérer les cas où le fichier a changé depuis le dernier run (invalidation des patches obsolètes).

5. **Polish UX**
   - Thème clair optionnel.
   - Préférences (police/taille par défaut, largeur de colonne).
   - Animation subtile sur apparition / disparition de suggestions.

Ces étapes gardent l’architecture actuelle (Tauri + Angular + CodeMirror + patches inline), mais remplacent progressivement le mock par un flux complet skill → diagnostics → patches → accept/reject.

