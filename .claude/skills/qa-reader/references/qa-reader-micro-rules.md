# Micro Rules — qa-reader

> Scope: micro
> Loaded when: always
> Evaluates: Paragraph and passage level engagement — the reader's moment-to-moment experience
> Preferred output: inline

---

## Finding Axes

### hook

- **Detects**: First sentence of scene/chapter that fails to create a question, an image, or a voice. Openings that explain, set up context, describe weather, or orient before engaging.
- **Works**: The first sentence is an audition. It creates curiosity, plants an image, or establishes a voice the reader wants to follow. Not a gimmick — a promise.
- **Fails**: The scene opens with exposition, weather description, or neutral context-setting. Nothing pulls the reader forward. The first line could be cut without losing engagement.
- **Fix pattern**: Replace with a line that creates a question, plants a sensory image, or introduces a voice with attitude. Enter the scene at the point of interest.
- **Example**:
  > "Il faisait froid ce matin-là et le ciel était gris."
  > → Ouverture météo — aucune question, aucune tension, aucune voix
  > → Entrer par un geste, un détail étrange, ou une pensée qui accroche

### density

- **Detects**: Passages, paragraphs, or sequences that can be removed without loss to the reading experience. Throat-clearing openings, redundant transitions, scenes that restate what's already established. Evaluated at passage and scene level — not sentence level.
- **Works**: Every passage earns its place. Each paragraph advances the scene, deepens a character, or creates texture the reader needs. Nothing redundant at the structural level.
- **Fails**: A full paragraph establishes something already clear from the previous scene. A transition restates what the reader already knows. A descriptive passage adds atmosphere already conveyed. Remove the passage — nothing changes. It shouldn't exist.
- **Fix pattern**: Cut the passage or merge its essential information into a stronger neighbor. If a scene repeats a beat from a previous scene, one of them goes.
- **Example**:
  > (A full paragraph recapping the argument from the previous scene before the new scene begins)
  > → Passage redondant — le lecteur sait déjà, la scène précédente l'a montré
  > → Couper la récapitulation, entrer directement dans le nouveau beat

### sensory-clarity

- **Detects**: Scenes without sensory grounding. Abstract atmosphere ("the room was tense") instead of concrete detail. Missing physical reality.
- **Works**: The reader can see, hear, feel the scene. Grounded in specific sensory detail. At least two senses per scene anchor. The physical world exists.
- **Fails**: The scene happens in a void. No sound, no texture, no temperature. Characters interact in abstract space. Emotions are named but not felt through the body.
- **Fix pattern**: Add one specific sensory detail that grounds the scene. Anchor through what the POV character perceives first — hierarchy of senses reveals character.
- **Example**:
  > "La tension entre eux était devenue insupportable."
  > → Abstrait — rien de physique, rien que le lecteur puisse sentir
  > → Montrer la tension par un son, un geste, un objet déplacé

### dialogue-effect

- **Detects**: Dialogue without subtext. Characters saying exactly what they mean. No gap between words and intentions. Dialogue that only exchanges information.
- **Works**: Characters say one thing, mean another. Tension in the gap between words and intentions. Distinct voices. Dialogue advances the scene AND reveals character. This axis judges the EFFECT on the reader, not the technical format.
- **Fails**: Characters explain their feelings in dialogue. Two characters agree without friction. Every line delivers information with no undertow. The reader learns but doesn't feel.
- **Fix pattern**: Add what's unsaid. Give one character a hidden agenda. Make the surface conversation about one thing and the real conversation about another.
- **Example**:
  > "— Je suis en colère contre toi. — Je sais, et je suis désolé."
  > → Dialogue frontal — aucun sous-texte, rien entre les lignes
  > → Faire passer la colère par un sujet détourné, un refus, un silence
