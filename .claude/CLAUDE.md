## Symlink

Le dossier `.claude/` de ce projet est un **symlink** vers le `.claude/` du projet **ai-blank-ink** :

```
.claude -> ../../apps/ai-studio/ai-blank-ink/.claude
```

Toute modification dans `.claude/` affecte directement le projet source `ai-blank-ink`.

## Structure

```
.claude/                 # symlink -> ai-blank-ink/.claude
  skills/                # Skills (SKILL.md + agents/ + references/)
  agents/                # Agents (.md)
  output-styles/         # Output styles
  CLAUDE.md
  hooks.json
  settings.local.json
README.md
.gitignore
```

## Sources

Skills et agents proviennent de 3 projets, assemblés dans **ai-blank-ink** :

- **ai-write-ink** — Skills d'écriture fiction (14 skills)
- **ai-forge-ink** — Skills visuels et worldbuilding (3 skills)
- **ai-crafter-toolkit** — Skills utilitaires (6 skills)
