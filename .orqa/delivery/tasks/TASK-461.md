---
id: TASK-461
title: "Merge overlapping documentation"
description: "Consolidate 4 pairs of overlapping docs into single authoritative sources. Merge governance-hub into governance, guide/workflow into process/workflow, component-inventory into svelte-components, artifact-types into artifact-framework."
status: done
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-064
depends-on:
  - TASK-460
assignee: null
skills:
  - SKILL-037
  - SKILL-011
acceptance:
  - DOC-038 (governance-hub) merged into DOC-039 (governance) — unique content preserved, file deleted
  - DOC-082 (guide/workflow) merged into DOC-035 (process/workflow) — file deleted
  - DOC-048 (component-inventory) merged into DOC-016 (svelte-components) — file deleted
  - DOC-081 (artifact-types) merged into DOC-036 (artifact-framework) — file deleted
  - All cross-references to merged docs updated to point to the surviving doc
  - No broken links remain
relationships:
  - target: EPIC-064
    type: delivers
    rationale: Phase 1 — consolidate documentation before connecting to graph
  - target: EPIC-064
    type: belongs-to
    rationale: Task belongs to this epic
  - target: TASK-460
    type: informed-by
    rationale: Deletions must happen first to avoid merge conflicts with already-deleted content
  - target: RES-062
    type: informed-by
    rationale: Documentation audit identified these overlaps
---

## Scope

Merge 4 pairs of overlapping documentation:

1. **DOC-038** (governance-hub.md, 99 lines) → **DOC-039** (governance.md, 239 lines): Both cover governance philosophy. Merge unique governance-hub content into governance.md, delete governance-hub.md.
2. **DOC-082** (guide/workflow.md, 105 lines) → **DOC-035** (process/workflow.md, 252 lines): Same topic in two directories. Merge any unique guide content, delete guide/workflow.md.
3. **DOC-048** (component-inventory.md, 213 lines) → **DOC-016** (svelte-components.md, 351 lines): Both catalog Svelte components. Merge, delete component-inventory.md.
4. **DOC-081** (artifact-types.md, 116 lines) → **DOC-036** (artifact-framework.md, 959 lines): Both explain artifact schemas. Merge, delete artifact-types.md.

For each merge: read both files, identify unique content in the source, integrate into the target, update all cross-references, delete the source file.
