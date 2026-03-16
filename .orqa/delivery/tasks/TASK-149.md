---
id: TASK-149
title: Create stub scanner pre-commit hook
description: Pre-commit hook that greps staged production files for TODO/FIXME/HACK comments and blocks the commit if found.
status: completed
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - Hook blocks commits containing TODO/FIXME/HACK in production source files
  - Test files are excluded from scanning
  - Hook runs as part of the existing pre-commit chain
  - RULE-020 updated to reference the hook
relationships:
  - target: EPIC-049
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-339
    type: depended-on-by
---
## What

[RULE-020](RULE-020) (No Stubs) explicitly calls for a stub scanner as part of CI/quality checks. Currently there is no automated enforcement — agents self-police. Create a pre-commit hook that scans staged production files for TODO/FIXME/HACK comments and blocks the commit.

## How

1. Create a hook script that filters staged files to production code (exclude tests/, tmp/)
2. Grep for TODO, FIXME, HACK patterns in those files
3. If matches found, print them and exit non-zero to block the commit
4. Integrate into the existing `.githooks/pre-commit` pipeline or `.orqa/process/hooks/`
5. Update [RULE-020](RULE-020) to reference the hook as enforcement mechanism

## Verification

- [ ] Commit with a TODO comment in a .rs file is blocked
- [ ] Commit with a TODO comment in a test file is allowed
- [ ] Hook output clearly shows which files/lines contain stubs
- [ ] Existing pre-commit checks still run
