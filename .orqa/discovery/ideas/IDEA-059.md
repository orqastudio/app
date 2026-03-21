---
id: "IDEA-53205849"
title: "Component Library SDK for Plugin Views"
description: "Extract shared components into an importable SDK so plugins can create dynamic views that match the design system."
status: "completed"
created: "2026-03-12"
updated: "2026-03-13"
horizon: "active"
research-needed:
  - "Which components should be in the SDK vs remain internal?"
  - "How should the view registration API work?"
  - "How do plugins access theme tokens?"
  - "What's the distribution mechanism (npm package, bundled, git submodule)?"
relationships:
  - target: "EPIC-527de6a3"
    type: "realises"
  - target: "PILLAR-569581e0"
    type: "grounded"
  - target: "PERSONA-015e8c2c"
    type: "benefits"
---
## Description

OrqaStudio's plugin architecture needs a way for plugins to create custom views. Currently, shared components live in `$lib/components/shared/` but are only available to the core app. Plugins need:

1. **Component library SDK** — shared components (EmptyState, StatusIndicator, etc.) as an importable library
2. **Artifact Graph SDK** — already exists (`artifact-graph.svelte.ts`), needs documentation
3. **View registration API** — plugins register custom views for artifact types or dashboard panels
4. **Theme tokens** — plugins access the design system tokens

See [RES-00ec6dd1](RES-00ec6dd1) for context on plugin architecture requirements.

## Related Ideas

- [IDEA-b77e2955](IDEA-b77e2955) — Plugin distribution via git submodules
- [IDEA-9713910f](IDEA-9713910f) — Integration ecosystem
- [IDEA-af73e0af](IDEA-af73e0af) — Multi-view output system
