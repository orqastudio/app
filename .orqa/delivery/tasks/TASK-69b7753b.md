---
id: TASK-69b7753b
title: Audit all skill definitions against codebase
description: "Verify every skill in .orqa/process/skills/ has accurate code patterns, valid file paths in examples, correct layer classification, and no stale module/function references."
status: completed
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - All file paths in skill examples resolve to existing files
  - Code patterns described match actual implementations
  - Function signatures in examples match actual source code
  - All Related Skills references point to existing skill directories
relationships:
  - target: EPIC-4a7aeacb
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-1ac4d16f
    type: depended-on-by
  - target: TASK-77b6e5bd
    type: depended-on-by
  - target: TASK-4a8c3c6a
    type: depended-on-by
  - target: TASK-ec136ce9
    type: depended-on-by
---
## What

Systematic audit of all skill definition files to ensure the patterns, examples, and references they contain are accurate against the current codebase.

## How

1. List all skill directories in `.orqa/process/skills/`
2. For each skill, read SKILL.md and any supporting files
3. For code patterns: search codebase to verify they match reality
4. For file paths: verify they exist on disk
5. Fix any stale content

## Verification

- Every file path referenced in skill examples exists
- Code patterns in skills match `grep`/`search_regex` results from the actual codebase
- No skills reference removed or renamed modules
