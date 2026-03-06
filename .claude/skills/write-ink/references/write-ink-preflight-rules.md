# Preflight Rules — write-ink

> Loaded when: --check flag active
> Purpose: strict context verification before writing

---

## Protocol

**This protocol fires BEFORE writing. Not after.**

### 1. Character Verification

List every character you intend to write in this scene. For EACH character:

| Check | Question | On failure |
|-------|----------|------------|
| Existence | Does this character exist in the provided context? | **STOP** — do not invent. Signal to user. |
| Alive | Is this character alive at this point in the timeline? | **STOP** — no accidental resurrection. Signal. |
| Location | Can this character physically be here? | Justify the movement or correct. |
| Physical state | Injuries, fatigue, clothing since last appearance? | Integrate current state into writing. |
| Knowledge | Does this character know what you're making them say/think? | Remove inaccessible information. |
| Voice | Matches sheet (register, tics, education)? | Adjust before writing. |

### 2. World Verification

| Check | Question | On failure |
|-------|----------|------------|
| Timeline | Are referenced events in the correct order? | Correct chronology. |
| Geography | Are distances and travel times consistent? | Adjust or signal. |
| Season/weather | Consistent with established timeline? | Align. |
| Lore | Are world rules (magic, technology, society) respected? | Correct or signal. |

### 3. Result

- **All checks pass** → write normally per active mode.
- **Issues detected** → list them, propose corrections, wait for validation before writing.

### 4. No context provided

If `--check` is active but no reference material was provided (no sheets, no previous chapters, no world docs):

> "--check active but no reference material provided. To verify, I need character sheets, previous chapters, or world documents. Provide the material or write without --check."
