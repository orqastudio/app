---
id: TASK-039
title: Shared list item and status indicator components
description: Extracts duplicated list item and status display logic into reusable shared components covering all artifact lifecycle statuses, used consistently across the sidebar and landing grid.
status: done
scope:
  - ui/lib/components/shared/
  - ui/lib/components/navigation/ArtifactNav.svelte
  - ui/lib/components/artifact/ArtifactLanding.svelte
  - ui/lib/components/artifact/FrontmatterHeader.svelte
created: "2026-03-09"
updated: "2026-03-09"
epic: EPIC-043
assignee: AGENT-001
skills:
  - SKILL-005
  - SKILL-008
acceptance:
  - A shared StatusIndicator component exists with semantic colours and icons per status value
  - StatusIndicator works in list items
  - detail views
  - and badge contexts
  - StatusIndicator covers all artifact lifecycle statuses (draft
  - ready
  - in-progress
  - done
  - captured
  - exploring
  - shaped
  - promoted
  - archived
  - surpassed
  - etc.)
  - A shared ArtifactListItem component exists with consistent structure (title
  - optional description
  - status indicator)
  - ArtifactListItem used in both ArtifactNav (sidebar) and ArtifactLanding (grid)
  - Task list displays title consistently (never raw TASK-ID when title exists)
  - Artifact list bars include status indicator (dot
  - icon
  - or colour)
  - Components are reusable across planning AND governance sections
---
## Findings Addressed

- **F12**: Task list inconsistent — sometimes title+description, sometimes just title, sometimes raw TASK-ID
- **F13**: Status badges generic except active — need clearer visual hierarchy
- **F14**: Artifact list bars need status indicator and wider layout
- **F17**: Status/metadata indicators should be reusable across sections

## Investigation Notes

`statusVariant()` function exists in `FrontmatterHeader.svelte` but is NOT exported — only usable within that component. `RuleViewer.svelte` and `ArtifactLanding.svelte` hardcode badge variants instead of using a shared function.

No shared list item component exists. Rendering is inline in both `ArtifactNav.svelte` (lines 182-235) and `ArtifactLanding.svelte` (lines 147-186) with duplicated code.

## Root Cause

Missing shared abstractions. Status display logic is duplicated. List item structure is defined inline per context rather than as a reusable component.

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
