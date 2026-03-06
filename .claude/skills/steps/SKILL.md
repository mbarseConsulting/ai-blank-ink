---
name: steps
description: "Use when: (1) analysis needs granular attention, (2) LLM is missing details on long text, (3) precision pass needed on a file or content"
allowed-tools: []
---

## BEHAVIOR

This modifier changes HOW you process input — whether used with a skill, a prompt, or any task that involves analyzing content. It does not change WHAT you do.

### 1. Chunk

Split the input into blocks of ~500 words.

- Cut at the nearest sentence end — never mid-sentence.
- Overlap ~50 words between blocks so nothing falls in a crack.
- If the full input is under 600 words, skip chunking — process normally.

### 2. Process

Run your current task's analysis on each block sequentially.

- Announce which block you're processing: "Block 1/N"
- Apply the full task criteria to each block independently.
- Same depth per block as you would for the whole text.

### 3. Synthesize

After all blocks are processed:

- Compile all findings, deduplicated.
- Group by severity or category (depending on the task).
- Flag anything that spans block boundaries (caught by overlap).

## OUTPUT

**Per block:** skill's normal output, prefixed with block number.

**Final synthesis:** deduplicated findings, grouped, with references to block numbers for traceability.

## ACTIVATION - DEACTIVATION - HANDOFF

**`[STEPS — ON]`** — Display this immediately.

**Persistent mode. Stays active until deactivated.**

User says "stop steps", "normal", "mode normal" → drop chunking, return to default single-pass behavior.

Confirm with **`[STEPS — OFF]`**.
