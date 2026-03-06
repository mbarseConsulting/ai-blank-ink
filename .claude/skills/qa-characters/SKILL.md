---
name: qa-characters
description: "Use when: (1) evaluating character psychology and credibility, (2) assessing character arcs and transformation, (3) analyzing cast dynamics and relationships, (4) checking if a character knows/reacts to things they shouldn't"
---

## LOAD AGENT

Read `skills/qa-characters/agents/agent-qa-characters.md` — you ARE this persona.

**Option — `-c` / `--context`:** Use the `Agent` tool with `subagent_type: "agent-qa-characters"`. Agent works in its own context.

## OPTIONS

**`-r` / `--report`:** Full diagnostic mode. Loads references and report template.

**`-i` / `--inline`:** Write output inline in the conversation instead of creating a file.

**Default (no flag):** Agent persona alone. No rules files loaded. No report template.

## SUPPORTING FILES

### References

**Loaded in `--report` mode only:**

| Input                                     | Rules loaded                                     |
| ----------------------------------------- | ------------------------------------------------ |
| Single character (sheet + text)           | `depth-rules`                                    |
| Character across multiple scenes/chapters | `depth-rules` `evolution-rules`                  |
| Multiple characters interacting           | `depth-rules` `dynamics-rules`                   |
| Full cast analysis                        | `depth-rules` `evolution-rules` `dynamics-rules` |

Rules path: `skills/qa-characters/references/qa-characters-{name}-rules.md`

**Report format:** `skills/qa-characters/references/qa-characters-report.md` + `references/qa-report-template.md`

Depth-rules is always loaded in report mode. Evolution and dynamics depend on material.

## OUTPUT

**In `--report` mode, output MUST follow the report template.** This is non-negotiable.

- File path provided → write there.
- **No path** → ask before creating, else create a `.md` file in `/mnt/user-data/outputs/`.
- `--inline` → write inline in the conversation.

## ACTIVATION - DEACTIVATION - HANDOFF

**`[CHARACTERS]`** — Display this immediately.

**Applies to this response only. Auto-resets after.**
