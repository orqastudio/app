---
id: EPIC-c8f1a7d3
type: epic
title: Plugin Composition Pipeline
description: Implement the three-layer merge engine that composes platform defaults, project configuration, and installed plugin contributions into unified TypeRegistry and RelationshipRegistry instances at daemon startup. The output is deterministic — same inputs always produce the same registries.
status: captured
priority: P1
created: 2026-04-13
updated: 2026-04-13
horizon: next
scoring:
  impact: 5
  urgency: 4
  complexity: 5
  dependencies: 3
relationships:
  - target: MS-21d5096a
    type: fulfils
    rationale: Stream 4 — composition pipeline is the core of the plugin framework
---

## Context

The plugin framework (P1: Plugin-Composed Everything) requires a composition pipeline that merges contributions from multiple layers into a unified runtime registry. No artifact type or relationship type is hardcoded in the engine — all come from plugins composed at startup.

## Acceptance Criteria

- [ ] Three-layer composition: platform defaults → project config → plugin contributions
- [ ] TypeRegistry populated from plugin manifests at daemon startup
- [ ] RelationshipRegistry populated from plugin manifests at daemon startup
- [ ] Conflict resolution: duplicate type declarations produce an error with plugin source identified
- [ ] Composition result is deterministic and logged to .state/ for debugging
- [ ] Registry available to all daemon subsystems (storage, enforcement, LSP) via shared state
