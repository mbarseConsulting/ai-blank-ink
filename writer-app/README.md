# AI Blank Ink – Writer App (Tauri + Angular)

Cette app desktop est l’atelier d’écriture qui orchestre les skills
(`qa-reader`, `qa-originality`, `qa-prose`, `write-ink`, `cowrite-ink`, `edit-ai-fr`, …)
directement sur les fichiers du projet (`stories/`, `deps/...`).

## Configuration

- **Valeurs en dur :** toutes dans `writer-app/config.json` (chemins, fenêtres, skills, UI, i18n). Voir **[CONFIG.md](CONFIG.md)** (réf. `writer-app-config`) — ne pas réintroduire de chaînes ou constantes en dur dans le code.
- **Clé API :** `writer-app/.env` (non committé) avec `GEMINI_API_KEY=...` ; chargée via `dotenvy` côté backend.

## Fonctionnalités récentes

- **Footer de conversation** : les réponses des skills s’affichent dans un footer redimensionnable sous l’éditeur, avec champ de suivi. Entrée = envoi, Shift+Entrée = nouvelle ligne.
- **Rendu Markdown** : réponses des skills (footer + bureau d’analyse) affichées en **gras**, *italique*, listes à puces, paragraphes. Parsing maison, échappement XSS.
- **write-ink** et **cowrite-ink** branchés sur Gemini (génération, réécriture, brainstorm/critique).
- **Bureau d’analyse** : rapports en Markdown, historique persistant par document+skill.

Ce dossier contient :

- `frontend/` – app Angular (UI)
- `src-tauri/` – app Tauri (shell desktop + commandes Rust)

## Structure cible

```text
writer-app/
  frontend/
    src/
      app/
        app.component.ts
        app.component.html
        app.component.css
        models.ts
  src-tauri/
    src/
      main.rs
    tauri.conf.json
```

## Rôle des parties

- **Angular (frontend)** : layout, éditeur de texte, panneau de skills, vue diff
- **Tauri (Rust)** : accès fichiers locaux, lecture des `SKILL.md`, appel au
  modèle (API Claude/OpenAI), consolidation des résultats en objets structurés
  (`SkillRun`, `Patch`, etc.)

