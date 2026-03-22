---
id: TASK-257a5482
title: Merge overlapping documentation
description: "Consolidate 4 pairs of overlapping docs into single authoritative sources. Merge governance-hub into governance, guide/workflow into process/workflow, component-inventory into svelte-components, artifact-types into artifact-framework."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - "DOC-038 (governance-hub) merged into DOC-1bc9d0b9 (governance) — unique content preserved, file deleted"
  - DOC-5d1eed43 (guide/workflow) merged into DOC-7c66f103 (process/workflow) — file deleted
  - DOC-048 (component-inventory) merged into DOC-52bbfba5 (svelte-components) — file deleted
  - DOC-7d811a5d (artifact-types) merged into DOC-01ddd8aa (artifact-framework) — file deleted
  - All cross-references to merged docs updated to point to the surviving doc
  - No broken links remain
relationships:
  - target: EPIC-915291e7
    type: delivers
    rationale: Phase 1 — consolidate documentation before connecting to graph
  - target: TASK-fb9cfd61
    type: depends-on
  - target: TASK-d1307b98
    type: depended-on-by
---

## Scope

Merge 4 pairs of overlapping documentation:

1. **DOC-038** (governance-hub.md, 99 lines) → **DOC-1bc9d0b9** (governance.md, 239 lines): Both cover governance philosophy. Merge unique governance-hub content into governance.md, delete governance-hub.md.
2. **DOC-5d1eed43** (guide/workflow.md, 105 lines) → **DOC-7c66f103** (process/workflow.md, 252 lines): Same topic in two directories. Merge any unique guide content, delete guide/workflow.md.
3. **DOC-048** (component-inventory.md, 213 lines) → **DOC-52bbfba5** (svelte-components.md, 351 lines): Both catalog Svelte components. Merge, delete component-inventory.md.
4. **DOC-7d811a5d** (artifact-types.md, 116 lines) → **DOC-01ddd8aa** (artifact-framework.md, 959 lines): Both explain artifact schemas. Merge, delete artifact-types.md.

For each merge: read both files, identify unique content in the source, integrate into the target, update all cross-references, delete the source file.
