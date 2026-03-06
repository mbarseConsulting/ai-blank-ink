---
name: parse
description: "Analyze intent and reformulate human language into AI-optimized instructions before writing to files."
disable-model-invocation: true
---

## OPTIONS

**`-s` / `--show`:** Display the full analysis (Intent, Gap, Reformulated) before writing. Without this flag, parse works silently — blacklist + reformulate internally, output the result directly.

## BEHAVIOR

### 1. Build a Blacklist

Before anything else: extract every meaningful word from the user's prompt (nouns, verbs, adjectives, adverbs — skip articles/prepositions). This is your blacklist. You MUST NOT reuse any of these words in your output. Find synonyms, rephrase from scratch, change the angle. If a blacklisted word slips through, you failed.

### 2. Analyze Intent

- **Intent:** what the user actually wants
- **Gap:** what's unclear, missing, or implied

If something's unclear → ask one question, don't write yet.

Only display this step if `--show` is active.

### 3. Reformulate

Transform human language → AI language:

- Make implicit → explicit
- Structure clearly (bullets, hierarchy, logic)
- Zero vocabulary overlap with the original prompt — every concept must be expressed with different words
- If you catch yourself reaching for a word from the blacklist, stop and find another path

Only display the reformulated version if `--show` is active.

### 4. Quality Check

Two questions:
1. "Would an AI with zero context understand this correctly?" If no → clarify or ask.
2. "Does any word from my blacklist appear in the output?" If yes → rewrite that part.

## ACTIVATION - DEACTIVATION - HANDOFF

**`[PARSE MODE — ON]`** — Display this immediately.

**Applies to this response only. Auto-resets after.**

Wait for: "ok" / "go" / "écris" / "c'est bon"
Then write + confirm **`[PARSE MODE — OFF]`**
