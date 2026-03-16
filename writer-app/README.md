### AI Blank Ink – Writer App (Tauri + Angular)

Writer App is the desktop workbench for AI Blank Ink.  
It runs skills (`qa-reader`, `qa-originality`, `qa-prose`, `qa-characters`, `qa-consistency`, `write-ink`, `cowrite-ink`, `edit-ai-fr`, …) directly on your local project files (`stories/`, `deps/...`).

---

## 1. Prerequisites

- **Git**
- **Node.js** LTS (18+) + `npm` (or `pnpm` / `yarn`)
- **Rust & Cargo** (via `https://rustup.rs`, min version in `writer-app/src-tauri/Cargo.toml`)
- **Tauri toolchain** for your OS  
  See `https://tauri.app/v2/guides/getting-started/prerequisites`.

Recommended: VS Code / Cursor or another modern editor.

---

## 2. Clone the repository

```bash
git clone <REPO_URL> ai-blank-ink
cd ai-blank-ink
```

Key structure:

```text
ai-blank-ink/
  stories/                  # stories / scenes
  deps/                     # skill bundles (SKILL.md, agents…)
  writer-app/
    frontend/               # Angular UI
    src-tauri/              # Tauri + Rust backend
    config.json             # runtime config
    CONFIG.md               # config documentation
```

---

## 3. Configure `writer-app/config.json`

All runtime “constants” (paths, skills, UI, i18n) live in `writer-app/config.json`.

- **Do not** hard-code paths / labels in the code.
- See `writer-app/CONFIG.md` for full details.

Typical defaults:

```jsonc
{
  "paths": {
    "storiesDir": "stories",
    "analysisHistoryDir": "writer-app/.analysis-history",
    "envFile": "writer-app/.env",
    "skillsBaseDir": "deps/ai-write-ink/skills"
  }
}
```

On a new machine you usually only need to confirm `storiesDir` and `skillsBaseDir`.

---

## 4. Add the Gemini API key

Create `writer-app/.env`:

```bash
cd writer-app
type NUL > .env   # Windows PowerShell / CMD
```

Put in:

```env
GEMINI_API_KEY=your_google_generative_language_api_key_here
# Optional:
# GEMINI_MODEL=gemini-2.5-flash-lite
```

This file is **not committed**.

---

## 5. Install dependencies

### Frontend (Angular)

```bash
cd writer-app/frontend
npm install
```

### Rust / Tauri

```bash
cd ../src-tauri
cargo check   # optional sanity check
```

Rust dependencies are handled via `Cargo.toml` / `Cargo.lock`.

---

## 6. Run the app (dev)

From `writer-app/src-tauri`:

```bash
cd writer-app/src-tauri
cargo run --no-default-features
```

This:

- loads `writer-app/config.json`,
- starts the desktop app,
- connects to the Angular frontend via a WebView.

For UI debugging, you may also run:

```bash
cd writer-app/frontend
npm run start   # e.g. http://localhost:4200
```

— but this is optional for normal use.

---

## 7. Using the app

When Tauri is running:

- Left: **stories tree** (from `storiesDir`),
- Center: **editor** (CodeMirror),
- Right / footer: **skills panel** + **conversation footer**.

Typical usage:

1. Open a story from the tree.
2. Select a skill (e.g. `qa-reader`, `qa-prose`, `edit-ai-fr`).
3. Run the skill:
   - `qa-*` skills produce analytical reports (Markdown).
   - `write-ink` / `cowrite-ink` generate or rewrite prose.
   - `edit-ai-fr` returns **inline suggestions** (patches) you can apply or reject in the editor.

The **Analysis Desk** window shows historical reports per document+skill.

---

## 8. Main components

- **Frontend (`writer-app/frontend`)**
  - `app.component.*`: app shell (editor + skills + footer).
  - `document-editor.component.*`: CodeMirror editor + inline patches (`edit-ai-fr`).
  - `analysis-shell.*`: Analysis Desk (Markdown reports + history).
  - `app-config.service.ts`: reads `writer-app/config.json` on the frontend.

- **Backend (`writer-app/src-tauri`)**
  - `src/lib.rs`: Tauri commands (`run_skill`, `open_document`, history, etc.), loads skill bundles, calls Gemini, builds `SkillRunDto` / `PatchDto`.
  - Special handling for `edit-ai-fr`: parses a JSON `suggestions` block and builds inline patches with robust matching (newline normalization + flexible whitespace).
  - `Cargo.toml` / `Cargo.lock`: Rust dependencies.
  - `tauri.conf.json`: Tauri app configuration.

---

## 9. Quick setup checklist

1. `git clone <REPO_URL> ai-blank-ink && cd ai-blank-ink`
2. Install Node, Rust, and the Tauri toolchain.
3. Adjust `writer-app/config.json` if needed.
4. Create `writer-app/.env` with `GEMINI_API_KEY=...`.
5. `cd writer-app/frontend && npm install`
6. `cd ../src-tauri && cargo check` (optional)
7. `cargo run --no-default-features` (from `writer-app/src-tauri`)
