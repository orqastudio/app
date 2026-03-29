---
id: "TASK-0ac5f595"
type: "task"
title: "Extend ArtifactRef with relationship_type and process relationships array"
description: "Add relationship_type field to ArtifactRef in Rust and TypeScript. Update collect_forward_refs to process the relationships array from frontmatter, creating typed edges with relationship_type populated from each relationship's type field."
status: archived
created: 2026-03-13T00:00:00.000Z
updated: 2026-03-13T00:00:00.000Z
acceptance:
  - "ArtifactRef has relationship_type: Option<String> in Rust and relationship_type: string | null in TypeScript"
  - "relationships array items create ArtifactRef entries with target_id from target, field='relationships', and relationship_type from type"
  - "Existing refs from SINGLE_REF_FIELDS and ARRAY_REF_FIELDS have relationship_type: None/null"
  - "make check passes with zero errors"
  - "Graph stats show increased edge count reflecting relationships array edges"
relationships:
  - target: "EPIC-82dd0bd2"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

Extend the artifact graph to treat `relationships` array entries as first-class edges. Currently only 12 hardcoded frontmatter fields (`epic`, `depends-on`, `milestone`, etc.) create edges. The `relationships` array — which carries typed semantic edges like `enforced-by`, `grounded`, `grounded-by` — is stored in `node.frontmatter` but doesn't generate `ArtifactRef` entries.

## How

1. Add `relationship_type: Option\<String\>` to `ArtifactRef` struct in `backend/src-tauri/src/domain/artifact_graph.rs`
2. Add `relationship_type: string | null` to `ArtifactRef` interface in `ui/src/lib/types/artifact-graph.ts`
3. In `collect_forward_refs`, add processing for the `relationships` array:
   - Each item has `target` (string) and `type` (string)
   - Create `ArtifactRef { target_id: target, field: "relationships", source_id, relationship_type: Some(type) }`
4. Existing refs from single/array fields get `relationship_type: None`
5. Pass 2 backlink inversion works unchanged — the `relationship_type` propagates naturally

## Verification

- `make check` passes
- Run app, check GraphStats — edge count should increase (relationships array edges now counted)
- `artifactGraphSDK.referencesFrom("PD-48b310f9")` should include `enforced-by` edges with `relationship_type` populated

## Lessons

(none yet)
