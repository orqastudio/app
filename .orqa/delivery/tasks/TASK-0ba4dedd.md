---
id: "TASK-0ba4dedd"
type: "task"
title: "Merge overlapping documentation"
description: "Consolidate 4 pairs of overlapping docs into single authoritative sources. Merge governance-hub into governance, guide/workflow into process/workflow, component-inventory into svelte-components, artifact-types into artifact-framework."
status: archived
priority: "P1"
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
created: 2026-03-14T00:00:00.000Z
updated: 2026-03-14T00:00:00.000Z
assignee: null
acceptance:
  - "DOC-038 (governance-hub) merged into DOC-06224bf6 (governance) — unique content preserved, file deleted"
  - "DOC-e42efeaf (guide/workflow) merged into DOC-db5b37dc (process/workflow) — file deleted"
  - "DOC-048 (component-inventory) merged into DOC-2c94f7ba (svelte-components) — file deleted"
  - "DOC-3d8ed14e (artifact-types) merged into DOC-28344cd7 (artifact-framework) — file deleted"
  - "All cross-references to merged docs updated to point to the surviving doc"
  - "No broken links remain"
relationships:
  - target: "EPIC-12fba656"
    type: "delivers"
    rationale: "Phase 1 — consolidate documentation before connecting to graph"
  - target: "TASK-ae0051a6"
    type: "depends-on"
---

## Scope

Merge 4 pairs of overlapping documentation:

1. **DOC-038** (governance-hub.md, 99 lines) → **DOC-06224bf6** (governance.md, 239 lines): Both cover governance philosophy. Merge unique governance-hub content into governance.md, delete governance-hub.md.
2. **DOC-e42efeaf** (guide/workflow.md, 105 lines) → **DOC-db5b37dc** (process/workflow.md, 252 lines): Same topic in two directories. Merge any unique guide content, delete guide/workflow.md.
3. **DOC-048** (component-inventory.md, 213 lines) → **DOC-2c94f7ba** (svelte-components.md, 351 lines): Both catalog Svelte components. Merge, delete component-inventory.md.
4. **DOC-3d8ed14e** (artifact-types.md, 116 lines) → **DOC-28344cd7** (artifact-framework.md, 959 lines): Both explain artifact schemas. Merge, delete artifact-types.md.

For each merge: read both files, identify unique content in the source, integrate into the target, update all cross-references, delete the source file.
