# arch-ink — Report Format

> Editorial role: structural architect
> Default output: Diagnostic
> Content language: match the text's language. Column headers: English. Axis names and severity codes: English.

---

## Severity

| Code | Level | Meaning |
|------|-------|---------|
| `BLK` | Blocking | Must fix. The structure is broken. |
| `MAJ` | Major | Should fix. The structure is weakened. |
| `SUG` | Suggestion | Could strengthen. Author decides. |

---

## Axis registry

### Macro (structure-rules — always loaded for synopsis/outline)

| Axis | Source | Default sev |
|------|--------|-------------|
| act-breaks | structure-rules | MAJ |
| midpoint | structure-rules | MAJ |
| central-conflict | structure-rules | BLK |
| crisis | structure-rules | BLK |
| thematic-throughline | structure-rules | MAJ |
| promise-delivery | structure-rules | BLK |
| reversal-credibility | structure-rules | MAJ |
| stakes-escalation | structure-rules | MAJ |
| tension-architecture | structure-rules | MAJ |
| information-design | structure-rules | MAJ |

### Macro (arc-rules — loaded when character arc material provided)

| Axis | Source | Default sev |
|------|--------|-------------|
| starting-state | arc-rules | MAJ |
| want-need-split | arc-rules | MAJ |
| lie-and-ghost | arc-rules | MAJ |
| transformation-beats | arc-rules | MAJ |
| arc-completion | arc-rules | MAJ |
| landing-state | arc-rules | MAJ |
| subplot-resonance | arc-rules | SUG |

### Meso (script-rules — loaded when script/chapter plan provided)

| Axis | Source | Type | Default sev |
|------|--------|------|-------------|
| chapter-function | script-rules | finding | BLK |
| chapter-change | script-rules | finding | BLK |
| scene-function | script-rules | finding | MAJ |
| scene-sequence | script-rules | finding | MAJ |
| scene-completeness | script-rules | finding | MAJ |
| information-sequence | script-rules | finding | MAJ |
| entry-exit-states | script-rules | finding | MAJ |
| script-specificity | script-rules | finding | SUG |
| pov-choice | script-rules | finding | SUG |
| scene-tension | script-rules | finding | MAJ |
| turning-point | script-rules | finding | MAJ |
| subtext | script-rules | finding | SUG |
| setup-payoff | script-rules | finding | MAJ |
| emotional-trajectory | script-rules | finding | MAJ |
| structural-soundness | script-rules | verdict | — |
| writing-readiness | script-rules | verdict | — |

---

## Default format: Diagnostic

```markdown
## arch-ink — Diagnostic

### Context
- **Material**: [synopsis / outline / arc sheet — length, type]
- **Rules loaded**: [structure-rules / structure-rules + arc-rules]
- **Genre / comparisons**: [if provided — or "not provided"]

### First Impression
[2-3 sentences. Gut read on structural soundness. Not a summary — the instinct.]

### Findings

| # | Sev | Axis | Beat | Issue | Direction |
|---|-----|------|------|-------|-----------|
| 1 | BLK | central-conflict | Act 1 | [≤15 words, in text language] | [≤15 words, in text language] |

> Direction column: MANDATORY. No finding without a concrete suggestion.

### Patterns

| Pattern | Count | Examples | Sev | Direction |
|---------|-------|----------|-----|-----------|
| [name] | N | beat A, beat B, beat C | MAJ | [systemic direction ≤15 words, in text language] |

> 3+ occurrences of same type = pattern. Max 3 examples. One systemic direction.
> If none: omit section.

### Structural grid

| Axis | State | Note | Suggestion |
|------|-------|------|------------|
| act-breaks | solid / misplaced / absent | [one sentence, in text language] | [one sentence, in text language] |
| midpoint | reversing / weak / absent | [one sentence, in text language] | [one sentence, in text language] |
| central-conflict | clear and persistent / vague / absent | [one sentence, in text language] | [one sentence, in text language] |
| crisis | impossible choice / forced but weak / absent | [one sentence, in text language] | [one sentence, in text language] |
| thematic-throughline | visible and anchored / present but passive / absent | [one sentence, in text language] | [one sentence, in text language] |
| promise-delivery | honored / partial / betrayed | [one sentence, in text language] | [one sentence, in text language] |
| reversal-credibility | prepared / partially prepared / pulled from nowhere | [one sentence, in text language] | [one sentence, in text language] |
| stakes-escalation | escalating / flat / inverted | [one sentence, in text language] | [one sentence, in text language] |
| tension-architecture | well-paced / monotone / arrhythmic | [one sentence, in text language] | [one sentence, in text language] |
| information-design | designed / accidental / flat | [one sentence, in text language] | [one sentence, in text language] |

> Arc-rules (if loaded):

| Axis | State | Note | Suggestion |
|------|-------|------|------------|
| starting-state | established / sketched / absent | [one sentence, in text language] | [one sentence, in text language] |
| want-need-split | distinct and in tension / present but passive / absent | [one sentence, in text language] | [one sentence, in text language] |
| lie-and-ghost | driving / present / decorative / absent | [one sentence, in text language] | [one sentence, in text language] |
| transformation-beats | complete / partial / collapsed | [one sentence, in text language] | [one sentence, in text language] |
| arc-completion | earned / declared / partial / absent | [one sentence, in text language] | [one sentence, in text language] |
| landing-state | earned and resonant / arrived but hollow / unresolved | [one sentence, in text language] | [one sentence, in text language] |
| subplot-resonance | resonant / disconnected / duplicate | [one sentence, in text language] | [one sentence, in text language] |

### Strengths

| Beat | Axis | Mechanism | Effect |
|------|------|-----------|--------|
| [identified beat] | [axis] | [what the author did] | [why it works] |

> Never empty unless genuinely nothing works — in which case state that explicitly.

### Global verdict

[1-2 sentences. The structural truth.]

### Priority fixes

1. BLK #[N] — [highest-impact action, in text language]

> Max 5. Impact order. Empty if no findings.
```

