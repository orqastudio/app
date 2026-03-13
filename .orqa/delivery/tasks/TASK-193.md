---
id: TASK-193
title: Add inject action to enforcement schema
description: |
  Add a new 'inject' action type to the enforcement schema, the plugin
  rule-engine.mjs, and the Rust enforcement engine. The inject action loads
  skill content and returns it as systemMessage instead of blocking or warning.
status: done
created: "2026-03-11"
updated: "2026-03-12"
epic: EPIC-052
depends-on: []
---

## What

Add `inject` as a third action type alongside `block` and `warn` in enforcement
entries. When an `inject` enforcement entry matches, it reads the specified skill
content and returns it as a `systemMessage` to inject domain knowledge into the
agent's context.

Add a `skills` field to enforcement entries — an array of skill directory names
to inject when the entry matches.

## How

1. Update `.orqa/process/rules/schema.json`: add `inject` to action enum, add `skills` field
2. Update plugin `rule-engine.mjs`: handle `inject` action — read SKILL.md files, return as systemMessage
3. Update Rust enforcement engine: handle `Inject` action variant
4. Add deduplication: track which skills have been injected per turn to prevent flooding

## Verification

- Schema validates with `inject` action
- Rule engine can load and return skill content
- Duplicate injections in same turn are suppressed
