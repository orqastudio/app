---
id: "TASK-53493d31"
title: "Design plugin-sidecar pairing mechanism (IMPL-e22b63b4, IMPL-a97eccb6)"
description: "Design the plugin type taxonomy, AI provider schema, and capability fulfilment model schemas. Design only — implementation is deferred to IDEA-1287dd52. Covers plugin.json schema extension, provider definition schema, and capability routing configuration shape."
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
acceptance:
  - "IMPL-e22b63b4 and IMPL-a97eccb6 maturity updated to understanding"
  - "Plugin.json schema extension designed with type array, requires shape per type, default-capabilities"
  - "AI provider schema designed for .orqa/providers/<name>.json"
  - "Capability routing config shape designed for project.json"
  - "All schemas documented, user-approved"
  - "IDEA-1287dd52 created to track implementation"
relationships:
  - target: "EPIC-942c7678"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-cea1bc37"
    type: "depended-on-by"
---
## What

Design how plugins declare which sidecar they require and how the system enforces that pairing. This covers [IMPL-e22b63b4](IMPL-e22b63b4) (declaration) and [IMPL-a97eccb6](IMPL-a97eccb6) (enforcement) as two sides of the same design.

Implementation is out of scope for [EPIC-942c7678](EPIC-942c7678) — deferred to [IDEA-1287dd52](IDEA-1287dd52).

## How

1. Extend plugin.json schema with `requires.sidecar` field
2. Define sidecar identity strings and detection mechanism
3. Design load-time filtering for the plugin loader
4. Design UI behaviour (greyed-out plugins for non-active sidecars)
5. Document interaction with [RULE-92dba0cb](RULE-92dba0cb) capability resolution
6. Update [IMPL-e22b63b4](IMPL-e22b63b4) and [IMPL-a97eccb6](IMPL-a97eccb6) to understanding

## Verification

- Design documented and user-approved
- Plugin schema extension is concrete (not conceptual)
- [IMPL-e22b63b4](IMPL-e22b63b4) and [IMPL-a97eccb6](IMPL-a97eccb6) have maturity: understanding
