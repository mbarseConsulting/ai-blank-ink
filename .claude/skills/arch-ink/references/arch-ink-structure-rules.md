# arch-ink — Structure Rules

> Loaded by: arch-ink in `--report` mode for any synopsis or outline
> Scope: macro structure — acts, inciting incident, midpoint, central conflict, thematic throughline, narrative promises, reversals
> Default output: Diagnostic

---

## Axes

### act-breaks

**What it looks at:** Position of act breaks. Timing of the inciting incident. Quality of act transitions.

**Failure modes:**
- Inciting incident arrives too late (after 25% of the narrative)
- Act transitions don't mark genuine reversals — mere event accumulation
- Act 2 lacks a midpoint split that reorients the protagonist's strategy
- Act 3 is not triggered by the protagonist's lowest point

**States:**
- `solid` — breaks at the right positions, marking genuine reversals
- `misplaced` — breaks exist but at the wrong moment
- `absent` — acts are not structurally distinguishable

**Default sev:** MAJ

---

### midpoint

**What it looks at:** What changes at the structural center. Whether it genuinely reverses the dynamic.

**Failure modes:**
- Midpoint is an action peak without a change in the protagonist's strategy
- Midpoint doesn't reverse the power balance or the protagonist's perception
- False peak: things improve superficially before declining, without a genuine intermediate transformation

**States:**
- `reversing` — genuinely changes the narrative dynamic
- `weak` — action peak without change of direction
- `absent` — no structural pivot at the center

**Default sev:** MAJ

---

### central-conflict

**What it looks at:** The protagonist's external desire. Its clarity. Its persistence across acts.

**Failure modes:**
- External desire shifts between acts without narrative justification
- Central conflict is vague or multiple (protagonist wants A then B then C)
- Antagonists / obstacles don't directly oppose the established desire

**States:**
- `clear and persistent` — established early, maintained, everything opposes it
- `vague` — present but insufficiently defined
- `absent` — protagonist reacts to events without a driving desire

**Default sev:** BLK

---

### thematic-throughline

**What it looks at:** Three layers — thematic premise (what is this story ABOUT, not plot — "the cost of power", "forgiveness as self-destruction"), central question (what question does the story pose to the reader — the question the climax answers), and controlling idea (what truth the story argues for through its ending). All three must be visible in the major structural beats, not just the subtext.

**Failure modes:**
- Theme is declared in dialogue but invisible in structural choices
- Major beats (inciting incident, midpoint, climax) don't crystallize the theme
- Theme shifts or dilutes across acts
- Thematic premise exists but no central question — the story illustrates without interrogating
- Controlling idea contradicts what the plot actually demonstrates
- Climax answers a different question than the one the story posed

**States:**
- `visible and anchored` — premise, question, and controlling idea are present and aligned in the structural beats
- `present but passive` — declared, not structured — or only one of three layers is active
- `absent` — no identifiable thematic thread

**Default sev:** MAJ

---

### promise-delivery

**What it looks at:** What chapter 1 / the opening promises. What the ending delivers or betrays.

**Failure modes:**
- Genre promise (thriller = tension, romance = emotional resolution) is not honored
- Act 1 establishes a tone / style the climax abandons
- Resolution of the central conflict doesn't flow from elements established in the opening

**States:**
- `honored` — the ending delivers what the opening promised
- `partial` — some promises kept, others betrayed
- `betrayed` — the ending contradicts the structural promises of the opening

**Default sev:** BLK

---

### reversal-credibility

**What it looks at:** Whether turning points are prepared or pulled from nowhere.

**Failure modes:**
- A major reversal arrives without setup in the preceding acts
- Twists depend on information withheld from the reader without narrative logic
- Setup-to-payoff ratio is unbalanced: absent setup or disproportionate payoff

**States:**
- `prepared` — reversal elements are established beforehand
- `partially prepared` — reversal has grounding but insufficient
- `pulled from nowhere` — no identifiable setup

**Default sev:** MAJ

---

### crisis

**What it looks at:** The impossible choice that forces the protagonist to reveal who they truly are — not what happens TO them, what they CHOOSE. Usually near the end of act 2 or start of act 3.

**Failure modes:**
- No genuine dilemma — the "right" choice is obvious to the reader
- Crisis is external (something happens) rather than a decision the protagonist makes
- The choice doesn't force the protagonist to confront their lie or need
- Crisis arrives without sufficient pressure buildup — feels arbitrary

**States:**
- `impossible choice` — two options, both costly, the choice reveals character
- `forced but weak` — a choice exists but one option is clearly better
- `absent` — no crisis point, protagonist is carried by events

**Default sev:** BLK

---

### stakes-escalation

**What it looks at:** How stakes build across acts — personal, relational, societal, existential. Each act should raise the cost of failure.

**Failure modes:**
- Stakes remain flat — act 3 risks the same as act 1
- Stakes escalate externally (bigger explosions) without personal escalation
- A major escalation arrives without the reader understanding what's at risk
- Stakes de-escalate mid-story without narrative justification

**States:**
- `escalating` — each act raises the cost of failure across multiple registers
- `flat` — stakes remain at the same level throughout
- `inverted` — stakes decrease or the highest point is too early

**Default sev:** MAJ

---

### tension-architecture

**What it looks at:** The rhythm of peaks and valleys across the manuscript. Breathing room between confrontations. Time pressure and urgency mechanisms (deadlines, countdowns, narrowing windows).

**Failure modes:**
- All tension, no release — reader exhaustion without breathing room
- All valleys, no peaks — story meanders without urgency
- Ticking clock introduced but forgotten or neutralized without consequence
- Tension peaks cluster together without escalation between them

**States:**
- `well-paced` — alternation of tension and release with overall escalation
- `monotone` — same intensity throughout, whether high or low
- `arrhythmic` — peaks and valleys placed without structural logic

**Default sev:** MAJ

---

### information-design

**What it looks at:** What the reader knows vs what characters know at the macro level. Dramatic irony, mystery, and suspense as deliberate structural tools across the manuscript.

**Failure modes:**
- Reader and characters always know the same things — no irony, no suspense, no mystery
- Information asymmetry exists but is never exploited for tension
- A major reveal depends on information that was withheld arbitrarily (no structural logic)
- Dramatic irony is set up but never paid off

**States:**
- `designed` — information asymmetries are deliberate and exploited
- `accidental` — asymmetries exist but aren't leveraged
- `flat` — reader and characters share the same information throughout

**Default sev:** MAJ

---

## Loading rules

- Always loaded in `--report` mode when a synopsis or outline is provided.
- Can operate without arc-rules if no arc material is provided.
- Priority: `central-conflict`, `crisis`, and `promise-delivery` — if all three are absent, everything else is secondary.
