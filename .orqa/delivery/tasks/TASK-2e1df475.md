---
id: TASK-2e1df475
title: "Frontend: References panel in artifact viewer"
description: Add a ReferencesPanel component below the frontmatter header showing incoming and outgoing artifact cross-references using the graph SDK.
status: completed
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - Incoming references shown as clickable ArtifactLink chips
  - Outgoing references shown as clickable ArtifactLink chips
  - Sections are collapsible
  - Empty sections are hidden
  - Clicking a reference navigates to the target artifact
  - make typecheck passes
relationships:
  - target: EPIC-6787bb93
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-3b119193
    type: depended-on-by
---


## What

Surface the artifact graph's cross-reference data directly in the viewer so users can see what references the current artifact and what it references, without leaving the viewer.

## How

1. Create `ReferencesPanel.svelte` in `ui/src/lib/components/artifact/`
2. Accept the current artifact ID as a prop
3. Call `referencesFrom(id)` and `referencesTo(id)` from the graph SDK
4. Render two collapsible sections: "Referenced by" (incoming) and "References" (outgoing)
5. Each reference rendered as an ArtifactLink chip
6. Place the component in ArtifactViewer below FrontmatterHeader

## Verification

- [ ] `make typecheck` passes
- [ ] References panel appears below frontmatter header
- [ ] Clicking a reference chip navigates to the target artifact
- [ ] Sections with no references are hidden
