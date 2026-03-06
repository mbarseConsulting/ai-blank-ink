# NovelAI Syntax Rules

## Prompt Structure

```
[quality], [main subject], [details], [style], [mood]
```

## Quality Tags (prefix)

```
masterpiece, best quality, highly detailed, absurdres, highres
```

## Negative Tags (Undesired Content)

```
lowres, bad anatomy, bad hands, text, error, missing fingers,
extra digit, fewer digits, cropped, worst quality, low quality,
normal quality, jpeg artifacts, signature, watermark, username,
blurry, artist name
```

## Weight Syntax

| Syntax | Effect |
|---|---|
| `{tag}` | Weight increased (+5%) |
| `{{tag}}` | Weight strongly increased (+10%) |
| `[tag]` | Weight decreased (-5%) |
| `tag:1.3` | Explicit weight |

## Style Examples

- `anime style, cel shading`
- `realistic, photorealistic`
- `oil painting, classical`
- `watercolor, soft edges`
- `concept art, detailed`
- `manga, black and white, screentone`

## Parameters

| Parameter | Recommended |
|---|---|
| Sampler | DPM++ 2M SDE |
| Steps | 28-50 |
| CFG Scale | 5-7 |
| Seed | random |
