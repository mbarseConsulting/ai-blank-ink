# Script Rules — arch-ink

> Loaded by: arch-ink in `--report` mode when input is a script or chapter plan
> Scope: chapter/scene structure — does the script earn its scenes and serve the arc?
> Evaluates: structural soundness of a chapter plan BEFORE writing
> Preferred output: Diagnostic or Assessment
> Requires: the script. Strengthened by: arc outline, character sheets, previous/next chapter scripts.

---

## Finding Axes

### chapter-function

**What it looks at:** Does the chapter serve a nameable function in the arc? Can you state what the arc loses without this chapter?

**Failure modes:**
- "It's a transition chapter" — transitions are not functions
- "It sets up later events" — setup without standalone value is dead weight
- Chapter function duplicates an adjacent chapter
- Chapter does three things vaguely instead of one thing precisely

**States:**
- `drives-arc` — the arc cannot work without this chapter
- `supports-arc` — the arc would survive, but weaker
- `decorative` — pleasant but removable
- `aimless` — no identifiable function

**Default sev:** BLK

---

### chapter-change

**What it looks at:** What is different at the end of the chapter vs the beginning? Something must change: character position, knowledge, relationships, stakes, power balance.

**Failure modes:**
- Nothing changes — the chapter is a vignette, not a unit of story
- Only external change (event happens) without internal shift
- Change is declared in the script notes but not dramatized in any scene

**States:**
- `irreversible` — something changed that cannot be undone
- `shift` — something moved, but could revert
- `static` — nothing is different at the end

**Default sev:** BLK

---

### scene-function

**What it looks at:** Does each scene have a function within the chapter? Can you name what the chapter loses if the scene is removed?

**Failure modes:**
- Scene exists "for atmosphere" or "to show character" without moving anything
- Scene's function duplicates another scene's function
- Scene does what the previous scene already accomplished
- Function is listed in the script notes but not embedded in the scene's beats

**States:**
- `load-bearing` — chapter breaks without it
- `supporting` — chapter weakens without it
- `dead-weight` — chapter survives without it

**Default sev:** MAJ

---

### scene-sequence

**What it looks at:** Is the scene order the strongest possible? Does each scene earn its position? Would reordering improve tension, revelation timing, or emotional build?

**Failure modes:**
- Scenes could be rearranged without consequence — no causal chain
- Climactic scene is not at the end (anti-climax follows climax)
- Emotional intensity flatlines across consecutive scenes
- A scene depends on information from a later scene

**States:**
- `earned` — each scene's position is justified by what precedes and follows
- `acceptable` — order works but isn't optimal
- `arbitrary` — scenes could be shuffled without consequence

**Default sev:** MAJ

---

### scene-completeness

**What it looks at:** Are all necessary scenes present? Is any step skipped that the reader needs to follow the chapter's logic or emotional arc?

**Failure modes:**
- A logical gap: character goes from state A to state C without B
- An emotional gap: character reacts to something the reader didn't witness
- A missing setup: a scene's impact depends on a beat that was skipped
- Conversely: a scene is present that the reader doesn't need (covered by scene-function)

**States:**
- `complete` — no gap identified
- `gap` — one or more missing steps
- `major-gap` — the chapter's logic breaks without an absent scene

**Default sev:** MAJ

---

### information-sequence

**What it looks at:** Is information revealed in the optimal order across scenes? Each scene should change what the reader knows or believes.

**Failure modes:**
- Reader learns everything in scene 1 — remaining scenes have no revelatory value
- Key information is buried in a low-stakes scene
- A revelation lands before the reader cares about it (no setup)
- Information repeats across scenes without adding new dimension

**States:**
- `well-sequenced` — revelations are timed for maximum impact
- `could-improve` — functional but not optimal
- `problematic` — information order weakens the chapter

**Default sev:** MAJ

---

### entry-exit-states

**What it looks at:** Are the chapter's entry and exit states defined? Is the exit state distinct from the entry state? Does exit create momentum toward the next chapter?

**Failure modes:**
- Entry state is unclear — reader doesn't know where characters stand emotionally/physically
- Exit state mirrors entry state — no progress
- Exit doesn't create forward pull — reader can close the book here
- Entry doesn't follow from the previous chapter's exit (if previous chapter exists)

**States:**
- `propulsive` — clear entry, distinct exit, forward momentum
- `adequate` — defined but not optimized
- `dead-end` — exit creates no reason to continue

**Default sev:** MAJ

---

### script-specificity

**What it looks at:** Is the script concrete enough to write from? Are beats defined, characters positioned, emotional targets clear?

**Failure modes:**
- Vague directions: "they discuss the situation", "tension builds"
- No concrete beats — just scene summaries
- Character emotional states implied but not specified
- Missing anchor: no key line, key gesture, or key image to write toward

**States:**
- `ready-to-write` — a writer could execute this script without guessing
- `needs-precision` — direction is clear but beats are vague
- `too-vague` — more planning needed before writing

**Default sev:** SUG

---

### pov-choice

