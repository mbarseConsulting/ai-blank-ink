---
name: qa-prose
description: "Use when: (1) evaluating sentence-level craft — POV, show-tell, dialogue technique, (2) line editing passages, (3) diagnosing why prose feels flat, over-explained, or generic"
---

## LOAD AGENT

Read `skills/qa-prose/agents/agent-qa-prose.md` — you ARE this persona.

**Option — `-c` / `--context`:** Use the `Agent` tool with `subagent_type: "agent-qa-prose"`. Agent works in its own context.

## OPTIONS

**`-r` / `--report`:** Full diagnostic mode. Loads references and report template.

**`-i` / `--inline`:** Write output inline in the conversation instead of creating a file.

**Default (no flag):** Agent persona alone. No rules files loaded. No report template.

## SUPPORTING FILES

### References

**Loaded in `--report` mode only:**

| Input    | Rules loaded  |
| -------- | ------------- |
| Any text | `craft-rules` |

Rules path: `skills/qa-prose/references/qa-prose-{name}-rules.md`

All rules files are always loaded. One scope (micro), no conditional logic.

**Report format:** `skills/qa-prose/references/qa-prose-report.md` + `references/qa-report-template.md`

## OUTPUT

**In `--report` mode, output MUST follow the report template.** This is non-negotiable.

- File path provided → write there.
- **No path** → ask before creating, else create a `.md` file in `/mnt/user-data/outputs/`.
- `--inline` → write inline in the conversation.

## ACTIVATION - DEACTIVATION - HANDOFF

**`[PROSE]`** — Display this immediately.

**Applies to this response only. Auto-resets after.**
