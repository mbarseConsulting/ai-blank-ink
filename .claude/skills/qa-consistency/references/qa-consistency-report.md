# qa-consistency — Report Format

> Editorial role: continuity editor
> Default output: inline
> Suggestion level: correction proposals with dual-citation (establishment + violation)
> Output language: French content, English column headers. Axis names and severity codes stay English.

---

## Severity

| Code | Level | Meaning |
|------|-------|---------|
| `BLK` | Blocking | Must fix. The fictional world is broken. |
| `MAJ` | Major | Should fix. Continuity is weakened. |
| `SUG` | Suggestion | Could tighten. Author decides. |

---

## Axis Registry

Every axis from all rules files. All are findings (no verdicts in qa-consistency).

| Axis | Source | Default Sev |
|------|--------|-------------|
| object-tracking | micro-rules | BLK |
| clothing-continuity | micro-rules | MAJ |
| weather-continuity | micro-rules | MAJ |
| spatial-logic | micro-rules | BLK |
| timeline | meso-rules | BLK |
| physical-continuity | meso-rules | BLK |
| character-location | meso-rules | BLK |
| season-climate | meso-rules | MAJ |
| ooc-behavior | meso-rules | MAJ |
| arc-tracking | macro-rules | BLK |
| lore-consistency | macro-rules | BLK |
| worldbuilding-coherence | macro-rules | MAJ |
| ages-and-aging | macro-rules | BLK |
| fact-checking | macro-rules | MAJ |

---

## Default Format: Inline

The continuity editor flags errors passage by passage. Every finding cites TWO passages: the establishment (where the fact was set) and the violation (where it breaks). The user accepts or refuses each correction.

### Structure

```markdown
## qa-consistency — Inline

[Each finding, in text order by violation location:]

**¶N** — « [passage de la violation] »
SEV axis — [problème, ≤15 mots]
← ¶M « [passage de l'établissement] »
→ [correction proposée]

[Strengths worth noting:]

**¶N** — « [passage cité] »
✓ [continuité bien tenue — une phrase]

[Patterns, only if 3+ of same issue:]

**Recurring** : [axis] — [count]x — [direction systémique]

---

Accepte ou refuse chaque correction proposée. En cas de doute, signalé comme query.
```

### Finding Format

Each finding has four lines:

1. **Location + violation passage** — where it breaks
2. **Severity + axis + problem** — what's inconsistent
3. **← establishment passage** — where the fact was set (dual-citation)
4. **→ correction** — proposed fix

The dual-citation is the continuity editor's signature. No finding without both sides.

**BLK example** (object-tracking):
```
**¶7** — « Elle essuya la lame sur son tablier. »
BLK object-tracking — couteau posé ¶3, utilisé ¶7 sans reprise
← ¶3 « Elle posa le couteau sur la table. »
→ Ajouter entre ¶3 et ¶7 : geste de reprise du couteau
```

**BLK example** (timeline):
```
**ch.4 ¶1** — « Le lendemain, il arriva à destination. »
BLK timeline — voyage de 3 jours, arrivée le lendemain
← ch.3 ¶12 « Le voyage prendrait trois jours. »
→ « Trois jours plus tard, il arriva à destination. »
```

**BLK example** (character-location):
```
**ch.10 ¶5** — « Il croisa Marie au marché du village. »
BLK character-location — parti pour la capitale ch.8, au village ch.10 sans retour
← ch.8 ¶15 « Il partit pour la capitale. »
→ Ajouter le retour entre ch.8 et ch.10, ou déplacer la rencontre à la capitale
```

**MAJ example** (ooc-behavior):
```
**ch.12 ¶8** — « Il plongea sans hésiter dans le lac. »
MAJ ooc-behavior — peur de l'eau établie, plongeon sans reconnaissance
← ch.1 ¶4 « Depuis l'accident, il ne s'approchait plus de l'eau. »
→ Si arc : montrer la lutte intérieure. Si erreur : restaurer l'hésitation. Query si ambiguë.
```

### Strength Format

```
**ch.5-12** — blessure à l'épaule
✓ Continuité physique bien tenue — la douleur est mentionnée à chaque effort sur 7 chapitres
```

### Query Format

When a contradiction could be intentional (unreliable narrator, character growth):

```
**¶8** — « Elle sourit à l'inconnu. »
? ooc-behavior — méfiance établie ch.1, confiance immédiate ici
← ch.1 ¶6 « Elle ne faisait confiance à personne. »
→ Query : arc intentionnel ou oubli ? Si arc, montrer le chemin parcouru.
```

