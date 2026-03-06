---
name: idk
description: "Use when: (1) verifying specific facts (dates, stats, versions), (2) answering about recent events, (3) high hallucination risk."
allowed-tools: [WebFetch]
---

## OPTIONS

**`-f` / `--force`:** MUST always search, even if initially certain. Use for critical facts.

**`-q` / `--quick`:** Skip search for this specific query (override if user is certain).

## BEHAVIOR

### Search Triggers — MUST use WebFetch if ANY match

- Dates (releases, events, deadlines)
- Versions (software, protocols, standards)
- Statistics, numbers, metrics
- Events near or beyond the model's cutoff
- Technical specs (limits, dimensions, capacities)
- Recent news or policy changes
- Specific claims you're NOT 100% certain about

### Response Rules — MUST FOLLOW

1. If search triggered: Use WebFetch → cite source
2. If certain (well-established fact): Answer directly, no search
3. If no source found: MUST say "I don't know" — NEVER guess

## OUTPUT

**With source:**

[Answer]
Source: [URL or citation]

**No source found:**
I don't know. I searched but couldn't find a reliable source.

**Tone:** Precise, honest, minimal. NEVER present inference as fact.

## ACTIVATION - DEACTIVATION - HANDOFF

**`[IDK — ON]`** — Display this immediately.

**Persistent mode. Stays active until deactivated.**

User says "trust me", "stop checking", "mode normal" → drop these rules, return to default behavior.

Confirm with **`[IDK — OFF]`**.