**What it looks at:** Whose eyes serve each scene best? Who has the most to lose, learn, or hide here? POV is a structural choice, not a default.

**Failure modes:**
- POV defaults to the protagonist without considering alternatives
- POV character is a passive observer — the scene happens around them, not to them
- POV switches between scenes without structural justification
- A scene hides information that the POV character would naturally know — POV chosen to manipulate rather than serve the story

**States:**
- `optimal` — POV character is the one with the most at stake in this scene
- `functional` — POV works but a different character would serve better
- `default` — no evidence of deliberate POV choice

**Default sev:** SUG

---

### scene-tension

**What it looks at:** Two dimensions — the conflict (what opposing forces collide in this scene: external obstacle, internal resistance, interpersonal friction) and the stakes (what the character risks if this goes wrong: safety, trust, identity, life).

**Failure modes:**
- No identifiable conflict — characters agree, nothing opposes the goal
- Conflict exists but stakes are absent — nothing is at risk
- Stakes are stated but not felt — reader knows what's at risk intellectually but not emotionally
- Conflict is external only — no internal or interpersonal dimension

**States:**
- `charged` — clear conflict with felt stakes across at least two registers
- `present` — conflict or stakes exist but only on one register
- `inert` — no identifiable opposition or risk

**Default sev:** MAJ

---

### turning-point

**What it looks at:** The moment within the scene where things shift — the beat that makes this a scene, not a passage. Before the turning point, the scene moves in one direction. After, it moves in another.

**Failure modes:**
- No identifiable pivot — scene runs at the same temperature throughout
- Turning point is a new event arriving from outside rather than emerging from the scene's own dynamics
- Multiple turns of equal weight — scene oscillates without resolution
- Turn arrives at the beginning — the rest of the scene is aftermath (scene started too early)

**States:**
- `pivots` — identifiable moment where the scene's direction changes
- `drifts` — scene moves but without a clear turning point
- `flat` — no change of direction within the scene

**Default sev:** MAJ

---

### subtext

**What it looks at:** What's happening beneath the surface action — what's unsaid, implied, or hidden from the characters themselves. Subtext is the gap between what characters do and what they mean.

**Failure modes:**
- Everything is on the surface — characters say exactly what they feel, do exactly what they mean
- Subtext exists but is immediately made explicit — "she smiled, but inside she was angry"
- No gap between dialogue and intention — characters are transparent
- Scene relies entirely on subtext with no surface action — the reader can't access what's happening

**States:**
- `layered` — visible gap between surface and depth, accessible to the reader
- `thin` — some gap exists but minimal
- `absent` — everything is on the surface, or subtext is inaccessible

**Default sev:** SUG

---

### setup-payoff

**What it looks at:** Chekhov's gun in both directions — what does this scene set up for later? What earlier setup does it pay off? Every scene should participate in at least one setup-payoff chain.

**Failure modes:**
- Scene contains a prominent element (object, information, promise) that is never paid off later
- Scene pays off something that was never set up — payoff feels unearned
- A scene is pure setup without standalone value — exists only to serve a later scene
- Setup is too obvious — reader sees the payoff coming (telegraphed)

**States:**
- `linked` — scene participates in identifiable setup-payoff chains
- `isolated` — scene neither sets up nor pays off — structurally disconnected
- `telegraphed` — setups are too visible, reducing later payoff impact

**Default sev:** MAJ

---

### emotional-trajectory

**What it looks at:** The emotional delta of the scene — what emotion does the reader enter with, what do they leave with? The gap between entry and exit emotion IS the scene's effect.

**Failure modes:**
- Reader enters and exits in the same emotional state — no delta
- Emotional shift is unearned — scene hasn't done the work to move the reader
- Emotional trajectory contradicts the scene's structural purpose — a scene meant to escalate tension ends on comfort
- Consecutive scenes have identical emotional trajectories — monotone

**States:**
- `delta` — clear shift between entry and exit emotion, earned by the scene's beats
- `flat` — reader enters and exits in the same state
- `contradictory` — emotional trajectory works against the scene's structural purpose

**Default sev:** MAJ

---

## Verdict Axes

### structural-soundness

**Assesses:** Overall structural health of the script at chapter level.

**States:**
- `solid` — scenes earn their place, sequence is justified, chapter serves the arc
- `weakened` — functional but with identified gaps or dead weight
- `broken` — chapter doesn't serve a clear function, or scenes don't serve the chapter

---

### writing-readiness

**Assesses:** Can a writer execute this script as-is, or does it need more work?

**States:**
- `ready` — specific enough to write, structurally sound, entry/exit defined
- `needs-revision` — structure is sound but beats need precision, or one structural issue to resolve
- `premature` — structural problems must be fixed before writing begins

---

## Loading rules

- Loaded when input is a script, chapter plan, scene breakdown, or beat sheet.
- Can combine with structure-rules if arc outline is provided (to verify chapter → arc coherence).
- Can combine with arc-rules if character arc material is provided.
- Priority: `chapter-function`, `chapter-change`, and `scene-tension` — if all three fail, everything else is secondary.
