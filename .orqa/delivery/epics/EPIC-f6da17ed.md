---
id: EPIC-f6da17ed
type: epic
title: "Core Workflow Engine and State Machines"
description: "Build the state machine evaluation engine, YAML workflow format, guard/action primitives, category-based composition, and workflow resolver. Plugins own state machines; core provides the engine."
status: captured
priority: P1
created: 2026-03-25
updated: 2026-03-25
horizon: active
relationships:
  - target: MS-654badde
    type: fulfils
    rationale: "Workflow engine is core infrastructure for dogfooding"
  - target: EPIC-c828007a
    type: depends-on
    rationale: "Workflow engine uses graph queries for guards — graph must be correct first"
  - target: EPIC-a63fde02
    type: depended-on-by
  - target: EPIC-ecef93a8
    type: depended-on-by
---

## Scope

From RES-55bacef1 sections 3 (Workflow Composition) and 7 (State Machine Design):

- YAML workflow format with JSON Schema validation
- State machine evaluation engine (transition resolution, guard evaluation, action execution)
- State category vocabulary (planning, active, review, completed, terminal) — UI renders generically via categories
- Workflow resolver — runs as part of `orqa plugin install`, merges plugin contributions into `.orqa/workflows/<name>.resolved.yaml`
- Plugin workflow file structure — workflow-definition plugin defines the skeleton, stage-definition plugins fill slots (composability)
- Guard primitives: field_check, relationship_check, query, role_check (declarative only, code hooks for complex logic)
- Action primitives: set_field, append_log, create_artifact, notify
- Migration framework — forward-compatible addition, `orqa migrate` for status mapping, no backwards compatibility (pre-release)
- Migrate existing hardcoded status values in schema.json to plugin-owned state machines

## Design Constraints (from AD-e9f71dc1)

- No workflow inheritance — plugin owns complete state machine
- Declarative guards only — code hooks for anything beyond field/relationship/graph checks
- Cross-plugin coupling via documented vocabulary is acceptable
- No backwards compatibility — breaking changes expected, data migrated
