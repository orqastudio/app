---
id: TASK-291
title: "Scope documentation to human-consumption content"
description: "Audit documentation/ contents. Process-defining docs (DoD, DoR) get absorbed into rules. Human-consumption docs stay. Move/delete/consolidate as needed."
status: todo
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-059
depends-on: [TASK-287, TASK-288, TASK-289]
assignee: null
docs: []
skills: []
acceptance:
  - "documentation/ contains only human-consumption content"
  - "Process-defining content absorbed into appropriate rules/skills"
  - "No documentation page defines a process that should be a first-class artifact"
  - "project.json documentation paths updated"
rule-overrides: []
---

## What

Audit and clean documentation/ so it only contains human-readable guides, not process definitions.

## How

1. List all files in `.orqa/documentation/`
2. For each: is this a human guide or a process definition?
3. Process definitions → absorb into rules or skills
4. Human guides → keep, ensure they reference process artifacts rather than defining them
5. Update `project.json` if subdirectory structure changes

## Verification

- Every file in documentation/ is a human-consumption guide
- No process definitions remain in documentation/
- All absorbed content is traceable to its new location
