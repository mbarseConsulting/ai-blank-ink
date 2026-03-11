# AI Blank Ink – Writer App (Tauri + Angular)

Cette app desktop sera l’atelier d’écriture qui orchestre les skills
(`calibrate-ink`, `arch-ink`, `qa-reader`, `qa-originality`, `edit-ai-fr`, …)
directement sur les fichiers du projet (`stories/`, `deps/...`).

Ce dossier contient :

- `frontend/` – app Angular (UI)
- `src-tauri/` – app Tauri (shell desktop + commandes Rust)

> Note : la structure ci-dessous est posée comme squelette. Les commandes
> Angular/Tauri réelles (ng new, tauri init) pourront être branchées ensuite.

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

