---
name: check
description: "Use when: (1) verifying nothing was missed in recent work, (2) self-audit against user requests, (3) catching gaps before moving on."
allowed-tools: Read
---

## BEHAVIOR

### 1. Gather

Look back at the last 2-3 user prompts and any modifications made (file edits, code changes, responses). Build two lists:

- **ASKED:** every explicit request, constraint, preference, and detail mentioned by the user
- **DONE:** every action taken, change made, or point addressed in response

### 2. Compare

For each item in ASKED, check against DONE:

- **COVERED** — addressed
- **PARTIAL** — touched but incomplete or imprecise
- **MISSED** — not addressed at all

### 3. Report

Display only PARTIAL and MISSED items. If everything is covered, say so in one line and stop.

For each gap:

- What was asked (quote the user)
- What was done (or not done)
- What's still needed

### 4. Detect Parroting

Flag every instance where the response reuses the user's own words, phrasing, or structure instead of actually doing the work. Parroting is not covering — it's faking coverage.

### What you NEVER do

- Make excuses for yourself. You are the one being audited. No "I interpreted it as...", no "the context suggested...".
- Pad the output. No preamble, no summary, no encouragement.
- Mark something as COVERED if it was only vaguely addressed or parroted back. When in doubt: PARTIAL.

## OUTPUT

```
[CHECK]

MISSED (N):
- ...

PARTIAL (N):
- ...

PARROTED (N):
- ...
```

If clean: `[CHECK] All covered.`

**Tone:** Self-critical, blunt, no excuses. You screwed up — own it, list it, fix it.

## ACTIVATION - DEACTIVATION - HANDOFF

**`[CHECK — ON]`** — Display this immediately.

**Single-shot. Runs once, reports, done. No persistent mode.**
