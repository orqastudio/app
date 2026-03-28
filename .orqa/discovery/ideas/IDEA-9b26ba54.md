---
id: "IDEA-9b26ba54"
type: discovery-idea
title: "Component Library SDK for Plugin Views"
description: "Extract shared components into an importable SDK so plugins can create dynamic views that match the design system."
status: archived
created: "2026-03-12"
updated: "2026-03-13"
horizon: "active"
research-needed:
  - "Which components should be in the SDK vs remain internal?"
  - "How should the view registration API work?"
  - "How do plugins access theme tokens?"
  - "What's the distribution mechanism (npm package, bundled, git submodule)?"
relationships:
  - target: "EPIC-6e11f0af"
    type: "realises"
  - target: "PILLAR-c9e0a695"
    type: "grounded"
  - target: "PERSONA-477971bf"
    type: "benefits"
---

## Description

OrqaStudio's plugin architecture needs a way for plugins to create custom views. Currently, shared components live in `$lib/components/shared/` but are only available to the core app. Plugins need:

1. **Component library SDK** — shared components (EmptyState, StatusIndicator, etc.) as an importable library
2. **Artifact Graph SDK** — already exists (`artifact-graph.svelte.ts`), needs documentation
3. **View registration API** — plugins register custom views for artifact types or dashboard panels
4. **Theme tokens** — plugins access the design system tokens

See [RES-8fee4dad](RES-8fee4dad) for context on plugin architecture requirements.

## Related Ideas

- [IDEA-5113eeae](IDEA-5113eeae) — Plugin distribution via git submodules
- [IDEA-7a57ba89](IDEA-7a57ba89) — Integration ecosystem
- [IDEA-d290f65c](IDEA-d290f65c) — Multi-view output system
