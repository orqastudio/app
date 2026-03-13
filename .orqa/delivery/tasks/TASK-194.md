---
id: TASK-194
title: Add lint event type to enforcement schema
description: |
  Add a 'lint' event type to the enforcement schema. This documents that
  enforcement for a pattern is delegated to an external linter tool, not
  handled by the OrqaStudio rule engine directly.
status: done
created: "2026-03-11"
updated: "2026-03-12"
epic: EPIC-052
depends-on: []
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
