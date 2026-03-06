# Meso Rules — qa-consistency

> Scope: meso
> Loaded when: text >= 2 scenes
> Evaluates: Cross-scene continuity — events, injuries, and states persist between scenes
> Does NOT evaluate: within-scene physics, manuscript arcs
> Preferred output: inline or diagnostic

---

## Finding Axes

### timeline

- **Detects**: Chronological contradictions between scenes. Travel times inconsistent with geography. Day/night cycle errors. Events referenced in dialogue not matching when they occurred.
- **Works**: Cause and effect respect chronology. "Three days later" means three days. Travel times match established distances. Day/night cycles are consistent.
- **Fails**: A character leaves on a three-day journey and arrives next scene, which is set "the next morning." Monday is established, and two scenes later it's still Monday but three events have passed.
- **Fix pattern**: Map the timeline explicitly. Adjust temporal markers or insert time-skip beats to restore chronological logic.
- **Example**:
  > Ch.3 "Le voyage prendrait trois jours." Ch.4 "Le lendemain, il arriva à destination."
  > → Timeline : voyage de 3 jours (ch.3), arrivée le lendemain (ch.4)
  > → Corriger : "trois jours plus tard" ou réduire la distance établie

### physical-continuity

- **Detects**: Injuries forgotten, fatigue ignored, physical states resetting between scenes. Bodies without memory.
- **Works**: A broken arm in chapter 3 is still broken in chapter 5. Fatigue from two sleepless days shows. Hunger, thirst, and physical state persist realistically.
- **Fails**: A character badly beaten in one scene shows no sign of injury in the next. A character who hasn't eaten in days performs at full capacity. Wounds heal at plot-convenient speed.
- **Fix pattern**: Track physical state across scenes. Add references to ongoing conditions — even brief mentions maintain continuity.
- **Example**:
  > Ch.5 "La balle lui traversa l'épaule." Ch.7 "Il escalada la falaise sans difficulté."
  > → Blessure : balle dans l'épaule (ch.5), escalade sans gêne (ch.7)
  > → Ajouter la douleur, la limitation, l'effort supplémentaire

### character-location

- **Detects**: Characters appearing in locations they can't have reached. Bilocation. Travel not accounted for. Characters present without having arrived.
- **Works**: If a character left for a journey, they can't appear at the origin without explanation. Characters are where the last scene placed them.
- **Fails**: A character last seen leaving for the capital appears at the village market two scenes later without returning. No one teleports.
- **Fix pattern**: Add the return/travel beat, or adjust the scene order so the character's presence is plausible.
- **Example**:
  > Ch.8 "Il partit pour la capitale." Ch.10 "Il croisa Marie au marché du village."
  > → Localisation : parti ch.8, au village ch.10 sans retour narré
  > → Ajouter le retour ou déplacer la rencontre à la capitale

### season-climate

- **Detects**: Weather and season inconsistent with established calendar and geography. Climate contradicting the setting.
- **Works**: If it's winter in the mountains, it's cold. Harvest means autumn. Daylight hours match latitude and season.
- **Fails**: Snow falls in July (northern hemisphere, realistic setting). Characters wear light clothing in established winter. Daylight at 10 PM in a Paris December.
- **Fix pattern**: Align weather with established calendar and geography. Adjust the description or the timeline.
- **Example**:
  > "En plein mois de janvier" ... "les champs de blé ondulaient sous le soleil estival."
  > → Saison : janvier établi, description estivale
  > → Aligner la description sur l'hiver ou corriger le mois

### ooc-behavior

- **Detects**: Character behavior contradicting established traits, knowledge, or voice without arc justification. Using information they haven't learned. Personality shifts without explanation.
- **Works**: Behavior matches what is ESTABLISHED about the character: their knowledge state, their personality, their voice. Changes are intentional and arc-driven.
- **Fails**: A character who fears heights climbs a tower without acknowledging the fear. A character uses information from a conversation they weren't part of. A shy character is suddenly gregarious.
- **Fix pattern**: Check arc first — is change intentional? If yes, ensure it's shown. If no, align behavior with established character. Flag as query if ambiguous.
- **Example**:
  > (Character afraid of water since ch.1, dives into a lake ch.12 without hesitation or comment)
  > → OOC : peur de l'eau établie, plongeon sans hésitation ni reconnaissance
  > → Si arc : montrer la lutte intérieure. Si erreur : restaurer la peur
