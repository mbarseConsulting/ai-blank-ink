# Closing Rules — qa-reader

> Scope: bookends
> Loaded when: mode = bookends, text includes closing
> Evaluates: Last paragraph through last chapter — the pages that determine whether the reader recommends the book
> Preferred output: inline or diagnostic

---

## Finding Axes

### final-sentence-resonance

- **Detects**: Last sentences that just end instead of resonating. Flat closings, explanatory wrap-ups, or accidental final lines.
- **Works**: The last sentence lands. It echoes. The reader would remember it tomorrow. It's the departure point — it should reverberate, not just stop.
- **Fails**: The last sentence is informational, decorative, or accidental. It doesn't compress meaning. The book whimpers instead of landing.
- **Fix pattern**: The final sentence should carry the book's emotional or thematic weight in compressed form. Cut everything after the resonant line.
- **Example**:
  > "Et la vie continua comme avant."
  > → Platitude — la dernière phrase ne porte rien, ne résonne pas
  > → Trouver l'image ou la sensation qui condense tout le roman en un souffle

### emotional-precision

- **Detects**: Endings that hit the wrong emotional note. The most dramatic beat instead of the most true. Emotional convenience over psychological truth.
- **Works**: The last beat is the RIGHT emotion for THIS story. Not the most dramatic — the most true. It feels inevitable even if unexpected.
- **Fails**: The ending goes for maximum drama when the story calls for quiet truth. Or ends on hope when the story earned ambiguity. The final emotion serves the author's wish, not the story's logic.
- **Fix pattern**: Ask: what emotion did this story earn? Not what would be satisfying — what is true given everything that happened. End there.
- **Example**:
  > (Dark, complex story ending on a neat, hopeful resolution)
  > → Émotion finale en contradiction avec le ton du roman
  > → Aligner l'émotion de fin sur ce que l'histoire a construit, pas sur le confort

### closure-openness

- **Detects**: Accidental ambiguity (meant to close but didn't) or accidental closure (meant to leave open but resolved). The choice between closed and open ending must be deliberate.
- **Works**: A closed ending resolves. An open ending resonates. The reader can tell the choice was deliberate.
- **Fails**: The ending is ambiguous because the author didn't decide, not because ambiguity serves the story. Or a complex story gets a tidy bow it didn't earn.
- **Fix pattern**: Decide: closed or open. If open, ensure the ambiguity is meaningful (the reader has enough to interpret). If closed, ensure the resolution feels earned.
- **Example**:
  > (Ending where the central conflict is neither resolved nor deliberately left open)
  > → Ambiguïté accidentelle — ni résolu, ni volontairement ouvert
  > → Choisir : fermer avec une résolution gagnée, ou ouvrir avec intention

### climax-payoff

- **Detects**: Climaxes that don't deliver on the tension built. Underwhelming confrontations, deus ex machina, or anticlimactic resolutions.
- **Works**: The highest-stakes moment feels like the consequence of everything before it. The climax is both the destination the story was heading toward and a moment of genuine surprise.
- **Fails**: The climax is resolved by coincidence, external intervention, or a skill/power not previously established. It doesn't feel proportional to the buildup.
- **Fix pattern**: Trace the climax backward — does every necessary element have a setup? Is the resolution earned by character choice, not chance? Strengthen the weakest link.
- **Example**:
  > (Villain defeated by a power the protagonist discovers in the final chapter)
  > → Deus ex machina — le pouvoir n'est pas établi, le climax n'est pas gagné
  > → Planter le pouvoir plus tôt — setup en acte 1 ou 2 pour payoff en acte 3

### character-arrival

- **Detects**: Protagonist ending in the same internal state as the beginning. No transformation, or transformation that isn't shown through behavior.
- **Works**: The protagonist has arrived somewhere different — internally, not just circumstantially. What they can see, do, or accept now that they couldn't at the beginning. The cost of change is visible.
- **Fails**: The character is the same person at the end. Or they've "changed" but only in narrated declaration, not demonstrated behavior. No visible cost.
- **Fix pattern**: Show the arrival through a scene where the character faces a situation similar to the opening — and responds differently. The contrast IS the transformation.
- **Example**:
  > "Il avait changé. Il le savait. Tout serait différent désormais."
  > → Transformation déclarée, pas montrée — le lecteur doit le VOIR
  > → Montrer un choix concret qui prouve le changement par l'action

### resolution-pacing

- **Detects**: Post-climax sections that are too rushed (reader feels cheated) or too drawn out (reader disengages). The breath after the peak matters.
- **Works**: The resolution gives the reader time to process without losing them. The space between climax and final page is proportional to what needs settling.
- **Fails**: The climax is 3 pages from the end — no room to land. Or the resolution stretches 50 pages with no remaining tension.
- **Fix pattern**: Map what MUST be resolved post-climax. Cut everything else. Give each necessary beat exactly the space it needs — no more.
- **Example**:
  > (40-page epilogue after the climax, with multiple catch-up scenes)
  > → Résolution trop longue — le lecteur a décroché après le climax
  > → Réduire : ne garder que les beats essentiels post-climax

### last-image

- **Detects**: Endings without a strong final image. The reader closes the book with no visual or sensory impression to carry.
- **Works**: The final image compresses the book's meaning into a single frame. It's what the reader carries after closing the book. A visual or sensory impression that resonates.
- **Fails**: The ending is conceptual, abstract, or conversational. No image. Nothing for the reader's eye to hold onto.
- **Fix pattern**: Find the image that embodies the book's central truth. End on a concrete, sensory frame — not a thought, not a statement, but something the reader can see.
- **Example**:
  > "Elle comprit alors que tout avait un sens, finalement."
  > → Abstrait — pas d'image finale, rien de visuel ou sensoriel
  > → Finir sur une image concrète qui porte le sens sans le nommer
