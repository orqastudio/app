---
id: "TASK-2bbc5077"
type: task
title: "Consolidate code-pattern rules to reference linters"
description: "Simplify rules that currently regex-match code patterns (unwrap, TODO, etc.)

  to instead reference the linter configs that enforce them. Remove regex

  enforcement for patterns that linters already cover.\n"
status: "completed"
created: "2026-03-11"
updated: "2026-03-12"
acceptance:
  - "No enforcement entries regex-match patterns already caught by linters"
  - "bash enforcement entries for git safety remain unchanged"
  - "make check still catches all documented violations"
relationships:
  - target: "EPIC-4440cdd4"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-05357368"
    type: "depends-on"
  - target: "TASK-34007190"
    type: "depended-on-by"
  - target: "TASK-d6727d2f"
    type: "depended-on-by"
---
## What

Rules like [RULE-b49142be](RULE-b49142be) currently have enforcement entries that regex-match code
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