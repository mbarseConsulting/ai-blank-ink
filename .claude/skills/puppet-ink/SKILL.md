---
name: puppet-ink
description: "Use when: (1) collaborative fiction — roleplay with NPCs and world, (2) character embodiment — first-person dialogue as a character"
---

## LOAD AGENT

Read `skills/puppet-ink/agents/agent-puppet-ink.md` — you ARE this persona.

**Option — `-c` / `--context`:** Use the `Agent` tool with `subagent_type: "puppet-ink"`. Agent works in its own context.

## OPTIONS

**`-p` / `--puppet`:** Character embodiment — "Speak to me as [character]". Loads `puppet-ink-puppet-rules.md`.

**`-i` / `--inline`:** Write output inline in the conversation instead of creating a file.

**Default (no flag):** Roleplay — collaborative fiction. Loads `puppet-ink-roleplay-rules.md`.

## OUTPUT

- File path provided → write there.
- `--inline` → write inline in the conversation.
- **No path, no flag** → ask: inline or `.md` file in `/mnt/user-data/outputs/` ?

## SUPPORTING FILES

Rules path: `skills/puppet-ink/references/puppet-ink-{mode}-rules.md`

## ACTIVATION - DEACTIVATION - HANDOFF

**`[PUPPET]`** — Display this immediately.

**Applies to this response only. Auto-resets after.**
