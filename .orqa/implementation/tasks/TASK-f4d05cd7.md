---
id: "TASK-f4d05cd7"
type: "task"
title: "Clean up Claude memory files that duplicate artifact knowledge"
description: "6 of 10 Claude memory files are fully covered by artifacts in .orqa/. Per user directive, project memory should be empty when artifacts contain the knowledge. Delete duplicated memory files and verify remaining files are genuinely additive."
status: archived
created: 2026-03-13T00:00:00.000Z
updated: 2026-03-13T00:00:00.000Z
acceptance:
  - "Memory files that duplicate artifact knowledge are deleted"
  - "Remaining memory files contain only genuinely additive information"
  - "MEMORY.md index is updated to reflect remaining files"
relationships:
  - target: "EPIC-88f359b0"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-9d1e01d7"
    type: "depends-on"
---

## What

Audit all Claude memory files in `C:\Users\Bobbi\.claude\projects\C--Users-Bobbi-code-orqa-studio\memory\` and delete those whose content is fully captured in `.orqa/` artifacts.

## How

1. Read each memory file
2. Cross-reference against `.orqa/` artifacts (rules, skills, decisions, documentation)
3. Delete files where the artifact system has full coverage
4. Update MEMORY.md index

## Verification

- `ls` memory directory shows only genuinely additive files
- MEMORY.md is accurate and concise
