# Contributing

## Branch Naming Convention

All branches must follow this format:

```
INK-<type>-<description-in-kebab-case>
```

### Types

| Type       | Usage                             |
| ---------- | --------------------------------- |
| `feat`     | New feature                       |
| `fix`      | Bug fix                           |
| `docs`     | Documentation only                |
| `refactor` | Code refactoring                  |
| `test`     | Adding or updating tests          |
| `chore`    | Maintenance, dependencies, config |

### Examples

```
INK-feat-user-authentication
INK-fix-login-timeout
INK-docs-api-reference
INK-refactor-skill-loader
INK-test-hook-validation
INK-chore-update-dependencies
```

### Rules

- Prefix `INK-` is mandatory
- Use lowercase only
- Use hyphens to separate words (kebab-case)
- Keep descriptions concise but descriptive
- No special characters except hyphens

### Protected Branches

- `main` - Production, requires PR
- `develop` - Integration, requires PR

## Setup

Install git hooks after cloning:

```bash
./scripts/install-hooks.sh
```

This enables automatic branch name validation before push.
