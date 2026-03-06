---
name: dialog-ink
description: "Use when: (1) writing dialogue scenes between characters, (2) staging physical movement and spatial action during exchanges, (3) creating raw scene material for prose romanticization"
---

## LOAD AGENT

Read `skills/dialog-ink/agents/agent-dialog-ink.md` — you ARE this persona.

**Option — `-c` / `--context`:** Use the `Agent` tool with `subagent_type: "dialog-ink"`. Agent works in its own context.

## OPTIONS

**`-p` / `--pass`:** Annotated diagnostic on existing prose. Loads `dialog-ink-pass-rules.md`.

**`--check`:** Strict context verification BEFORE writing. Combines with any mode. If no context provided: signal, request material.

**`-i` / `--inline`:** Write output inline in the conversation instead of creating a file.

**Default (no flag):** Scene mode — semi-theatrical script.

## SUPPORTING FILES

### References

| Mode/Flag | Rules loaded |
| --------- | ------------ |
| `scene` | none |
| `pass` | `dialog-ink-pass-rules.md` |
| `--check` | `dialog-ink-rules.md` |

Rules path: `skills/dialog-ink/references/{name}.md`

`--check` combines with any mode: `scene --check`, `pass --check`.

## OUTPUT

- File path provided → write there.
- **No path** → ask before creating, else create a `.md` file in `/mnt/user-data/outputs/`.

## ACTIVATION - DEACTIVATION - HANDOFF

**`[DIALOG]`** — Display this immediately.

**Applies to this response only. Auto-resets after.**
