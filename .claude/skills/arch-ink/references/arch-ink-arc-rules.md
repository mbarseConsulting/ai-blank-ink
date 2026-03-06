# arch-ink — Arc Rules

> Loaded by: arch-ink in `--report` mode when arc material is provided (character sheet, explicit want/need, arc trajectory)
> Scope: protagonist arc — want/need, lie, ghost, transformation, subplot resonance
> Default output: Diagnostic (supplements structure-rules)

---

## Axes

### starting-state

**What it looks at:** Who is this character before the story breaks them? Their status quo, comfort zone, routine. The baseline against which all transformation is measured.

**Failure modes:**
- No identifiable status quo — story starts with the character already in crisis
- Status quo is described but never shown in action — reader has no anchor
- Status quo is too comfortable — no friction that prefigures the need for change
- Character starts already self-aware about their flaw — no room for arc

**States:**
- `established` — clear status quo, visible comfort zone, baseline behavior shown
- `sketched` — implied but not dramatized
- `absent` — no identifiable pre-story state

**Default sev:** MAJ

---

### want-need-split

**What it looks at:** Distinction between the protagonist's conscious desire (want) and actual psychological need (need). Are both distinct, active, and in tension?

**Failure modes:**
- Want and need are identical — protagonist wants what they need, no internal conflict
- Want is clear, need is absent or never endangered
- Need is stated explicitly in dialogue without ever being shown structurally

**States:**
- `distinct and in tension` — want and need actively oppose each other, structure exploits both
- `present but passive` — distinction exists but doesn't generate narrative internal conflict
- `absent` — only one register (want without need, or immediate need without deeper desire)

**Default sev:** MAJ

---

### lie-and-ghost

**What it looks at:** The foundational wound (ghost) and the false belief it generates (lie). Do both drive the decisions that propel the plot?

**Failure modes:**
- Ghost is biographical information without narrative consequence (mentioned, not exploited)
- Lie is never endangered by story events
- Wound and lie are identical — no distinction between the fact and the belief it generates
- Ghost is revealed too early, reducing arc tension

**States:**
- `driving` — ghost and lie generate identifiable decisions across the acts
- `present` — identifiable but don't influence major structural beats
- `decorative` — mentioned, no effect on plot
- `absent` — no identifiable ghost or lie

**Default sev:** MAJ

---

### arc-completion

**What it looks at:** Whether the transformation is earned through story events or simply declared.

**Failure modes:**
- Character "realizes" something without the story having forced that realization
- Transformation arrives at the climax without being prepared by micro-transformations in preceding acts
- Arc is purely external (protagonist succeeds at their mission) without internal transformation
- Transformation is reversed but not credible (fall arc without accumulation of failures)

**States:**
- `earned` — transformation flows from events, micro-progressions visible across acts
- `declared` — character changes but the structure hasn't forced the change
- `partial` — external transformation without corresponding internal transformation
- `absent` — character ends where they began, without justification for intentional stasis

**Default sev:** MAJ

---

### subplot-resonance

**What it looks at:** Whether subplots resonate with the main arc (mirror, complication, counterpoint).

**Failure modes:**
- Subplots are narratively disconnected from the main arc — they neither influence nor reflect anything
- A subplot resolves the same theme as the main arc without adding anything — duplication
- Subplots have their own complete arc without contact point with the protagonist
- Subplot resolutions don't inform the climax of the main arc

**States:**
- `resonant` — each major subplot complicates or illuminates the main arc
- `disconnected` — subplots run in parallel without resonance
- `duplicate` — repeats the main arc without distinct contribution

**Default sev:** SUG (single subplot) / MAJ (if all subplots)

---

### transformation-beats

**What it looks at:** The five intermediate steps of the character's journey — not as rigid formula, but as structural checkpoints. Each beat changes the character's relationship to their lie.

**Beats checked:**
1. **Inciting disruption** — what event forces the character out of their comfort zone and into confrontation with their lie?
2. **Resistance phase** — how do they try to solve the problem without changing? What's the cost of clinging to the lie?
3. **Midpoint reckoning** — what truth cracks the lie open, even if they resist? The mirror moment — glimpse of who they could become.
4. **Dark night** — the moment lie and need collide at maximum pressure. The lowest point, where change feels impossible.
5. **Climax choice** — the impossible decision that proves transformation through ACTION. Not declaration, not realization — a CHOICE with consequences.

**Failure modes:**
- Beats are missing — character jumps from status quo to transformed without intermediate steps
- Beats are present but out of order — reckoning before resistance, dark night without buildup
- Transformation happens through realization rather than action — character "understands" without choosing
- Resistance phase is skipped — character changes at the first sign of pressure (too easy)
- Dark night is external only — bad things happen TO the character rather than the lie creating the crisis

**States:**
- `complete` — all five beats identifiable and ordered, each changes the character's relationship to the lie
- `partial` — some beats present, gaps in the journey
- `collapsed` — transformation happens in one or two jumps without intermediate steps

**Default sev:** MAJ

---

### landing-state

**What it looks at:** Three aspects of where the character ends up — arrival (how they are measurably different from page 1), cost of change (what transformation cost them — nothing is free), and thematic embodiment (does the character's arc PROVE the story's controlling idea — their journey IS the argument).

**Failure modes:**
- Character is "changed" but the change isn't measurable — no new behavior, no new capability, no new acceptance
- Transformation is costless — the character gained everything and lost nothing
- Arc resolves positively regardless of the story's theme — happy ending imposed on a dark thesis
- Character's journey doesn't connect to the controlling idea — the arc and the theme run on parallel tracks
- Cost of change is melodramatic rather than organic — loss feels manufactured

**States:**
- `earned and resonant` — arrival is measurable, cost is real, arc embodies the theme
- `arrived but hollow` — character changed, but cost or thematic connection is missing
- `unresolved` — no clear landing state — arc trails off

**Default sev:** MAJ

---

## Loading rules

- Required: arc material provided (character sheet with want/need, outline with arc, or explicit author statement).
- Always supplementing structure-rules, never standalone.
- Priority: `lie-and-ghost`, `transformation-beats`, and `arc-completion` — correcting subplot-resonance before them is premature.
- `starting-state` and `landing-state` frame the arc — if both are absent, the arc has no measurable delta.
