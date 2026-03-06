---
name: qa-originality
description: "Use when: (1) evaluating creative singularity — voice, concept, freshness, (2) detecting clichés, stock scenes, derivative elements, (3) assessing artistic identity and ambition at manuscript level"
---

## LOAD AGENT

Read `skills/qa-originality/agents/agent-qa-originality.md` — you ARE this persona.

**Option — `-c` / `--context`:** Use the `Agent` tool with `subagent_type: "agent-qa-originality"`. Agent works in its own context.

## OPTIONS

**`-r` / `--report`:** Full diagnostic mode. Loads references and report template.

**`-i` / `--inline`:** Write output inline in the conversation instead of creating a file.

**Default (no flag):** Agent persona alone. No rules files loaded. No report template.

## SUPPORTING FILES

### References

**Loaded in `--report` mode only:**

| Input                          | Rules loaded                             |
| ------------------------------ | ---------------------------------------- |
| Excerpt / passage              | `micro-rules`                            |
| Complete scene or chapter      | `micro-rules` `meso-rules`               |
| Multiple chapters / manuscript | `micro-rules` `meso-rules` `macro-rules` |

Rules path: `skills/qa-originality/references/qa-originality-{name}-rules.md`

**Report format:** `skills/qa-originality/references/qa-originality-report.md` + `references/qa-report-template.md`

## OUTPUT

**In `--report` mode, output MUST follow the report template.** This is non-negotiable.

- File path provided → write there.
- **No path** → ask before creating, else create a `.md` file in `/mnt/user-data/outputs/`.
- `--inline` → write inline in the conversation.

## ACTIVATION - DEACTIVATION - HANDOFF

**`[ORIGINALITY]`** — Display this immediately.

**Applies to this response only. Auto-resets after.**
