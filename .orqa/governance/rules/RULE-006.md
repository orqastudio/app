---
id: RULE-006
title: Coding Standards
description: "Rust and TypeScript coding standards: formatting, linting, error handling, component patterns, and coverage requirements."
status: active
created: "2026-03-07"
updated: "2026-03-07"
layer: canon
scope: project
---
**Source of Truth:** `.orqa/documentation/development/coding-standards.md`

**READ the full document before writing any code.**

## Rust Standards

- **Formatting**: `rustfmt` on all code, no exceptions
- **Linting**: `clippy` with pedantic and nursery lint groups enabled. Zero warnings in CI.
- **Error handling**: `thiserror` for all error types. Every function returns `Result<T, E>`. NO `unwrap()`, `expect()`, or `panic!()` in production code — only in tests.
- **Types**: All IPC types derive `Serialize`, `Deserialize`, `Debug`, `Clone`. Domain types should be immutable by default.
- **Module organization**: One module per domain concept. Public API via `mod.rs` or `lib.rs`. Keep `main.rs` minimal — it wires things together.
- **Functions**: <=50 lines (domain: 20-30, commands: 30-50, utilities: 10-20). Extract helpers when exceeding limits.
- **Dependencies**: Prefer well-maintained crates. No duplicate functionality. Pin versions in `Cargo.toml`.

## TypeScript / Svelte Standards

- **Svelte version**: Svelte 5 runes ONLY (`$state`, `$derived`, `$effect`, `$props`). No Svelte 4 patterns (no `let:`, no `$:` reactive statements, no `export let` for props).
- **Strict TypeScript**: `strict: true` in `tsconfig.json`. No `any` types. No `@ts-ignore`. No `as unknown as T` casts.
- **Components**: shadcn-svelte as the component library. Use variant props (`size`, `spacing`, `layout`) on shadcn components instead of inline Tailwind overrides. If a class appears 3+ times on a component, add it as a variant.
- **Component purity**: Pages and containers fetch data (call `invoke()`). Display components receive props only. No `invoke()` calls in `$lib/components/`.
- **Store pattern**: Runes-based stores in `.svelte.ts` files. Expose reactive state and actions. Stores call `invoke()`, components read stores.
- **NO emoji in UI** — use Lucide icons for all visual indicators. Emoji only for emotional reactions in conversational text.

## Both Languages

- **Coverage**: 80%+ test coverage. No exceptions without documented justification.
- **No TODO comments**: If something isn't done, it's tracked in TODO.md, not scattered across the codebase. TODO comments in committed code are a build failure.
- **No commented-out code**: Delete it. Git history preserves it.
- **No hardcoded fake data**: See [RULE-020](RULE-020) (no-stubs).
- **MUST use shared components**: See [RULE-024](RULE-024) (reusable-components) for the shared component library.

## Enforcement

Run before every commit:

```bash
make check
```

This single command runs: `fmt-check` + `clippy` + `test-rust` + `check-frontend` + `lint` + `test-frontend`.

A git pre-commit hook (`.githooks/pre-commit`) enforces this automatically. It runs the relevant subset of checks based on which files are staged. **NEVER bypass the hook with `--no-verify`.**

For individual checks, see `.orqa/documentation/development/commands.md` or run `make help`.

## Lint Rule Alignment (NON-NEGOTIABLE)

Coding standards MUST be reflected in automated linting rules. If a standard exists in this document or in `.orqa/documentation/development/coding-standards.md`, there MUST be a corresponding lint rule that enforces it. Conversely, if a lint rule enforces something, that standard MUST be documented.

**When modifying coding standards:**

1. Update this document AND `.orqa/documentation/development/coding-standards.md`
2. Add or update the corresponding ESLint rule in `eslint.config.js` (frontend) or clippy configuration (Rust)
3. Run `make check` to verify the rule works
4. Fix ALL existing violations introduced by the new rule in the same commit
5. Update the pre-commit hook if the enforcement mechanism changes

**When a lint rule catches violations:**

- Pre-existing violations are NOT an excuse to skip or disable the rule
- Fix every violation, even in files you did not modify
- If the violation count is very large (50+), flag it to the user for prioritization — but never silently ignore it

**FORBIDDEN:**

- `// eslint-disable` without a documented justification in the same line
- `#[allow(clippy::...)]` without a documented justification
- Adding a rule to an ignore list instead of fixing the code
- Claiming "this error existed before" as a reason not to fix it

## Related Rules

- [RULE-012](RULE-012) (error-ownership) — *when* to verify (always, before every call)
- [RULE-024](RULE-024) (reusable-components) — *which* components to use (shared library)
- [RULE-029](RULE-029) (testing-standards) — testing patterns and coverage requirements
- [RULE-005](RULE-005) (chunkhound-usage) — use semantic search before creating new code
