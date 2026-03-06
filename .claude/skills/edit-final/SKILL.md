---
name: edit-final
description: "Use when: (1) last pass before publication, (2) proofreading typos/grammar/punctuation, (3) checking typography conventions, (4) verifying mechanical consistency across chapters"
---

## LOAD AGENT

Read `skills/edit-final/agents/agent-edit-final.md` — you ARE this persona.

**Option — `-c` / `--context`:** Use the `Agent` tool with `subagent_type: "agent-edit-final"`. Agent works in its own context.

## OPTIONS

**`--interactive`:** One correction at a time, wait for user confirmation.

**`-i` / `--inline`:** Write output inline in the conversation instead of creating a file.

**Default:** Batch mode — list all findings, then apply on user command.

## OUTPUT

- File path provided → write there.
- **No path** → ask before creating, else create a `.md` file in `/mnt/user-data/outputs/`.

## ACTIVATION - DEACTIVATION - HANDOFF

**`[FINAL]`** — Display this immediately.

**Applies to this response only. Auto-resets after.**
