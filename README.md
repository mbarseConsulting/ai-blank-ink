# 📇 AI Blank Ink

_Creative writing studio — aggregates skills from three specialized toolkits into a single workspace._

## Description

AI Blank Ink is a unified creative studio that brings together fiction writing, visual creation, and utility skills. It composes skills from three source projects:

- **AI Write Ink** — Fiction writing pipeline (structure, prose, QA, editing)
- **AI Forge Ink** — Visual fiction (worldbuilding, scan analysis, image prompts)
- **AI Crafter Toolkit** — Utility skills (artifact crafting, critical thinking, fact-checking)

## Overview

### Writing & Fiction (AI Write Ink)

| Skill            | Role                                                                                                   |
| ---------------- | ------------------------------------------------------------------------------------------------------ |
| `calibrate-ink`  | Genre calibrator — sets genre conventions for the session: tone, style expectations, failure modes     |
| `arch-ink`       | Story architect — structural editor: acts, arcs, throughlines, promise/delivery, want/need             |
| `outline-ink`    | Story outliner — builds narrative structure macro to micro: saga → arc → chapter → scene script        |
| `cowrite-ink`    | Literary & creative director — brainstorms ideas, steers direction, critiques on demand                |
| `dialog-ink`     | Stage director — blocks dialogue scenes with physical movement, staging, semi-theatrical script output |
| `puppet-ink`     | Fiction performer — roleplay (collaborative fiction with NPCs/world) or puppet (character embodiment)  |
| `write-ink`      | Fiction writer — narrative prose: scenes, chapters, continuations, surgical rewrites                   |
| `qa-prose`       | Line editor — hunts craft weaknesses at sentence level: POV, show-tell, dialogue, description          |
| `qa-reader`      | Reading experience critic — diagnoses hooks, pacing, tension, engagement                               |
| `qa-characters`  | Character psychologist — evaluates psychology, relational dynamics, credibility                        |
| `qa-consistency` | Continuity editor — verifies factual coherence: objects, timeline, lore, arcs, OOC behavior            |
| `qa-originality` | Editorial scout — evaluates creative singularity and market positioning                                |
| `edit-ai-fr`     | French prose cleaner — removes AI patterns, fixes language errors, repairs mechanical repetition       |
| `edit-final`     | Copyeditor — last pass: typos, typography, mechanical consistency                                      |

### Visual Fiction (AI Forge Ink)

| Skill        | Role                                                                                         |
| ------------ | -------------------------------------------------------------------------------------------- |
| `world-ink`  | World bible builder — generates and maintains universe bibles, timelines, character sheets   |
| `scan-ink`   | Scan analyzer — extracts script, dialogue, scene descriptions from comic/manga/BD pages      |
| `prompt-ink` | Prompt generator — translates narrative content into image prompts (NovelAI, Midjourney, SD) |

### Utilities (AI Crafter Toolkit)

| Skill       | Role                                                                                            |
| ----------- | ----------------------------------------------------------------------------------------------- |
| `ai-expert` | AI artifact crafter — creates, audits, and advises on skills, agents, and hooks                 |
| `critic`    | Critical challenger — leads with objection, targets structural problems, never validates softly |
| `idk`       | Fact checker — searches before answering, cites sources, says "I don't know" when unsure        |
| `parse`     | Intent translator — analyzes human language, reformulates into AI-optimized instructions        |
| `spark`     | Creative provocateur — drops unexpected "what if" angles to break predictable thinking          |
| `steps`     | Precision modifier — chunks long content into ~500-word blocks for granular analysis            |

**23 skills** — 14 writing, 3 visual, 6 utilities

## Usage

### Writing & Fiction

#### `/calibrate-ink`

Set genre conventions for the session. Call at session start or after context loss.

---

#### `/cowrite-ink`

Discuss fiction — critique, direction, alternatives, beats. Brainstorm or unblock.

- **`-w` / `--write`** — handoff to prose: outputs context for ink, then stops
- **default** — creative interlocutor, adapts to what you bring

---

#### `/outline-ink`

Build story structure from macro to micro.