---

## Script Diagnostic (when script-rules loaded)

```markdown
## arch-ink — Script Diagnostic

### Context
- **Material**: [script / chapter plan — chapter N, book title]
- **Rules loaded**: [script-rules / script-rules + structure-rules / script-rules + arc-rules]
- **Arc context**: [arc outline provided — or "not provided"]
- **Adjacent chapters**: [previous/next scripts provided — or "not provided"]

### First Impression
[2-3 sentences. Gut read on whether this script is ready to write. Not a summary — the instinct.]

### Findings

| # | Sev | Axis | Scene | Issue | Direction |
|---|-----|------|-------|-------|-----------|
| 1 | BLK | chapter-function | — | [≤15 words, in text language] | [≤15 words, in text language] |
| 2 | MAJ | scene-function | Sc. 3 | [≤15 words, in text language] | [≤15 words, in text language] |

> Direction column: MANDATORY. No finding without a concrete suggestion.

### Scene Grid

| Scene | Function | Change | Tension | Turning point | POV | Subtext | Emotional delta | Verdict |
|-------|----------|--------|---------|---------------|-----|---------|-----------------|---------|
| Sc. 1 | [load-bearing / supporting / dead-weight] | [what changes] | [charged / present / inert] | [pivots / drifts / flat] | [optimal / functional / default] | [layered / thin / absent] | [entry → exit] | [one sentence, in text language] |
| Sc. 2 | | | | | | | | |

### Chapter Grid

| Axis | State | Note | Suggestion |
|------|-------|------|------------|
| chapter-function | drives-arc / supports-arc / decorative / aimless | [one sentence, in text language] | [one sentence, in text language] |
| chapter-change | irreversible / shift / static | [one sentence, in text language] | [one sentence, in text language] |
| scene-completeness | complete / gap / major-gap | [one sentence, in text language] | [one sentence, in text language] |
| information-sequence | well-sequenced / could-improve / problematic | [one sentence, in text language] | [one sentence, in text language] |
| entry-exit-states | propulsive / adequate / dead-end | [one sentence, in text language] | [one sentence, in text language] |
| setup-payoff | linked / isolated / telegraphed | [one sentence, in text language] | [one sentence, in text language] |

### Strengths

| Scene / Beat | Axis | Mechanism | Effect |
|-------------|------|-----------|--------|
| [identified scene] | [axis] | [what the author did] | [why it works] |

### Verdicts

| Verdict | State | Justification | Suggestion |
|---------|-------|---------------|------------|
| structural-soundness | solid / weakened / broken | [one sentence, in text language] | [one sentence, in text language] |
| writing-readiness | ready / needs-revision / premature | [one sentence, in text language] | [one sentence, in text language] |

### Global verdict
[1-2 sentences. Is this script ready to write?]

### Priority fixes
1. BLK #[N] — [highest-impact action, in text language]

> Max 5. Impact order. Empty if no findings.
```

---

## Alternative: Assessment

On specific author question ("does my act 2 work?", "is my midpoint strong enough?").

```markdown
## arch-ink — Assessment

### Verdict
[Direct answer. 2-3 sentences. Yes / no / partially + WHY. In French.]

### Context
- **Material**: [synopsis / outline / arc sheet]
- **Rules loaded**: [structure-rules / arc-rules]

### Grid

| Axis | State | Note | Suggestion |
|------|-------|------|------------|
| [relevant axis] | [state] | [one sentence, in text language] | [one sentence, in text language] |

### Key issues
[If problems found. Beat cited, with direction. In French.]

### Strengths
[What works. Mechanism named. In French.]

### Suggestions
[2-5 concrete next steps, priority order. In French.]
```

---

## Alternative: Inline

On explicit request ("annotate my outline").

```markdown
## arch-ink — Inline

[For each issue, in document order:]

**[Beat / section]** — [summary ≤5 words]
SEV axis — [issue, ≤15 words, in text language]
→ [proposed fix, in text language]

[For notable strengths:]

**[Beat / section]**
✓ [structural mechanism that works — one sentence, in text language]

---

[Global verdict: 1-2 sentences, in French.]
```

---

## Rules

### Suggestion law
Every structural finding gets a concrete direction. No naked criticism.

- Finding → Direction column
- Grid → Suggestion column (even for positive states: "maintain" or "push further")
- Verdict → recommended action

### Citation
1. Every finding identifies the relevant beat (act, chapter, sequence).
2. Name the missing mechanism — not "doesn't work", WHY.
3. One direction per finding.

### Severity adjustment
Default from the registry. Adjust per context:
- Weak midpoint in an experimental narrative → SUG, not MAJ.
- Absent central conflict in a commercial thriller → BLK.
- Context governs. The default is a starting point.
