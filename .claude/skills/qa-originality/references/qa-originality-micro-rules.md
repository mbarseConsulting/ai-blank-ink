# Micro Rules — qa-originality

> Scope: micro
> Loaded when: always
> Evaluates: Sentence and paragraph level freshness — formulation, images, word choices
> Preferred output: inline

---

## Finding Axes

### formulation-freshness

- **Detects**: Stock metaphors, generic images, off-the-shelf formulations. Also detects fresh, world-anchored images that work.
- **Works**: Every image is specific to the scene, the character, and the world. No metaphor has been used by a thousand other novels. The formulation makes the reader pause and see differently.
- **Fails**: "Son coeur se serra." "Un frisson lui parcourut l'échine." "Le silence pesait." Stock images that belong to every novel and no novel.
- **Fix pattern**: Replace the stock image with one anchored in the specific world, character, and moment. What would THIS character compare it to, given THEIR experience?
- **Example**:
  > "Le temps s'arrêta. Le monde autour d'elle cessa d'exister."
  > → Stock — formulation vue dans des milliers de romans
  > → Ancrer dans le monde : qu'est-ce qui s'arrête concrètement pour CE personnage ?

### linguistic-audacity

- **Detects**: Stylistic risk-taking — neologisms, broken rules, unexpected word choices, inventive constructions. Deliberate departures from convention (not errors).
- **Works**: The risk creates an effect no conventional construction could achieve. A neologism captures something. A broken rule serves meaning. The reader feels the author's confidence.
- **Fails**: The prose is correct but plays it safe. Every sentence follows convention. No surprises in word choice or structure. Competent but timid.
- **Fix pattern**: Identify the moments where conventional language falls short and push — find the word that doesn't exist yet, the construction that breaks the rule to create meaning.
- **Example**:
  > "Elle ressentit une grande tristesse."
  > → Conventionnel, sans risque — la langue ne fait rien que le dictionnaire ne fasse
  > → Chercher l'angle linguistique unique : un mot inventé, un détournement, une image neuve

---

## Verdict Axes

### voice-singularity

- **Assesses**: Whether the prose bears a specific authorial fingerprint at the paragraph level.
- **Works**: Could any author have written this sentence, or does it belong to THIS writer? Distinctive rhythm, vocabulary, angle of perception. Voice is the hardest thing to achieve and the most valuable.
- **Fails**: The prose is competent but anonymous. Any skilled author could have produced these paragraphs. No personality in the language.
- **States**: distinctive / emerging / competent / generic
- **Suggestion pattern**: If emerging → identify the passages where voice breaks through and use them as template for the rest. If competent → what does this author notice that others don't? Find their angle and amplify. If generic → the author needs to find what they're angry about, obsessed with, unable to say conventionally.
