---
id: TASK-085
title: Audit all skill definitions against codebase
description: Verify every skill in .orqa/team/skills/ has accurate code patterns, valid file paths in examples, correct layer classification, and no stale module/function references.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
scope:
  - Read every SKILL.md in .orqa/team/skills/
  - Verify code patterns match actual codebase implementations
  - Verify file paths in examples exist
  - Verify function signatures and type definitions are current
  - Check layer field accuracy (canon/project/plugin)
  - Check Related Skills references point to existing skills
acceptance:
  - All file paths in skill examples resolve to existing files
  - Code patterns described match actual implementations
  - Function signatures in examples match actual source code
  - All Related Skills references point to existing skill directories
---
## What

Systematic audit of all skill definition files to ensure the patterns, examples, and references they contain are accurate against the current codebase.

## How

1. List all skill directories in `.orqa/team/skills/`
2. For each skill, read SKILL.md and any supporting files
3. For code patterns: search codebase to verify they match reality
4. For file paths: verify they exist on disk
5. Fix any stale content

## Verification

- Every file path referenced in skill examples exists
- Code patterns in skills match `grep`/`search_regex` results from the actual codebase
- No skills reference removed or renamed modules
