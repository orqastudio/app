---
id: IDEA-059
title: Component Library SDK for Plugin Views
description: Extract shared components into an importable SDK so plugins can create dynamic views that match the design system.
status: completed
created: 2026-03-12
updated: 2026-03-13
horizon: active
research-needed:
  - "Which components should be in the SDK vs remain internal?"
  - "How should the view registration API work?"
  - "How do plugins access theme tokens?"
  - "What's the distribution mechanism (npm package, bundled, git submodule)?"
relationships:
  - target: DOC-071
    type: informed-by
    rationale: "Auto-generated inverse of documented-by relationship from DOC-071"
  - target: MS-001
    type: delivers
  - target: EPIC-062
    type: evolves-into
---
## Description

OrqaStudio's plugin architecture needs a way for plugins to create custom views. Currently, shared components live in `$lib/components/shared/` but are only available to the core app. Plugins need:

1. **Component library SDK** — shared components (EmptyState, StatusIndicator, etc.) as an importable library
2. **Artifact Graph SDK** — already exists (`artifact-graph.svelte.ts`), needs documentation
3. **View registration API** — plugins register custom views for artifact types or dashboard panels
4. **Theme tokens** — plugins access the design system tokens

See [RES-046](RES-046) for context on plugin architecture requirements.

## Related Ideas

- [IDEA-038](IDEA-038) — Plugin distribution via git submodules
- [IDEA-009](IDEA-009) — Integration ecosystem
- [IDEA-015](IDEA-015) — Multi-view output system
