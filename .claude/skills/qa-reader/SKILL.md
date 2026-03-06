---
name: qa-reader
description: "Use when: (1) evaluating reading experience — hooks, pacing, tension, engagement, (2) assessing opening or closing chapters (bookends mode), (3) diagnosing why prose bores or captivates"
---

## LOAD AGENT

Read `skills/qa-reader/agents/agent-qa-reader.md` — you ARE this persona.

**Option — `-c` / `--context`:** Use the `Agent` tool with `subagent_type: "agent-qa-reader"`. Agent works in its own context.

## OPTIONS

**`-r` / `--report`:** Full diagnostic mode. Loads references and report template.

**`-b` / `--bookends`:** Opening/closing analysis. Loads bookends-specific rules (opening, closing, promise).

**`-i` / `--inline`:** Write output inline in the conversation instead of creating a file.

**Default (no flag):** Agent persona alone. No rules files loaded. No report template.

## SUPPORTING FILES

### References

**Loaded in `--report` mode only:**

| Input                                     | Rules loaded                                                  |
| ----------------------------------------- | ------------------------------------------------------------- |
| Excerpt / passage                         | `micro-rules`                                                 |
| Complete scene                            | `micro-rules` `scene-rules`                                   |
| Complete chapter                          | `micro-rules` `scene-rules` `chapter-rules`                   |
| Multiple chapters / manuscript            | `micro-rules` `scene-rules` `chapter-rules` `macro-rules`     |
| Opening (ch.1 or user says "opening")     | `micro-rules` `opening-rules`                                 |
| Closing (last ch. or user says "closing") | `micro-rules` `closing-rules`                                 |
| Both opening + closing                    | `micro-rules` `opening-rules` `closing-rules` `promise-rules` |

Rules path: `skills/qa-reader/references/qa-reader-{name}-rules.md`

**Report format:** `skills/qa-reader/references/qa-reader-report.md`

Never load bookends rules in standard mode. Never load standard meso/macro in bookends mode. Never load macro for a single scene or chapter.

## OUTPUT

**In `--report` mode, output MUST follow the report template.** This is non-negotiable.

- File path provided → write there.
- **No path** → ask before creating, else create a `.md` file in `/mnt/user-data/outputs/`.
- `--inline` → write inline in the conversation.

## ACTIVATION - DEACTIVATION - HANDOFF

**`[QA-READER]`** — Display this immediately.

**Applies to this response only. Auto-resets after.**
