---
name: prompt-ink
description: "Use when: (1) generating image prompts from narrative content or descriptions, (2) translating character sheets or scenes into visual prompts, (3) creating img2img prompts from reference images"
---

# prompt-ink

**`[PROMPT-INK]`** — Display this immediately.
**Applies to this response only. Auto-resets after.**

## OPTIONS

**`--novelai`:** Load `references/prompt-ink-novelai.md`. Output in NovelAI syntax. **(default)**

**`--midjourney`:** Load `references/prompt-ink-midjourney.md`. Output in Midjourney syntax.

**`--sd`:** Load `references/prompt-ink-stable-diffusion.md`. Output in Stable Diffusion syntax.

**Default (no flag):** NovelAI.

---

## CORE PROCESS — **MUST APPLY ALL**

1. **MUST identify input type** — Description, narrative excerpt, character sheet, or reference image. Adapt extraction accordingly.
2. **MUST translate narrative to visual** — Convert literary descriptions into concrete visual tags. Mood becomes lighting. Emotion becomes expression. Action becomes pose.
3. **MUST prioritize essential elements** — Too many tags = confusion. Keep the prompt focused on what matters most in the scene.
4. **MUST maintain character consistency** — Same character = same visual tags across prompts.
5. **NEVER include text-generating tags** — Avoid prompts that produce text artifacts in the image.
6. **NEVER ignore anatomy safeguards** — Always include relevant negative prompts to avoid anatomical errors.

**Focus:**

- Narrative-to-visual translation fidelity
- Composition and framing appropriate to the scene's intent
- Style coherence across a series of prompts

## TRANSLATION TABLE

### Characters

| Narrative | Prompt |
|---|---|
| "dark hair like ink" | `black hair, ink-like hair, dark hair` |
| "piercing gaze" | `sharp eyes, intense gaze, piercing eyes` |
| "scar on cheek" | `scar on cheek, facial scar` |
| "worn clothes" | `worn clothes, tattered clothing, weathered outfit` |

### Atmosphere

| Narrative | Prompt |
|---|---|
| "palpable tension" | `dramatic lighting, tense atmosphere` |
| "breaking dawn" | `dawn, early morning light, golden hour, sunrise` |
| "abandoned place" | `abandoned, ruins, overgrown, decay` |

### Composition

| Scene | Prompt |
|---|---|
| Emotional close-up | `close-up, portrait, emotional, detailed face` |
| Action scene | `dynamic pose, action shot, motion blur` |
| Establishing shot | `wide shot, landscape, establishing shot` |

---

## OUTPUT

```
## Prompt

### Main prompt
[optimized prompt following loaded model syntax]

### Negative prompt
[negative tags following loaded model syntax]

### Suggested parameters
[model-specific settings]

### Variations
- [Variation 1]
- [Variation 2]

### Notes
- [Choices explanation]
- [Elements difficult to render]
```

**Tone:** Technical, precise. Prompt content in English, interaction in user's language.

**Language:** Prompts always in English. Respond to user in their language.
