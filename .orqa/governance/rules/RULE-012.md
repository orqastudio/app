---
id: RULE-012
title: Error Ownership
description: All errors are your responsibility. Pre-existing errors must be fixed. Never skip or ignore failures.
status: active
created: "2026-03-07"
updated: "2026-03-07"
layer: canon
scope: system
---
**Source of Truth:** `TODO.md` -> "HOW TO WORK" section

## Rule (NON-NEGOTIABLE)

**ALL errors are YOUR responsibility. No exceptions.**

- Do NOT claim "this error existed before"
- Do NOT skip or ignore failures
- Do NOT commit with failing checks
- Pre-existing errors: Fix them as part of your commit
- Lint errors in files you did not write: Still your responsibility if they block the commit
- A broken ESLint/clippy config: Fix the config, don't skip the check

**"Not related to my changes" is NEVER an excuse.** If `make check` fails, you fix it — regardless of who introduced the failure. The codebase must be clean after every commit.

## Pre-Commit Hook Enforcement

A git pre-commit hook (`.githooks/pre-commit`) runs `make check` automatically. This hook:

- Runs Rust checks (fmt, clippy, tests) when `.rs` or `Cargo.*` files are staged
- Runs frontend checks (svelte-check, ESLint, Vitest) when `.svelte`, `.ts`, `.js`, `.css`, or `.html` files are staged
- **MUST NOT be bypassed** with `git commit --no-verify`
- If the hook fails, the commit is rejected — fix the errors and retry

## Integration Verification

**NEVER assume. ALWAYS verify.**

Before calling ANY existing function or API:

1. **Read the source** — Check actual function signature
2. **Check the types** — Verify parameter names and types
3. **Run checks** — `make clippy` for Rust, `make check-frontend` for TypeScript — catch mismatches immediately

**NO backwards compatibility shims.** Fix ALL callers in same commit.

## ChunkHound Integration

Use `search_regex` to find function definitions before calling them — faster and more thorough than manual file reading. Use `search_semantic` for "how does X work" questions.

## Related Rules

- RULE-006 (coding-standards) — defines *what* to verify (specific checks and patterns)
- RULE-005 (chunkhound-usage) — tools for finding and verifying code before modifying it
- RULE-010 (end-to-end-completeness) — the full chain that must be verified
