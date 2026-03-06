# Midjourney Syntax Rules

## Prompt Structure

Natural language descriptions, more verbose than tag-based systems. End with parameters.

```
[descriptive sentence about the scene] --ar [ratio] --v [version] --style [style]
```

## Weight Syntax

| Syntax | Effect |
|---|---|
| `subject::2` | Double weight on subject |
| `subject::0.5` | Half weight |
| `--no [element]` | Negative prompt (exclude element) |

## Key Parameters

| Parameter | Values |
|---|---|
| `--ar` | `16:9`, `9:16`, `1:1`, `3:2`, `4:3` |
| `--v` | `6`, `6.1` (latest) |
| `--style` | `raw` (photorealistic), default (stylized) |
| `--q` | `0.25`, `0.5`, `1` (quality/speed tradeoff) |
| `--s` | `0-1000` (stylization level) |
| `--c` | `0-100` (chaos/variation) |

## Tips

- Write in natural sentences, not comma-separated tags
- Be specific about lighting, camera angle, and mood
- Reference real artists or art movements for style guidance
- `--style raw` for less Midjourney aesthetic, more literal interpretation
