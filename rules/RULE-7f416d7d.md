---
id: "RULE-7f416d7d"
title: "Tooling Ecosystem Management"
description: "OrqaStudio manages linter configuration to match documented standards. Code quality enforcement belongs in linters, not in regex matching."
status: "active"
created: "2026-03-11"
updated: "2026-03-12"
enforcement:
  - "event: lint"
  - "event: lint"
  - "event: lint"
  - "event: lint"
  - "event: lint"
relationships:
  - target: "AD-aa6b409a"
    type: "enforces"
---
OrqaStudio delegates code quality enforcement to the appropriate linting tools. The
enforcement engine does NOT regex-match patterns that linters already catch. Instead,
OrqaStudio ensures the tooling ecosystem is configured to match documented standards.

## The Chain

```
Documented standard (RULE-b49142be, knowledge)
  → Linter configuration (clippy.toml, eslint.config.js)
  → Hook trigger (.githooks/pre-commit, make check)
  → Violation reported to developer
```

OrqaStudio's role is managing the full chain, not replicating any step.

## Linter-to-Standard Mapping

### Rust (clippy pedantic)

| Standard | Linter Rule | Config |
|----------|-----------|--------|
| No unwrap/expect/panic | `clippy::unwrap_used`, `clippy::expect_used`, `clippy::panic` | Enabled via clippy pedantic |
| Function size limits | `clippy::too_many_lines` | Configured per module guidelines |
| Zero warnings | `-D warnings` flag | Passed in `make lint-backend` |
| rustfmt formatting | `cargo fmt --check` | Default rustfmt config |

### TypeScript/Svelte (ESLint + svelte-check)

| Standard | Linter Rule | Config |
|----------|-----------|--------|
| No `any` types | `@typescript-eslint/no-explicit-any` | ESLint strict TS config |
| No `@ts-ignore` | `@typescript-eslint/ban-ts-comment` | ESLint strict TS config |
| No Svelte 4 patterns | svelte-check strict mode | Svelte 5 migration rules |
| Strict TypeScript | `tsconfig.json` strict: true | TypeScript compiler config |

### Pre-commit Hook Chain

| File Types Staged | Checks Run |
|------------------|-----------|
| `.rs`, `Cargo.*` | rustfmt, clippy, cargo test |
| `.svelte`, `.ts`, `.js`, `.css`, `.html` | svelte-check, ESLint, Vitest |
| `.orqa/**/*.md` | Schema validation, artifact auto-linking |

## Lint Event Entries

Enforcement entries with `event: lint` are declarative — they document which linter
rule enforces which standard. They don't execute anything. They exist for traceability:
every documented standard should map to either a linter rule (lint event) or a process
gate/knowledge injection (file/inject event).

## When OrqaStudio Should NOT Regex-Match

If a pattern is already caught by a configured linter:
- Don't add a `file` enforcement entry that regex-matches the same pattern
- Instead, add a `lint` entry documenting the delegation
- The knowledge artifact for that area should describe how to fix the violation

If a pattern is NOT caught by any linter:
- Consider adding a linter rule first
- Only use `file` enforcement as a last resort for patterns no linter covers
- `bash` enforcement for command-line safety (e.g., `--no-verify`) is appropriate since no linter covers shell commands

## FORBIDDEN

- Regex-matching code patterns that configured linters already catch
- Adding `file` enforcement for patterns that clippy or ESLint enforce
- Disabling linter rules instead of fixing violations
- Standards without corresponding linter configuration

## Related Rules

- [RULE-b49142be](RULE-b49142be) (coding-standards) — the standards this rule maps to linters
- [RULE-f9d0279c](RULE-f9d0279c) (knowledge-injection) — knowledge injection complements linter delegation
- [RULE-57ccb4a3](RULE-57ccb4a3) (error-ownership) — pre-commit hook enforcement
