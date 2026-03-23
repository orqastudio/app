---
id: TASK-4f45e5b9
type: task
title: Add lint event type to enforcement schema
description: |
  Add a 'lint' event type to the enforcement schema. This documents that
  enforcement for a pattern is delegated to an external linter tool, not
  handled by the OrqaStudio rule engine directly.
status: completed
created: 2026-03-11
updated: 2026-03-12
acceptance:
  - lint event type added to enforcement schema and validates
  - "Enforcement entries with event: lint pass schema validation"
relationships:
  - target: EPIC-4440cdd4
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-965b2b81
    type: depended-on-by
  - target: TASK-d6727d2f
    type: depended-on-by
---

## What

Add `lint` as a new event type in enforcement entries. A `lint` event documents
that a specific coding standard is enforced by an external tool (clippy, ESLint,
svelte-check) rather than by OrqaStudio's regex engine. This is declarative — it
doesn't execute anything, but it closes the gap between "documented standard" and
"enforcement exists".

## How

1. Update `.orqa/process/rules/schema.json`: add `lint` to event enum
2. Document the convention: `lint` entries carry `pattern` (the linter rule name),
   `message` (what it enforces), and optionally `paths` (which files it applies to)
3. No changes needed to rule-engine.mjs — it ignores events it doesn't handle

## Verification

- Schema validates with `lint` event
- Can create enforcement entries with `event: lint` that pass schema validation