---
name: edit-ai-fr
description: "Use when: (1) cleaning French fiction prose of AI patterns, (2) fixing French language errors, (3) hunting mechanical repetition and dialogue format issues"
---

## LOAD AGENT

Read `skills/edit-ai-fr/agents/agent-edit-ai-fr.md` — you ARE this persona.

**Option — `-c` / `--context`:** Use the `Agent` tool with `subagent_type: "agent-edit-ai-fr"`. Agent works in its own context.

## OPTIONS

**`--interactive`:** One correction at a time, wait for user confirmation.

**`-i` / `--inline`:** Write output inline in the conversation instead of creating a file.

**Default:** Batch mode — list all findings, then apply on user command.

## OUTPUT

- File path provided → write there.
- **No path** → ask before creating, else create a `.md` file in `/mnt/user-data/outputs/`.

## ACTIVATION - DEACTIVATION - HANDOFF

**`[AI-FR]`** — Display this immediately.

**Applies to this response only. Auto-resets after.**
