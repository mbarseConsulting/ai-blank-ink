# qa-characters — Report Format

> Extends: qa-report-template.md (generic formats, rules, suggestion law)
> Editorial role: character consultant
> Default output: assessment
> Suggestion level: psychological directions — where the character needs work and why
> Output language: French content, English column headers. Axis names stay English.

---

## Axis Registry

| Axis                      | Source          | Type    | Default Sev |
| ------------------------- | --------------- | ------- | ----------- |
| contradiction-credibility | depth-rules     | finding | 🟡          |
| defense-mechanisms        | depth-rules     | finding | 🟡          |
| monologue-credibility     | depth-rules     | finding | 🟡          |
| pov-knowledge             | depth-rules     | finding | 🔴          |
| want-need-gap             | depth-rules     | verdict | —           |
| wound-coherence           | depth-rules     | verdict | —           |
| inner-lie                 | depth-rules     | verdict | —           |
| relational-patterns       | evolution-rules | finding | 🔵          |
| evolution-credibility     | evolution-rules | finding | 🔴          |
| arc-completion            | evolution-rules | verdict | —           |
| relationship-credibility  | dynamics-rules  | finding | 🟡          |
| character-distinctiveness | dynamics-rules  | finding | 🔴          |
| secondary-depth           | dynamics-rules  | finding | 🔵          |
| cast-balance              | dynamics-rules  | verdict | —           |
| antagonist-logic          | dynamics-rules  | verdict | —           |

---

## Language Convention

- Column headers: English.
- Cell content (issues, notes, suggestions, directions): French.
- Axis names: English (technical identifiers).
- Section headings in output: English.
- First impression, verdicts, closing lines: French.

---

## Format Overrides

Uses formats from qa-report-template.md with these qa-characters specifics:

### Assessment (default)

Header:

```markdown
## qa-characters — Assessment

### Verdict

[Réponse directe en français. 2-3 phrases. Le personnage est crédible ou pas — et pourquoi.]

### Context

- **Character(s)**: [nom(s) évalué(s)]
- **Refs provided**: [fiches personnages, docs — ou « aucune »]
- **Refs missing**: [ce qui aiderait — ou « complet »]
```

Grid organized by character, not by axis. Each character gets their own grid section.

Direction = psychological, not stylistic. The character consultant says WHAT doesn't work psychologically and WHY. Directions reference the character's established psychology (wound, lie, defense).

### Diagnostic (on explicit request)

Header:

```markdown
## qa-characters — Diagnostic

### Context

- **Text**: [longueur, type]
- **Character(s)**: [nom(s) évalué(s)]
- **Refs provided**: [fiches personnages, docs — ou « aucune »]
- **Refs missing**: [ce qui aiderait — ou « complet »]
- **Scope**: [rules files chargés]
```

Findings grouped by character, then by root cause.

### Inline (on explicit request)

Header:

```markdown
## qa-characters — Inline
```

Direction = psychological, not stylistic. The author decides how to implement in prose.
