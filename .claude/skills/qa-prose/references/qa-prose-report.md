# qa-prose — Report Format

> Extends: qa-report-template.md (generic formats, rules, suggestion law)
> Editorial role: line editor
> Default output: inline
> Suggestion level: proposed rewrites — the user accepts or refuses each one
> Output language: French content, English column headers. Axis names stay English.

---

## Axis Registry

Craft quality only. All are findings (no verdicts in qa-prose).

| Axis                 | Source      | Default Sev |
| -------------------- | ----------- | ----------- |
| pov-discipline       | craft-rules | 🔴          |
| show-tell            | craft-rules | 🟡          |
| over-explanation     | craft-rules | 🟡          |
| exposition-control   | craft-rules | 🟡          |
| dialogue-craft       | craft-rules | 🔵          |
| description-craft    | craft-rules | 🔵          |
| emotional-repetition | craft-rules | 🟡          |
| sentence-rhythm      | craft-rules | 🔵          |
| word-precision       | craft-rules | 🔵          |

---

## Language Convention

- Column headers: English.
- Cell content (issues, notes, rewrites): French.
- Axis names: English (technical identifiers).
- Section headings in output: English.

---

## Prose-Specific Rules

These extend the generic report rules from qa-report-template.md:

### Rewrite Law

Every finding gets a proposed rewrite. Not a direction — the text. The user decides.

- If you can't write the rewrite, you haven't understood the problem.
- The rewrite preserves the author's substance. It fixes the craft issue and nothing else.
- Never add content the author didn't intend.

---

## Format Overrides

Uses formats from qa-report-template.md with these qa-prose specifics:

### Inline (default)

Line editing is manuscript markup. Each finding is a margin note with a proposed rewrite.

```markdown
## qa-prose — Inline

**¶N** — « [passage cité] »
🔴|🟡|🔵 axis — [problème, ≤15 mots]
→ « [réécriture proposée] »

**¶N** — « [passage cité] »
🟢 [ce qui fonctionne et pourquoi — une phrase]

**Recurring**: [axis] — [count]x — [direction systémique, une phrase]

---

Accepte ou refuse chaque réécriture proposée. Les adaptations sont bienvenues.
```

The proposed rewrite is the actual text, ready to paste. Not a direction.

### Diagnostic (on explicit request)

Header:

```markdown
## qa-prose — Diagnostic

### Context

- **Text**: [longueur, type]
- **Refs provided**: [fiches, docs — ou « aucune »]
- **Scope**: craft-rules
```

Rewrite column in findings table: actual text, not direction.

### Assessment (on explicit request)

Header:

```markdown
## qa-prose — Assessment

### Verdict

[Réponse directe. 2-3 phrases. L'avis honnête du line editor.]
```

Grid states per axis:

- pov-discipline: clean / slips / broken
- show-tell: shows / mixed / tells
- over-explanation: trusts reader / occasionally doubles / systematic
- exposition-control: woven / some blocks / dumps
- dialogue-craft: alive / functional / flat
- description-craft: filtered / mixed / catalog
- emotional-repetition: unique signatures / some reuse / systematic
- sentence-rhythm: varied / mostly uniform / monotone
- word-precision: precise / approximate / generic
