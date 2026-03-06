# Agent Template

## Structure

```markdown
---
name: kebab-case-name
description: "[What it does. What it does NOT do.]"
tools: []
model: sonnet
color: white
---

**`[TAG]`** — Display at the start of your first response.

## ROLE

[Persona in 2-3 sentences. Who, what they care about, how they operate.]

**Style:** [Register — direct, concise, technical, etc.]

## OPTIONS

- **[Mode A]** — [scope]. Default.
- **[Mode B]** — [scope]. Loaded when [trigger].

## BEHAVIOR

### What you MUST do

- [Core directive]
- [Core directive]

### What you NEVER do

- [Guardrail]
- [Guardrail]

## FOCUS

[Optional - The agent's methodology — how it works and what it evaluates.]

## OUTPUT

[Optional]

## HANDOFF

[Optional]
```

## Frontmatter Fields

> Reference: https://code.claude.com/docs/en/sub-agents

**Required:** `name`, `description`

| Field             | Required | Description                                                         |
| ----------------- | -------- | ------------------------------------------------------------------- |
| `name`            | Yes      | Unique identifier, lowercase letters and hyphens only               |
| `description`     | Yes      | When Claude should delegate to this subagent                        |
| `tools`           | No       | Allowlist of tools. Inherits all tools if omitted                   |
| `disallowedTools` | No       | Denylist of tools to remove from inherited list                     |
| `model`           | No       | `sonnet`, `opus`, `haiku`, or `inherit` (default: `inherit`)        |
| `permissionMode`  | No       | `default`, `acceptEdits`, `dontAsk`, `bypassPermissions`, or `plan` |
| `maxTurns`        | No       | Maximum agentic turns before the subagent stops                     |
| `skills`          | No       | Skills to inject into the subagent's context at startup             |
| `mcpServers`      | No       | MCP servers available to this subagent                              |
| `hooks`           | No       | Lifecycle hooks scoped to this subagent                             |
| `memory`          | No       | Persistent memory scope: `user`, `project`, or `local`              |
| `background`      | No       | Set to `true` to always run as a background task                    |
| `isolation`       | No       | Set to `worktree` to run in an isolated git worktree                |
