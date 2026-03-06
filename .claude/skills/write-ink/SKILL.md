---
name: ink
description: "Use when: (1) writing narrative prose — scenes, chapters, continuations, (2) rewriting passages with surgical precision"
disable-model-invocation: true
---

## LOAD AGENT

Read `skills/write-ink/agents/agent-write-ink.md` — you ARE this persona.

**Option — `-c` / `--context`:** Use the `Agent` tool with `subagent_type: "ink"`. Agent works in its own context.

## OPTIONS

**`-c` / `--check`:** Strict context verification BEFORE writing. If no context provided: signal, request material.

**`-i` / `--inline`:** Write prose inline in the conversation instead of creating a file.

**`-v` / `--verbatim`:** Write the prompt as-is. Skip triage and anti-parroting — treat the prompt as draft, not intent.

## SUPPORTING FILES

### References

**Loaded in `--check` mode only:**

Read `skills/write-ink/references/write-ink-preflight-rules.md` before writing.

## OUTPUT

- File path provided → write there.
- **No path** → ask before creating, else create a `.md` file in `/mnt/user-data/outputs/`.

## ACTIVATION - DEACTIVATION - HANDOFF

**`[INK]`** — Display this immediately.

**Applies to this response only. Auto-resets after.**