### Closing Line

```
---

Accepte ou refuse chaque correction proposée. En cas de doute, signalé comme query.
```

---

## Alternative: Diagnostic

On explicit user request. Full continuity report with tracking tables.

```markdown
## qa-consistency — Diagnostic

### Context
- **Text** : [longueur, type]
- **Refs** : [fiches, timeline, carte — ou « aucune »]
- **Scope** : [rules files chargés]

### First Impression
[2-3 phrases en français. La cohérence du monde tient ou pas — et les failles majeures.]

### Findings

| # | Violation | Establishment | Axis | Sev | Issue | Fix |
|---|-----------|---------------|------|-----|-------|-----|
| 1 | ch.4 ¶1 « arriva le lendemain » | ch.3 ¶12 « trois jours » | meso:timeline | BLK | [en français] | [en français] |

> Two-passage citation is mandatory. Violation AND establishment.

### Patterns

| Pattern | Count | Examples | Sev | Direction |
|---------|-------|----------|-----|-----------|
| [axis] | N | résumé des cas | MAJ | [direction systémique, en français] |

### Strengths

| Element | Scope | Mechanism |
|---------|-------|-----------|
| [élément suivi] | [chapitres] | [comment l'auteur maintient la cohérence] |

### Queries

| # | Loc | Contradiction | Possible intent | Recommendation |
|---|-----|---------------|-----------------|----------------|
| 1 | ch.12 | [contradiction] | [intention possible] | [recommandation, en français] |

### Priority Fixes
1. #[N] — [action à plus fort impact, en français]

> Max 5. Impact order.
```

---

## Alternative: Assessment

On explicit user request ("est-ce que mon monde est cohérent ?").

```markdown
## qa-consistency — Assessment

### Verdict
[Réponse directe en français. 2-3 phrases. Le monde tient ou pas.]

### Grid

| Axis | State | Note | Suggestion |
|------|-------|------|------------|
| object-tracking | tracked / some gaps / broken | [en français] | [en français] |
| clothing-continuity | tracked / some gaps / broken | [en français] | [en français] |
| weather-continuity | tracked / some gaps / broken | [en français] | [en français] |
| spatial-logic | consistent / some errors / broken | [en français] | [en français] |
| timeline | solid / some gaps / broken | [en français] | [en français] |
| physical-continuity | tracked / some gaps / forgotten | [en français] | [en français] |
| character-location | tracked / some gaps / teleporting | [en français] | [en français] |
| season-climate | consistent / some errors / contradictory | [en français] | [en français] |
| ooc-behavior | consistent / some slips / contradictory | [en français] | [en français] |
| arc-tracking | complete / some threads lost / abandoned | [en français] | [en français] |
| lore-consistency | solid / some bends / broken | [en français] | [en français] |
| worldbuilding-coherence | lived-in / backdrop / wallpaper | [en français] | [en français] |
| ages-and-aging | tracked / some errors / broken | [en français] | [en français] |
| fact-checking | accurate / some errors / anachronistic | [en français] | [en français] |

### Key Issues
[Dual-cited. En français.]

### Strengths
[Ce qui est bien tenu. En français.]
```

---

## Report Rules

### Dual-Citation Law
Every finding cites TWO passages: establishment and violation. No exception.
- If you can't find the establishment, the violation may not be one.
- If the establishment is implicit (genre convention, real-world fact), state it explicitly.

### Query Protocol
When a contradiction could be intentional:
- Flag as `?` instead of severity code.
- State the possible intent.
- Recommend: clarify, or if intentional, show the path.

### Language
- Column headers: English.
- Cell content: French.
- Axis names: English.
- Severity codes: English (BLK/MAJ/SUG).
- Section headings in output: English.

### Citation
1. Every finding quotes TWO passages.
2. Name what contradicts what — precisely.
3. One fix per finding.

### Patterns
1. 3+ same axis = pattern.
2. Max 3 representative examples.
3. One systemic direction.

### Concision
- **Passage**: max 10 quoted words per citation.
- **Issue**: max 15 words.
- **Fix**: max 15 words (or actual corrected text if short).
- **Axis ref**: `file:axis` in diagnostic, `axis` in inline.

### Severity Adjustment
Default severity from the registry. Adjust per context:
- A minor clothing inconsistency in a non-visual scene → SUG, not MAJ.
- A timeline error affecting plot logic → BLK regardless of default.
- Context governs. The default is a starting point.
