# Macro Rules — qa-consistency

> Scope: macro
> Loaded when: text >= multiple chapters or full manuscript
> Evaluates: Manuscript-level consistency — arcs, lore, worldbuilding, facts
> Does NOT evaluate: scene physics, cross-scene tracking
> Preferred output: diagnostic

---

## Finding Axes

### arc-tracking

- **Detects**: Setup without payoff (Chekhov violations). Payoff without setup. Foreshadowing that leads nowhere. Character goals that vanish. Thematic threads abandoned.
- **Works**: Every promise made to the reader is honored, subverted, or deliberately acknowledged. A gun shown in act 1 fires in act 3. A character goal established early reaches resolution.
- **Fails**: A mysterious letter introduced in chapter 4 is never mentioned again. A character's stated goal vanishes by the midpoint. A prophecy is set up and forgotten.
- **Fix pattern**: For missing payoff: add the resolution or remove the setup. For missing setup: plant the seed earlier. Every thread the reader tracks must be resolved or acknowledged.
- **Example**:
  > Ch.4 "Elle glissa la lettre dans sa poche — elle la lirait plus tard." (Never mentioned again)
  > → Arc : lettre introduite ch.4, jamais résolue — promesse abandonnée
  > → Ajouter la scène de lecture ou retirer la lettre si non-essentielle

### lore-consistency

- **Detects**: World rules that change for plot convenience. Magic systems with shifting costs. Political structures that contradict established rules. Geography that moves.
- **Works**: Rules established early hold throughout. Magic costs what it costs. Geography stays fixed. Factions behave according to established logic. No convenient exceptions.
- **Fails**: Magic is established as exhausting, but the protagonist casts endlessly in the climax. A river flows east in chapter 2 and west in chapter 20.
- **Fix pattern**: Align the violation with the established rule, or establish an exception BEFORE it's needed (setup for the payoff).
- **Example**:
  > Ch.3 "La magie drainait les forces vitales." Ch.28 "Il enchaîna les sorts sans fatigue."
  > → Lore : magie épuisante (ch.3), usage illimité (ch.28) — exception non justifiée
  > → Ajouter le coût ou établir pourquoi cette exception existe

### worldbuilding-coherence

- **Detects**: World rules without consequences. Society that doesn't reflect its own constraints. Setting as backdrop instead of active force.
- **Works**: Geography shapes culture. Economy shapes power. History shapes conflict. Magic reshapes society. The world's rules have visible consequences in how characters live.
- **Fails**: A world with strict class hierarchy where characters move freely between classes. A magical society that functions exactly like a non-magical one. Setting is wallpaper.
- **Fix pattern**: Trace the world rule to its social consequence. If the consequence is missing, add it. The world should constrain characters and create friction.
- **Example**:
  > (Medieval setting with strict caste system, but protagonist moves freely between social circles)
  > → Cohérence : système de castes établi, aucune conséquence sur les déplacements
  > → Ajouter les frictions : refus, humiliations, stratégies de contournement

### ages-and-aging

- **Detects**: Age inconsistencies across the manuscript. Characters not aging with time skips. Age-inappropriate behavior or capability.
- **Works**: A 15-year-old in chapter 1 with a 2-year time skip is 17 in chapter 15. Age-appropriate behavior, knowledge, and physical capability throughout.
- **Fails**: A child established as 8 displays the reasoning of a 15-year-old. A character's age doesn't advance with established time skips. Birth dates contradict stated ages.
- **Fix pattern**: Build an age tracker. Cross-reference with time skips. Adjust behavior to match established age or correct the stated age.
- **Example**:
  > Ch.1 "Léa, huit ans" ... (2 ans plus tard) ... Ch.15 "Léa, huit ans, courut vers lui."
  > → Âge : Léa 8 ans (ch.1), toujours 8 ans après 2 ans — erreur de vieillissement
  > → Corriger : "Léa, dix ans" ou ajuster le saut temporel

### fact-checking

- **Detects**: Historical, procedural, or geographical errors in real-world settings. Anachronistic technology. Inaccurate professional jargon. Wrong real-world geography.
- **Works**: 1920s Paris has appropriate cars and telephones. Medical procedures follow real-world logic. Professional jargon is accurate. Real places are correctly described.
- **Fails**: A character uses a mobile phone in 1985 Paris. A surgical procedure is described inaccurately. A real street is placed in the wrong arrondissement.
- **Fix pattern**: Verify the fact and correct the reference. If the error is plot-critical, adjust the surrounding context to accommodate the correction.
- **Example**:
  > "En 1920, il décrocha son téléphone portable."
  > → Anachronisme : téléphone portable en 1920
  > → Corriger : téléphone fixe, ou ajuster la date si le contexte le permet
