---
id: "TASK-f51abfea"
type: "task"
title: "Design plugin-sidecar pairing mechanism (IMPL-b8ec72e2, IMPL-acea0394)"
description: "Design the plugin type taxonomy, AI provider schema, and capability fulfilment model schemas. Design only — implementation is deferred to IDEA-459f417a. Covers plugin.json schema extension, provider definition schema, and capability routing configuration shape."
status: archived
created: "2026-03-13"
updated: "2026-03-13"
acceptance:
  - "IMPL-b8ec72e2 and IMPL-acea0394 maturity updated to understanding"
  - "Plugin.json schema extension designed with type array, requires shape per type, default-capabilities"
  - "AI provider schema designed for .orqa/providers/<name>.json"
  - "Capability routing config shape designed for project.json"
  - "All schemas documented, user-approved"
  - "IDEA-459f417a created to track implementation"
relationships:
  - target: "EPIC-88f359b0"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

Design how plugins declare which sidecar they require and how the system enforces that pairing. This covers [IMPL-b8ec72e2](IMPL-b8ec72e2) (declaration) and [IMPL-acea0394](IMPL-acea0394) (enforcement) as two sides of the same design.

Implementation is out of scope for [EPIC-88f359b0](EPIC-88f359b0) — deferred to [IDEA-459f417a](IDEA-459f417a).

## How

1. Extend plugin.json schema with `requires.sidecar` field
2. Define sidecar identity strings and detection mechanism
3. Design load-time filtering for the plugin loader
4. Design UI behaviour (greyed-out plugins for non-active sidecars)
5. Document interaction with [RULE-8abcbfd5](RULE-8abcbfd5) capability resolution
6. Update [IMPL-b8ec72e2](IMPL-b8ec72e2) and [IMPL-acea0394](IMPL-acea0394) to understanding

## Verification

- Design documented and user-approved
- Plugin schema extension is concrete (not conceptual)
- [IMPL-b8ec72e2](IMPL-b8ec72e2) and [IMPL-acea0394](IMPL-acea0394) have maturity: understanding
