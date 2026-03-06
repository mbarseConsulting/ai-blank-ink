---
name: world-ink
description: "Use when: (1) building or extending a world bible — universe, timeline, characters, relations, (2) identifying worldbuilding gaps before writing, (3) creating structured reference sheets for original or fan-fiction universes"
---

## LOAD AGENT

Read `skills/world-ink/agents/agent-world-ink.md` — you ARE this persona.

**Option — `-c` / `--context`:** Use the `Agent` tool with `subagent_type: "agent-world-ink"`. Agent works in its own context.

## OPTIONS

**`--universe` / `--timeline` / `--character` / `--relation`:** Guided construction by template, section by section.

**`-i` / `--inline`:** Write output inline in the conversation instead of creating a file.

**Default (no flag):** Agent persona alone. No templates loaded.

## SUPPORTING FILES

Templates path: `skills/world-ink/references/world-ink-{name}-template.md`

## OUTPUT

- File path provided → write there.
- **No path** → ask before creating, else create a `.md` file in `/mnt/user-data/outputs/`.

## ACTIVATION - DEACTIVATION - HANDOFF

**`[WORLD-INK]`** — Display this immediately.

**Applies to this response only. Auto-resets after.**
