# Skill Template

Covers both Standalone and Skill+Agent patterns.

**Instructions:**

1. Copy template below
2. Delete conditional sections you don't need
3. Fill placeholders

---

```markdown
---
name: skill-name
description: "Use when: (1) [trigger], (2) [trigger], (3) [trigger]"
tools: [] # Add if needed: [WebFetch, FileRead]
---

<!-- ============================================ -->
<!-- AGENT LOADING: Only for Skill+Agent pattern -->
<!-- Delete this entire section if Standalone    -->
<!-- ============================================ -->

## LOAD AGENT

Read `agents/{agent-name}.md` — you ARE this persona.

**Option — `-c` / `--context`:** Use the `Agent` tool with `subagent_type: "{agent-name}"`. Agent works in its own context.

<!-- ============================================ -->
<!-- END AGENT LOADING -->
<!-- ============================================ -->

## OPTIONS

<!-- Add your custom options: -->

**`-f` / `--flag`:** [Description with MUST/NEVER if critical]

<!-- If no options at all: -->

**NONE** — This skill has no configurable options.

<!-- ============================================ -->
<!-- BEHAVIOR + FOCUS: Only for Standalone        -->
<!-- Delete if Skill+Agent (agent has its own)    -->
<!-- ============================================ -->

## BEHAVIOR

### What you MUST do

- [Core directive]
- [Core directive]

### What you NEVER do

- [Guardrail]
- [Guardrail]

## FOCUS

- [Key aspect 1]
- [Key aspect 2]
- [Key aspect 3]

<!-- ============================================ -->
<!-- END BEHAVIOR + FOCUS -->
<!-- ============================================ -->

## SUPPORTING FILES

### References

Read `references/{reference-name}.md` before processing.

## OUTPUT

**Structure:**

- [Define expected structure]
- [Add formatting requirements]

**Tone:** [Adjectives describing voice]

**Example (optional):**

[Show example output if format is complex]

## ACTIVATION - DEACTIVATION - HANDOFF

**`[TAG]`** — Display this immediately.

<!-- ============================================ -->
<!-- CHOOSE MODE: Keep ONE, delete the others    -->
<!-- ============================================ -->

<!-- MODE: ONE-SHOT -->

**Applies to this response only. Auto-resets after.**

<!-- MODE: PERSISTENT -->

**Persistent mode. Stays active until deactivated.**

<!-- MODE: HYBRID -->

**Mode:** One-shot by default. Use `--stay` to keep active.

<!-- ============================================ -->
<!-- END MODE SELECTION -->
<!-- ============================================ -->

<!-- ============================================ -->
<!-- DEACTIVATION: Only for PERSISTENT or HYBRID -->
<!-- Delete if ONE-SHOT mode                     -->
<!-- ============================================ -->

User says "[keyword 1]", "[keyword 2]", "mode normal" → drop rules, return to default behavior.

Confirm with **`[TAG — OFF]`**.

<!-- ============================================ -->
<!-- END DEACTIVATION -->
<!-- ============================================ -->

<!-- ============================================ -->
<!-- HANDOFF: Optional                           -->
<!-- Delete if no handoff needed                 -->
<!-- ============================================ -->

**Handoff:** [context to pass, to whom]

<!-- ============================================ -->
<!-- END HANDOFF -->
<!-- ============================================ -->
```

## Frontmatter Fields

> Reference: https://code.claude.com/docs/en/skills

**Required:** `name`, `description`

| Field                      | Description                                                         |
| -------------------------- | ------------------------------------------------------------------- |
| `name`                     | Display name. If omitted, uses directory name                       |
| `description`              | What the skill does and when to use it                              |
| `argument-hint`            | Hint shown during autocomplete, e.g. `[issue-number]`               |
| `disable-model-invocation` | `true` = only user can invoke (not Claude auto-load)                |
| `user-invocable`           | `false` = hidden from `/` menu, only Claude can invoke              |
| `allowed-tools`            | Tools Claude can use without permission prompt when skill is active |
| `model`                    | Model to use when this skill is active                              |
| `context`                  | Set to `fork` to run in an isolated subagent context                |
| `agent`                    | Subagent type to use when `context: fork` is set                    |
| `hooks`                    | Hooks scoped to this skill's lifecycle                              |
