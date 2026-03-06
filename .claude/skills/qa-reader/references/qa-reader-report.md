# qa-reader — Report Format

> Extends: qa-report-template.md (generic formats, rules, suggestion law)
> Editorial role: developmental editor
> Default output: diagnostic (standard mode) / assessment (bookends mode)
> Suggestion level: structural directions — not rewrites, but where to push and why
> Output language: French content, English column headers. Axis names stay English.

---

## Axis Registry

### Standard Mode — Always loaded

| Axis            | Source      | Type    | Default Sev |
| --------------- | ----------- | ------- | ----------- |
| hook            | micro-rules | finding | 🟡          |
| density         | micro-rules | finding | 🟡          |
| sensory-clarity | micro-rules | finding | 🟡          |
| dialogue-effect | micro-rules | finding | 🔵          |
| sentence-rhythm | micro-rules | finding | 🔵          |
| word-precision  | micro-rules | finding | 🔵          |

### Standard Mode — Loaded per scope

| Axis               | Source        | Type    | Default Sev | Loaded when       |
| ------------------ | ------------- | ------- | ----------- | ----------------- |
| tension            | scene-rules   | finding | 🔴          | >= 1 scene        |
| scene-arc          | scene-rules   | finding | 🔴          | >= 1 scene        |
| scene-economy      | scene-rules   | finding | 🟡          | >= 1 scene        |
| microtension       | scene-rules   | finding | 🔵          | >= 1 scene        |
| character-agency   | scene-rules   | finding | 🟡          | >= 1 scene        |
| pacing             | scene-rules   | finding | 🟡          | >= 1 scene        |
| pov-strategy       | scene-rules   | finding | 🟡          | >= 1 scene        |
| temporal-effect    | scene-rules   | finding | 🟡          | >= 1 scene        |
| chapter-ending     | chapter-rules | finding | 🟡          | >= 1 chapter      |
| emotional-truth    | chapter-rules | finding | 🔴          | >= 1 chapter      |
| information-reveal | chapter-rules | finding | 🟡          | >= 1 chapter      |
| necessity          | chapter-rules | verdict | —           | >= 1 chapter      |
| tonal-consistency  | macro-rules   | finding | 🟡          | multiple chapters |
| narrative-arc      | macro-rules   | verdict | —           | multiple chapters |
| thematic-resonance | macro-rules   | verdict | —           | multiple chapters |
| global-engagement  | macro-rules   | verdict | —           | multiple chapters |

### Bookends Mode

| Axis                     | Source        | Type    | Default Sev | Loaded when           |
| ------------------------ | ------------- | ------- | ----------- | --------------------- |
| first-sentence-impact    | opening-rules | finding | 🔴          | text includes opening |
| opening-contract         | opening-rules | finding | 🔴          | text includes opening |
| entry-orientation        | opening-rules | finding | 🟡          | text includes opening |
| character-introduction   | opening-rules | finding | 🟡          | text includes opening |
| world-entry              | opening-rules | finding | 🟡          | text includes opening |
| information-control      | opening-rules | finding | 🟡          | text includes opening |
| hook-architecture        | opening-rules | finding | 🔴          | text includes opening |
| final-sentence-resonance | closing-rules | finding | 🟡          | text includes closing |
| emotional-precision      | closing-rules | finding | 🔴          | text includes closing |
| closure-openness         | closing-rules | finding | 🟡          | text includes closing |
| climax-payoff            | closing-rules | finding | 🔴          | text includes closing |
| character-arrival        | closing-rules | finding | 🟡          | text includes closing |
| resolution-pacing        | closing-rules | finding | 🟡          | text includes closing |
| last-image               | closing-rules | finding | 🔵          | text includes closing |
| narrative-promise        | promise-rules | verdict | —           | opening + closing     |
| genre-contract           | promise-rules | verdict | —           | opening + closing     |
| thematic-arc             | promise-rules | verdict | —           | opening + closing     |
| promise-fulfillment      | promise-rules | verdict | —           | opening + closing     |
| arc-resolution           | promise-rules | verdict | —           | opening + closing     |
| surprise-inevitability   | promise-rules | verdict | —           | opening + closing     |

---

## Language Convention

- Column headers: English.
- Cell content (issues, notes, suggestions, directions): French.
- Axis names: English (technical identifiers).
- Section headings in output: English.
- First impression, verdicts, closing lines: French.

---

## Format Overrides

Uses formats from qa-report-template.md with these qa-reader specifics:

### Diagnostic (Standard Mode — default)

Header:

```markdown
## qa-reader — Diagnostic

### Context

- **Text** : [longueur, type]
- **Refs provided** : [fiches, docs — ou « aucune »]
- **Refs missing** : [ce qui aiderait — ou « complet »]
- **Scope** : [rules files chargés selon le texte]
- **Mode** : standard
- **Register** : [if set at init]
```

### Assessment (Bookends Mode — default)

Header:

```markdown
## qa-reader — Assessment (Bookends)

### Verdict

[Réponse directe en français. 2-3 phrases. L'ouverture/la clôture fonctionne ou pas — et pourquoi.]

### Context

- **Refs provided** : [fiches, docs — ou « aucune »]
- **Refs missing** : [ce qui aiderait — ou « complet »]
```

### Inline (on explicit user request)

Header:

```markdown
## qa-reader — Inline
```

Direction = structural, not rewrite. The dev editor says WHERE to push, not HOW to rewrite. The author decides implementation.
