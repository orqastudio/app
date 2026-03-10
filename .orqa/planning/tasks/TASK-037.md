---
id: TASK-037
title: Fix documentation visibility and navigation
description: Fixes the documentation section failing to appear in the sidebar and artifact viewer when selected, and corrects breadcrumb paths that duplicate hierarchy segments.
status: done
created: "2026-03-09"
updated: "2026-03-09"
epic: EPIC-043
assignee: AGENT-002
skills:
  - SKILL-005
  - SKILL-008
scope:
  - ui/lib/components/layout/AppLayout.svelte
  - ui/lib/components/layout/NavSubPanel.svelte
  - ui/lib/components/layout/ActivityBar.svelte
  - ui/lib/components/navigation/ArtifactNav.svelte
  - ui/lib/components/artifact/Breadcrumb.svelte
  - ui/lib/stores/navigation.svelte.ts
acceptance:
  - Documentation section visible and navigable when clicking the docs icon in ActivityBar
  - Sidebar populates with documentation tree when a direct-type artifact is selected
  - Breadcrumbs start with a home icon linking to dashboard
  - Breadcrumb path does not duplicate section hierarchy (no "Planning > Research > .orqa > Planning > Research > Audits")
  - Breadcrumb segments between home and leaf are clickable and navigate to the correct level
---
## Findings Addressed

- **F1**: Documentation not visible in UI at all
- **F2**: Main nav icon → sidebar and artifact viewer are empty
- **F15**: Breadcrumbs duplicate path, base should be home icon

## Investigation Notes

Documentation IS configured in `project.json` (key: "docs", path: ".orqa/documentation"). The scanner should find it. The issue is likely in how `NavigationStore.setActivity("docs")` renders for direct types vs groups — `NavSubPanel` may not trigger correctly for non-group entries.

Breadcrumbs are built in two separate places (`ArtifactNav.svelte:89-118` and `ArtifactViewer.svelte:98-104`) with different logic. The duplication comes from `buildBreadcrumbs()` including both group label AND full path segments.

## Root Cause

Navigation assumes artifact sections are groups with children. Direct types (like "docs") take a different code path that may not populate the sidebar or viewer correctly.

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
