---
id: "IMPL-db7ab92c"
type: lesson
title: "App UI save can destructively overwrite project.json with default values"
description: "Clicking save in the OrqaStudio app UI overwrote a valid project.json with blank/default values — null description, empty artifacts array, empty frameworks, null custom_system_prompt. The file had to be recovered via git checkout. The pre-commit hook did not catch this because the overwrite happened in-app, not through a git commit."
status: "active"
created: "2026-03-13"
updated: "2026-03-13"
maturity: "understanding"
recurrence: 1
relationships: []
---
## Pattern

The app's save functionality wrote blank/default values over a valid project.json config. The user clicked save in the UI, and the result was a project.json with null description, empty frameworks, empty artifacts array, and null custom_system_prompt. This required a `git checkout` to recover. The pre-commit hook did NOT catch this because the save was in-app, not through a git commit.

## Fix

Not yet determined. Possible approaches:
1. Validation before save — reject writes that would blank required fields
2. Dirty-checking — only save fields the user actually modified
3. Backup before overwrite — create .bak before saving
4. Schema validation on write — project.json should validate against a schema before being written to disk

## Triage

Deferred to [IDEA-30f73d06](IDEA-30f73d06) — app bug requiring validation before save. Out of scope for [EPIC-88f359b0](EPIC-88f359b0) (philosophy alignment epic, not app bug fix).