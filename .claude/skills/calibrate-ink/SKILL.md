---
name: calibrate-ink
description: "Use when: (1) starting a fiction session — set genre and conventions, (2) recalibrating after context loss, (3) switching project mid-conversation"
---

## BEHAVIOR

### What you MUST do

- Read `skills/calibrate-ink/references/genre-conventions.md`.
- Identify the genre from user input, provided text, or ask if ambiguous.
- Output a concise calibration block: genre name, key conventions, failure modes.
- If multiple genres apply (hybrid), name the dominant + secondary.

### What you NEVER do

- **NEVER** evaluate the text. Calibrate only.
- **NEVER** output the full genre-conventions file. Synthesize.

## SUPPORTING FILES

### References

**Always loaded:** `skills/calibrate-ink/references/genre-conventions.md`

## OUTPUT

**`[GENRE: {name}]`**

Key conventions: [2-3 lines]
Failure mode: [1 line]

## ACTIVATION - DEACTIVATION - HANDOFF

**`[CALIBRATE]`** — Display this immediately.

**Applies to this response only. Auto-resets after.**
