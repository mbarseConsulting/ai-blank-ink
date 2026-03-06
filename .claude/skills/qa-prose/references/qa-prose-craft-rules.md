# Craft Rules — qa-prose

> Scope: micro
> Loaded when: always
> Evaluates: Writing craft at the sentence and paragraph level
> Does NOT evaluate: AI patterns, French language quality
> Preferred output: inline

---

## Finding Axes

### pov-discipline

- **Detects**: Head-hopping within a scene, information the POV character can't access, unauthorized omniscience, perceptions not filtered through the established POV.
- **Works**: Every thought, sensation, and judgment belongs to one character per scene. The reader inhabits one pair of eyes.
- **Fails**: The narrator suddenly knows what another character thinks or feels without behavioral evidence. Information appears that the POV character has no way of knowing.
- **Fix pattern**: Rewrite through POV character's senses — what they see, hear, infer. Never state another character's interior directly.
- **Example**:
  > "Elle sourit, soulagée de le voir partir. Il sentit une pointe de culpabilité."
  > → Two POVs in consecutive sentences — head-hopping
  > → Ancrer dans un seul POV : montrer le soulagement PAR le comportement visible

### show-tell

- **Detects**: Emotions named by abstract labels instead of shown through action, sensation, or behavior. The narrator diagnoses instead of rendering.
- **Works**: The reader INFERS the emotion from concrete physical or behavioral detail. No label needed — the feeling is self-evident.
- **Fails**: The text tells the reader what to feel: "elle était triste", "il ressentit de la colère", "une profonde mélancolie l'envahit."
- **Fix pattern**: Replace the abstract label with a physical manifestation, a behavior, or a gesture specific to this character in this moment.
- **Example**:
  > "Elle ressentit une profonde tristesse en regardant la maison vide."
  > → Emotion nommée, pas montrée — diagnostic, pas écriture
  > → Montrer : ce qu'elle fait, ce qu'elle voit, comment son corps réagit

### over-explanation

- **Detects**: Gesture AND narrated emotion for the same beat. Dialogue AND incise explaining the same intention. Subtext killed by explicit statement.
- **Works**: One vector per emotional beat. The gesture OR the label. The dialogue OR the explanation. Never both. Subtext breathes.
- **Fails**: The text shows the emotion through action, then immediately names it. Double-encoding: the reader understood from the gesture, and the narrator explains anyway.
- **Fix pattern**: Cut the weaker of the two encodings. Keep the one that carries more texture.
- **Example**:
  > "Elle détourna le regard, incapable de soutenir ses yeux. La honte l'envahissait."
  > → Le geste dit déjà la honte — la deuxième phrase est redondante
  > → Supprimer "La honte l'envahissait" — le geste suffit

### exposition-control

- **Detects**: Backstory dumped in blocks, narrator pausing the scene to lecture, "as you know" dialogue, information delivered outside of action or character need.
- **Works**: Backstory woven into action, dialogue, and observation. The iceberg principle: show 10%, imply the rest. Information arrives when a character needs it, not when the author wants to explain it.
- **Fails**: The narrative stops so the narrator can explain history, lore, or context in a standalone paragraph. Dialogue becomes a vehicle for information the characters already know.
- **Fix pattern**: Distribute exposition across action and dialogue. Deliver information at the moment of dramatic need, not before.
- **Example**:
  > "Le château avait été construit en 1453 par le duc de Valmont, qui avait voulu en faire une forteresse imprenable après les guerres de..."
  > → Bloc d'exposition — la scène s'arrête pour une leçon d'histoire
  > → Intégrer dans l'action : un détail architectural remarqué par le personnage

### dialogue-craft

- **Detects**: Tags without action beats, info dumps disguised as conversation, missing subtext, non-standard French dialogue formatting.
- **Works**: French dialogue conventions respected (tirets cadratins). Incises carry action beats, not just attribution. Tags only for clarity. Dialogue reveals character through what's said AND what's avoided.
- **Fails**: "Dit-il" chains without physical context. Characters explain things to each other that both already know. No gap between what's said and what's meant.
- **Fix pattern**: Replace empty tags with action beats. Cut information both characters know. Add subtext through avoidance, deflection, or mismatch.
- **Example**:
  > "— Je ne comprends pas, dit-il. — Moi non plus, dit-elle. — Qu'est-ce qu'on fait ? dit-il."
  > → Chaîne de « dit-il/elle » sans action — dialogue désincarné
  > → Remplacer les tags par des gestes qui révèlent l'état intérieur

### description-craft

- **Detects**: Catalog descriptions disconnected from POV and action. Paused scenes for inventory. Generic sensory details not filtered through character perception.
- **Works**: Descriptions grounded in POV character's perception and integrated into action. The character NOTICES what matters to them, misses what doesn't. Sensory hierarchy reveals character.
- **Fails**: The narrator lists objects, colors, and dimensions like a camera pan. The character sees everything equally. The description interrupts the scene instead of serving it.
- **Fix pattern**: Filter description through character psychology — what they notice first reveals who they are. Integrate into movement and action.
- **Example**:
  > "La pièce contenait un canapé bleu, une table basse en bois, trois étagères remplies de livres et un tapis persan usé."
  > → Inventaire — aucun filtre par le personnage, aucune intégration à l'action
  > → Le personnage remarque UN détail qui lui parle — le reste en périphérie
