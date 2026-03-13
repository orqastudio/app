---
id: TASK-307
title: "Design plugin-sidecar pairing mechanism (IMPL-019, IMPL-020)"
description: "Design the plugin type taxonomy, AI provider schema, and capability fulfilment model schemas. Design only — implementation is deferred to IDEA-071. Covers plugin.json schema extension, provider definition schema, and capability routing configuration shape."
status: todo
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-059
depends-on: []
acceptance:
  - "IMPL-019 and IMPL-020 maturity updated to understanding"
  - "Plugin.json schema extension designed with type array, requires shape per type, default-capabilities"
  - "AI provider schema designed for .orqa/providers/<name>.json"
  - "Capability routing config shape designed for project.json"
  - "All schemas documented, user-approved"
  - "IDEA-071 created to track implementation"
---

## What

Design how plugins declare which sidecar they require and how the system enforces that pairing. This covers IMPL-019 (declaration) and IMPL-020 (enforcement) as two sides of the same design.

Implementation is out of scope for EPIC-059 — deferred to [IDEA-071](IDEA-071).

## How

1. Extend plugin.json schema with `requires.sidecar` field
2. Define sidecar identity strings and detection mechanism
3. Design load-time filtering for the plugin loader
4. Design UI behaviour (greyed-out plugins for non-active sidecars)
5. Document interaction with RULE-040 capability resolution
6. Update IMPL-019 and IMPL-020 to understanding

## Verification

- Design documented and user-approved
- Plugin schema extension is concrete (not conceptual)
- IMPL-019 and IMPL-020 have maturity: understanding
