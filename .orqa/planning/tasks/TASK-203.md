---
id: TASK-203
title: Consolidate code-pattern rules to reference linters
description: |
  Simplify rules that currently regex-match code patterns (unwrap, TODO, etc.)
  to instead reference the linter configs that enforce them. Remove regex
  enforcement for patterns that linters already cover.
status: done
created: "2026-03-11"
updated: "2026-03-12"
epic: EPIC-052
depends-on:
  - TASK-202
---

## What

Rules like [RULE-006](RULE-006) currently have enforcement entries that regex-match code
patterns (unwrap, TODO comments, etc.). These patterns are already caught by
clippy and ESLint. The regex enforcement should be replaced with `lint` event
entries that document the linter delegation, and the regex entries removed.

## How

1. Identify all enforcement entries with `event: file` that match code patterns
2. For each, verify the corresponding linter rule exists and is configured
3. Replace `file` + regex entries with `lint` entries documenting the delegation
4. Keep `bash` enforcement entries (e.g., --no-verify) — those aren't linter-covered
5. Update the rule-enforcement skill to reflect the new approach

## Verification

- No enforcement entries regex-match patterns that linters already catch
- `bash` enforcement entries for git safety remain unchanged
- `make check` still catches all the same violations via linters
