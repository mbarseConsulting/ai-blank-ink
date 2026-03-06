# Opening Rules — qa-reader

> Scope: bookends
> Loaded when: mode = bookends, text includes opening
> Evaluates: First sentence through first chapter — the pages where most manuscripts are rejected
> Preferred output: inline or diagnostic

---

## Finding Axes

### first-sentence-impact

- **Detects**: First sentences that explain, describe weather, or set up context instead of creating a question, image, or voice.
- **Works**: The first sentence is an audition. It creates curiosity, an image, or a voice. The reader decides in this line whether to trust the author.
- **Fails**: The first sentence is neutral, informational, or decorative. It could be removed without loss. It doesn't make the reader need the second sentence.
- **Fix pattern**: Replace with a line that creates an immediate question, plants a vivid image, or establishes a voice with attitude. No setup — engagement.
- **Example**:
  > "L'histoire que je vais vous raconter commence un mardi de novembre."
  > → Annonce — on dit au lecteur qu'on va raconter au lieu de raconter
  > → Commencer dans l'action ou la perception — pas dans le métadiscours

### opening-contract

- **Detects**: First paragraphs that fail to establish the reading contract: POV, tone, register, world, the author's relationship with language. Contracts broken later.
- **Works**: The first paragraph tells the reader exactly what kind of book this is. Every element of the contract — voice, register, distance, world — is sustained throughout.
- **Fails**: The opening promises literary intimacy but the book delivers action-adventure. Or the opening is lyrical but the prose becomes flat by chapter 2. The contract is set and then violated.
- **Fix pattern**: Align the opening with what the book actually delivers. If the book is a thriller, the opening should FEEL like a thriller. The contract must be honest.
- **Example**:
  > (Opening in intimate first-person present, then switching to distant third-person past by chapter 3)
  > → Contrat rompu — le registre de l'ouverture ne correspond pas à la suite
  > → Aligner l'ouverture sur le registre dominant, ou justifier la transition

### entry-orientation

- **Detects**: Readers lost on the first page. Missing answers to: who, where, when, why care. Confusion that isn't compensated by sensory grounding or intrigue.
- **Works**: Within the first page: the reader knows (or has clues about) who they're following, where they are, when this is, and why it matters. Not all explicit — but the reader isn't lost.
- **Fails**: Three pages in and the reader has no anchor. No character, no setting, no time. In medias res without the sensory grounding that compensates for narrative mystery.
- **Fix pattern**: Anchor one element immediately — a character with a name and a physical action, a specific setting, a clear temporal marker. Mystery is fine. Confusion is not.
- **Example**:
  > "Les choses avaient changé. Tout était différent maintenant. On ne pouvait plus revenir en arrière."
  > → Trois phrases abstraites — aucune orientation, aucun ancrage
  > → Un personnage, un lieu, un geste concret dans la première phrase

### character-introduction

- **Detects**: Protagonist introduced through description rather than action. Physical portrait before the reader has a reason to care. Character described instead of revealed.
- **Works**: The reader meets someone they want to FOLLOW — not like, follow. Curiosity, concern, recognition, fascination. The character DOES something that reveals who they are.
- **Fails**: Two paragraphs of physical description. The character is presented from the outside. The reader knows what they look like but not who they are.
- **Fix pattern**: Introduce through action, choice, or reaction. What the character DOES in the first scene tells the reader everything a physical description can't.
- **Example**:
  > "Marie avait les cheveux bruns, les yeux verts, et un visage fin encadré de taches de rousseur."
  > → Portrait physique — le lecteur ne sait pas QUI elle est
  > → Montrer Marie en train d'agir — son premier geste la définit

### world-entry

- **Detects**: World introduction that stalls the story. Encyclopedia entries disguised as opening. Too much context, or too little grounding.
- **Works**: Enough context to orient, not so much it stalls. Details are specific and lived-in, not encyclopedic. The reader enters a world, not a briefing. The iceberg principle: the world exists beyond what's shown.
- **Fails**: The first three pages explain the political system, the magic rules, or the historical context before anything happens. Or: a scene occurs in a total void with no world.
- **Fix pattern**: Enter the world through a character's perception. One specific, lived-in detail is worth more than a page of context. Show the world in use, not in explanation.
- **Example**:
  > "Le royaume de Valdris était gouverné depuis trois siècles par la dynastie des Haelmar, qui avait instauré un système de..."
  > → Exposition de worldbuilding — briefing, pas entrée dans un monde
  > → Un personnage qui vit dans ce monde — ce qu'il voit, subit, contourne

### information-control

- **Detects**: Imbalance between mystery and orientation. Too much withheld (reader gives up) or too much given (reader bored). Curiosity not sustained.
- **Works**: What's withheld creates curiosity. What's given creates grounding. The balance sustains the reader: enough to orient, enough mystery to pull forward.
- **Fails**: Everything is explained — no questions to drive reading. Or nothing is explained — the reader has no handhold and gives up.
- **Fix pattern**: For each piece of information: does revealing it create engagement (ground) or kill it (spoil)? Does withholding it create curiosity (mystery) or frustration (confusion)?
- **Example**:
  > (Opening that explains the character's backstory, motivation, and goal in the first two pages)
  > → Tout est donné — aucune question ouverte, rien à découvrir
  > → Retenir le pourquoi, montrer le quoi — le lecteur veut comprendre

### hook-architecture

- **Detects**: Chapter 1 ending that doesn't create a need for chapter 2. Clean resolution, summary, or fade-out at the end of the opening chapter.
- **Works**: The end of chapter 1 makes the reader NEED chapter 2. Not want — NEED. A question unanswered, a promise half-made, a door opened onto something impossible to walk away from.
- **Fails**: Chapter 1 ends with the character going to bed, a neat summary, or a resolved emotional beat. The reader can close the book here without regret.
- **Fix pattern**: End chapter 1 on an open beat: a revelation that changes everything, a question that demands an answer, or a decision that creates irreversible consequences.
- **Example**:
  > "Satisfaite de sa première journée, elle rangea ses affaires et se prépara pour le lendemain."
  > → Résolution complète — aucune raison de tourner la page
  > → Finir sur l'élément qui rend le lendemain dangereux ou irrésistible
