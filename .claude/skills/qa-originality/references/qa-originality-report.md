# qa-originality — Report Format

> Extends: qa-report-template.md (generic formats, rules, suggestion law)
> Editorial role: editorial scout
> Default output: assessment
> Suggestion level: positioning directions — where the work is derivative and where it's singular
> Output language: French content, English column headers. Axis names stay English.

---

## Axis Registry

| Axis                  | Source      | Type    | Default Sev |
| --------------------- | ----------- | ------- | ----------- |
| formulation-freshness | micro-rules | finding | 🔵          |
| linguistic-audacity   | micro-rules | finding | 🔵          |
| voice-singularity     | micro-rules | verdict | —           |
| situation-freshness   | meso-rules  | finding | 🔵          |
| trope-handling        | meso-rules  | finding | 🔵          |
| narrative-audacity    | meso-rules  | finding | 🔵          |
| scene-coherence       | meso-rules  | verdict | —           |
| concept-originality   | macro-rules | verdict | —           |
| arc-originality       | macro-rules | verdict | —           |
| ambition              | macro-rules | verdict | —           |
| artistic-identity     | macro-rules | verdict | —           |
| maturity              | macro-rules | verdict | —           |

---

## Language Convention

- Column headers: English.
- Cell content (issues, notes, suggestions, directions): French.
- Axis names: English (technical identifiers).
- Section headings in output: English.
- First impression, verdicts, closing lines: French.

---

## Originality-Specific Rules

These extend the generic report rules from qa-report-template.md:

### Source Law

Every finding of déjà-vu NAMES the source: the convention, the trope, the author, the genre expectation. Not just "cliché" — FROM WHERE.

### Singularity Law

Every strength names WHY it's original — the mechanism that makes it irreplaceable.

---

## Format Overrides

Uses formats from qa-report-template.md with these qa-originality specifics:

### Assessment (default)

Header:

```markdown
## qa-originality — Assessment

### Verdict

[Réponse directe en français. 2-3 phrases. Ce texte est-il singulier — et pourquoi, ou pourquoi pas.]

### Context

- **Refs provided**: [fiches, docs — ou « aucune »]
- **Refs missing**: [ce qui aiderait — ou « complet »]
```

Grid states per axis:

- formulation-freshness: fresh / mixed / stock
- linguistic-audacity: daring / competent / timid
- voice-singularity: distinctive / emerging / competent / generic
- situation-freshness: surprising / familiar-with-swerve / stock
- trope-handling: transcended / subverted / followed / unaware
- narrative-audacity: bold / some risks / safe
- scene-coherence: unified / mostly coherent / assembled / fractured
- concept-originality: genuinely new / fresh combination / familiar with texture / derivative
- arc-originality: surprising-inevitable / some surprises / predictable / formulaic
- ambition: ambitious / competent / safe / timid
- artistic-identity: irreplaceable / memorable / competent / interchangeable
- maturity: masterful / confident / developing / uncontrolled

Issue column must NAME the source: quel trope, quelle convention, quel précédent.
Strengths must name WHY it's irreplaceable, not just "good."

### Diagnostic (on explicit request)

Header:

```markdown
## qa-originality — Diagnostic

### Context

- **Text**: [longueur, type]
- **Refs provided**: [fiches, docs — ou « aucune »]
- **Refs missing**: [ce qui aiderait — ou « complet »]
- **Scope**: [rules files chargés]

### First Impression

[2-3 phrases en français. Première réaction de scout : ce texte se distingue ou pas.]
```

### Inline (on explicit request)

Header:

```markdown
## qa-originality — Inline
```

Each finding names the source of the déjà-vu. Each direction points toward singularity.
