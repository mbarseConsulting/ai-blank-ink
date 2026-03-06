---
name: scan-ink
description: "Use when: (1) analyzing a comic/manga/BD scan, (2) extracting dialogue and script from visual pages, (3) describing scenes or identifying characters from illustrated panels"
---

# scan-ink

**`[SCAN-INK]`** — Display this immediately.
**Applies to this response only. Auto-resets after.**

## OPTIONS

**`--script`:** Extract script only (dialogue + stage directions). No scene description, no tags.

**`--scene`:** Scene description only. No dialogue extraction.

**`--tags`:** Categorized tags only.

**`--characters`:** Focus on character identification and description.

**Default (no flag):** Full analysis — dialogue + scene + summary.

---

## CORE PROCESS — **MUST APPLY ALL**

1. **MUST read the image before any output** — Analyze the full page: panels, reading order, text, characters, backgrounds.
2. **MUST preserve reading order** — Left-to-right for Western comics/BD, right-to-left for manga. Ask if ambiguous.
3. **MUST distinguish text types** — Dialogue, internal monologue, narration, sound effects, environmental text.
4. **MUST indicate confidence** — When text is illegible or content is ambiguous, say so. Never fabricate.
5. **NEVER invent dialogue** — Extract only what is visible. If a bubble is unreadable, mark it `[illegible]`.
6. **NEVER generate content beyond the image** — No worldbuilding, no story continuation, no prompt generation.

**Focus:**

- Accuracy over completeness — better to flag uncertainty than guess
- Cultural context — recognize comic traditions (manga, BD, American comics, manhwa)
- Preserve original language — translate only if requested

---

## OUTPUT

### Default (full analysis)

```
## Page Analysis

### Layout
[Panel count, reading order, notable composition]

### Script

**Panel 1:**
[Character/Narrator/SFX]: "Text"
*[Stage direction: action, expression, movement]*

**Panel 2:**
[...]

### Scene
[Setting, atmosphere, mood, notable visual elements]

### Summary
[What happens on this page in 2-3 sentences]
```

### --script

```
**Panel 1:**
[Character]: "Dialogue"
[Narrator]: Narration text
[SFX]: Sound effect
*[Action/expression/movement]*

**Panel 2:**
[...]
```

### --scene

Structured prose: setting, characters present, action, mood, notable visual elements.

### --tags

```
- Characters: [names or descriptions]
- Setting: [location, time, atmosphere]
- Themes: [narrative themes]
- Genre: [genre markers]
- Visual Style: [art style descriptors]
- Content: [action types, emotional content]
```

### --characters

```
## Character: [Name or description]
- Appearance: [physical description, clothing]
- Position: [where in the panel, doing what]
- Expression: [emotional state, body language]
- Interactions: [with whom, how]
```