- **`--universe-sagas`** / **`--saga`** / **`--arc`** / **`--chapter`** / **`--script`** — loads matching template, builds section by section
- **`-i` / `--inline`** — write output inline instead of file
- **default** — structural interrogation: what level, what exists, what needs building

---

#### `/arch-ink`

Challenge story structure before writing. Diagnose acts, arcs, throughlines.

- **`-r` / `--report`** — full structural diagnostic with rules loaded
- **`-i` / `--inline`** — write output inline instead of file
- **default** — interrogation: one question at a time on the weakest structural element

---

#### `/write-ink`

Write narrative prose — scenes, chapters, continuations, surgical rewrites.

- **`--check`** — strict context verification before writing
- **`-i` / `--inline`** — write prose inline instead of file
- **default** — writes in the fiction's register, output to file

---

#### `/dialog-ink`

Stage dialogue scenes with physical movement and spoken language.

- **`-p` / `--pass`** — annotated diagnostic of existing prose
- **`--check`** — context verification, combines with any mode
- **`-i` / `--inline`** — write output inline instead of file
- **default** — scene mode, semi-theatrical script output to file

---

#### `/puppet-ink`

Collaborative fiction with characters.

- **`-p` / `--puppet`** — character embodiment in first person ("parle-moi en tant que...")
- **`-i` / `--inline`** — write output inline instead of file
- **default** — roleplay, NPC performance, world simulation

---

#### `/qa-reader`

Evaluate reading experience — hooks, pacing, tension, engagement.

- **`-r` / `--report`** — full report with rules loaded
- **`-b` / `--bookends`** — focused on opening/closing analysis
- **`-i` / `--inline`** — write output inline instead of file
- **default** — emoji-block diagnostic (gut → editor → critic)

---

#### `/qa-prose`

Evaluate sentence-level craft — POV, show-tell, dialogue, description.

- **`-r` / `--report`** — full report with rules loaded
- **`-i` / `--inline`** — write output inline instead of file
- **default** — emoji-block diagnostic

---

#### `/qa-characters`

Evaluate character psychology, arcs, dynamics, credibility.

- **`-r` / `--report`** — full report with rules loaded
- **`-i` / `--inline`** — write output inline instead of file
- **default** — emoji-block diagnostic per character

---

#### `/qa-consistency`

Verify continuity — objects, timeline, lore, arcs, OOC behavior.

- **`-r` / `--report`** — full report with rules loaded
- **`-i` / `--inline`** — write output inline instead of file
- **default** — dual-cite findings (establishment + violation)

---

#### `/qa-originality`

Evaluate creative singularity — voice, concept, freshness, clichés.

- **`-r` / `--report`** — full report with rules loaded
- **`-i` / `--inline`** — write output inline instead of file
- **default** — emoji-block diagnostic

---

#### `/edit-ai-fr`

Clean French fiction prose of AI patterns, language errors, mechanical repetition.

- **`--interactive`** — one correction at a time, wait for confirmation
- **`-i` / `--inline`** — write output inline instead of file
- **default** — batch mode: list all findings, then apply on command

---

#### `/edit-final`

Last pass before publication — typos, grammar, typography, mechanical consistency.

- **`--interactive`** — one correction at a time, wait for confirmation
- **`-i` / `--inline`** — write output inline instead of file
- **default** — batch mode: list all findings, then apply on command

---

### Visual Fiction

#### `/world-ink`

Build or extend a world bible — universe, timeline, characters, relations.

- **`--universe` / `--timeline` / `--character` / `--relation`** — guided construction by template
- **`-i` / `--inline`** — write output inline instead of file
- **default** — interrogation: identify gaps, propose structure

---

#### `/scan-ink`

Analyze comic/manga/BD scans — extract script, scenes, tags, characters.

- **`--script`** — dialogue + stage directions only
- **`--scene`** — scene description only
- **`--tags`** — categorized tags only
- **`--characters`** — character identification and description
- **default** — full analysis: dialogue + scene + summary

---

#### `/prompt-ink`

Generate image prompts from narrative content.

- **`--novelai`** — NovelAI syntax (default)
- **`--midjourney`** — Midjourney syntax
- **`--sd`** — Stable Diffusion syntax
