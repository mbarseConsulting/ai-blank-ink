---
name: qa-consistency
description: "Use when: (1) verifying continuity — objects, injuries, timeline, locations, (2) checking lore and worldbuilding coherence, (3) tracking narrative arcs for abandoned threads, (4) fact-checking historical or procedural accuracy"
---

## LOAD AGENT

Read `skills/qa-consistency/agents/agent-qa-consistency.md` — you ARE this persona.

**Option — `-c` / `--context`:** Use the `Agent` tool with `subagent_type: "agent-qa-consistency"`. Agent works in its own context.

## OPTIONS

**`-r` / `--report`:** Full diagnostic mode. Loads references and report template.

**`-i` / `--inline`:** Write output inline in the conversation instead of creating a file.

**Default (no flag):** Agent persona alone. No rules files loaded. No report template.

## SUPPORTING FILES

### References

**Loaded in `--report` mode only.**

| Input                          | Rules loaded                             |
| ------------------------------ | ---------------------------------------- |
| Single scene                   | `micro-rules`                            |
| 2+ scenes / chapter            | `micro-rules` `meso-rules`               |
| Multiple chapters / manuscript | `micro-rules` `meso-rules` `macro-rules` |

Rules path: `skills/qa-consistency/references/qa-consistency-{name}-rules.md`

**Report format:** `skills/qa-consistency/references/qa-consistency-report.md` + `references/qa-report-template.md`

## OUTPUT

**In `--report` mode, output MUST follow the report template.** This is non-negotiable.

- File path provided → write there.
- **No path** → ask before creating, else create a `.md` file in `/mnt/user-data/outputs/`.
- `--inline` → write inline in the conversation.

## ACTIVATION - DEACTIVATION - HANDOFF

**`[CONSISTENCY]`** — Display this immediately.

**Applies to this response only. Auto-resets after.**
