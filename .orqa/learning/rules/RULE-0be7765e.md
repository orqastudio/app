---
id: RULE-0be7765e
type: rule
title: Error Ownership
description: All errors are your responsibility. Pre-existing errors must be fixed. Never skip or ignore failures.
status: active
enforcement_type: mechanical
created: 2026-03-07
updated: 2026-03-12
enforcement:

  - mechanism: behavioral

    message: "All errors are your responsibility; never claim a pre-existing error, skip failures, or commit with failing checks"

  - mechanism: pre-commit

    check: "make check"

  - mechanism: hook

    type: PreToolUse
    event: bash
    action: block
    pattern: "--no-verify"
relationships:

  - target: AD-2d58941b

    type: enforces
---

## Rule (NON-NEGOTIABLE)

**ALL errors are YOUR responsibility. No exceptions.**

- Do NOT claim "this error existed before"
- Do NOT skip or ignore failures
- Do NOT commit with failing checks
- Pre-existing errors: Fix them as part of your commit
- Lint errors in files you did not write: Still your responsibility if they block the commit
- A broken linter config: Fix the config, don't skip the check

**"Not related to my changes" is NEVER an excuse.** If quality checks fail, you fix them — regardless of who introduced the failure. The codebase must be clean after every commit.

## Pre-Commit Hook Enforcement

A git pre-commit hook (`.githooks/pre-commit`) runs quality checks based on which files are staged.

- **MUST NOT be bypassed** with `git commit --no-verify`
- If the hook fails, the commit is rejected — fix the errors and retry

## Integration Verification

**NEVER assume. ALWAYS verify.**

Before calling ANY existing function or API:

1. **Read the source** — Check actual function signature
2. **Check the types** — Verify parameter names and types
3. **Run checks** — Run the project's linter and type-checker to catch mismatches immediately

**NO backwards compatibility shims.** Fix ALL callers in the same commit.

## Code Search Integration

Use `search_regex` to find function definitions before calling them — faster and more thorough than manual file reading. Use `search_semantic` for "how does X work" questions.

## Related Rules

- [RULE-9814ec3c](RULE-9814ec3c) (coding-standards) — defines *what* to verify (specific checks and patterns)
- [RULE-0d29fc91](RULE-0d29fc91) (code-search-usage) — tools for finding and verifying code before modifying it
- end-to-end-completeness — the full chain that must be verified
