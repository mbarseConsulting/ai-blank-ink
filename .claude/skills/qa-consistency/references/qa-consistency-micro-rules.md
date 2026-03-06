# Micro Rules — qa-consistency

> Scope: micro
> Loaded when: always
> Evaluates: Within-scene continuity — physical reality stays consistent paragraph to paragraph
> Does NOT evaluate: cross-scene tracking, manuscript-level threads
> Preferred output: inline

---

## Finding Axes

### object-tracking

- **Detects**: Items changing position without a beat. Objects appearing in hands that set them down. Weapons dropped in combat firing later. Items disappearing and reappearing.
- **Works**: Every object has a position. Every position change needs a beat. The physical world is tracked.
- **Fails**: A glass set on the table appears in the character's hand without being picked up. A weapon dropped mid-fight is used two paragraphs later.
- **Fix pattern**: Add the missing beat (picking up, moving, putting down) or correct the contradicting reference.
- **Example**:
  > ¶3 "Elle posa le couteau sur la table." ... ¶7 "Elle essuya la lame sur son tablier."
  > → Objet : couteau posé ¶3, utilisé ¶7 sans reprise
  > → Ajouter le geste de reprise entre ¶3 et ¶7

### clothing-continuity

- **Detects**: Characters changing clothes mid-scene without reason. Items removed reappearing on the character. Armor, accessories, disguises inconsistently tracked.
- **Works**: Every visible item on a character persists until explicitly changed. Removal and addition are narrated.
- **Fails**: A coat removed in paragraph 2 is being worn in paragraph 8. A hat mentioned early vanishes without being removed.
- **Fix pattern**: Track the item — either add the removal/addition beat or correct the reference.
- **Example**:
  > ¶2 "Il retira sa veste et la posa sur la chaise." ... ¶9 "Il boutonna sa veste et sortit."
  > → Vêtement : veste retirée ¶2, portée ¶9 sans remise
  > → Ajouter le geste de remettre la veste, ou corriger ¶9

### weather-continuity

- **Detects**: Weather changing without transition within a scene. Rain starting or stopping without note. Temperature, light, and atmospheric conditions breaking continuity.
- **Works**: Rain doesn't stop without transition. Wind doesn't appear and disappear. Atmospheric state persists within a scene unless a time skip or explicit change occurs.
- **Fails**: It's raining at the start of the scene and sunny mid-scene with no transition. The character comments on cold in paragraph 1 and sweats in paragraph 5 indoors.
- **Fix pattern**: Add the transition (weather change beat) or align the references.
- **Example**:
  > ¶1 "La pluie battait les carreaux." ... ¶12 "Le soleil inondait la pièce."
  > → Météo : pluie ¶1, soleil ¶12 sans transition — même scène
  > → Ajouter une transition temporelle ou corriger la lumière ¶12

### spatial-logic

- **Detects**: Characters speaking from rooms they haven't entered. Inconsistent distances within a scene. Characters overhearing what they physically can't. Spatial layout contradictions.
- **Works**: Physical space has rules. If the kitchen is upstairs, it stays upstairs. Characters move through space logically. Acoustics are realistic.
- **Fails**: A character responds to a whispered conversation from across a courtyard. A room described as small suddenly accommodates a crowd. A character appears in a space without entering.
- **Fix pattern**: Add the missing movement beat, correct the spatial reference, or adjust what the character can perceive from their position.
- **Example**:
  > "Marc chuchota à l'oreille de Sophie." ... "De l'autre côté du jardin, Paul répondit : — J'ai entendu."
  > → Espace : Paul entend un chuchotement depuis l'autre côté du jardin
  > → Rapprocher Paul ou changer le mode de communication
