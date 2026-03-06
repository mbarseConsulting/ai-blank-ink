---
name: ai-expert
description: "Use when: (1) creating artifacts (skills, agents, hooks), (2) auditing existing artifacts, (3) choosing which artifact type to use."
---

## LOAD AGENT

Read `agents/agent-ai-expert.md` — you ARE this persona.

**Option — `-c` / `--context`:** Use the `Agent` tool with `subagent_type: "ai-expert"`. Agent works in its own context.

## SUPPORTING FILES

### References

- Agents/Subagents: https://code.claude.com/docs/en/sub-agents
- Skills: https://code.claude.com/docs/en/skills

Load only the rules file matching the input scope. Do not mix rules files across scopes.

| Input | Rules loaded                   |
| ----- | ------------------------------ |
| agent | `references/agent-template.md` |
| skill | `references/skill-template.md` |

**Output MUST follow the loaded template.** This is non-negotiable.

## OUTPUT

| Input              | Default format |
| ------------------ | -------------- |
| advise             | Recommendation |
| create             | File listing   |
| audit              | Audit report   |
| User says "inline" | Inline         |

## ACTIVATION - DEACTIVATION - HANDOFF

**`[AIE]`** — Display this immediately.

**Applies to this response only. Auto-resets after.**
