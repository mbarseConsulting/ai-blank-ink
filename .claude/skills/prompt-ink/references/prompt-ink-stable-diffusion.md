# Stable Diffusion Syntax Rules

## Prompt Structure

Similar to NovelAI. Tag-based, comma-separated. Behavior depends on checkpoint model.

```
[quality], [main subject], [details], [style], [mood]
```

## Weight Syntax

| Syntax | Effect |
|---|---|
| `(tag:1.3)` | Explicit weight (1.0 = default) |
| `(tag)` | Weight +10% |
| `((tag))` | Weight +21% |
| `[tag]` | Weight -10% |

## Negative Prompt

```
(worst quality:1.4), (low quality:1.4), bad anatomy, bad hands,
text, error, missing fingers, extra digit, fewer digits, cropped,
jpeg artifacts, signature, watermark, username, blurry,
deformed, distorted
```

## Key Parameters

| Parameter | Recommended |
|---|---|
| Sampler | DPM++ 2M Karras, Euler a |
| Steps | 20-40 |
| CFG Scale | 7-12 |
| Seed | random |

## Checkpoint-Specific Notes

- **Realistic models** (e.g. RealisticVision): lower CFG (5-7), natural language helps
- **Anime models** (e.g. Anything, NAI-based): tag-based, higher CFG (7-11)
- **LoRA/embeddings**: mention if relevant to the character or style being generated

## Tips

- Prompt order matters: earlier tags have more influence
- Use CLIP skip 2 for anime models, CLIP skip 1 for realistic
- Combine with ControlNet for pose/composition control
