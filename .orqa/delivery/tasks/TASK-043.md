---
id: TASK-043
title: Scanner frontmatter extraction
description: Extends the artifact scanner to extract and expose the status field from YAML frontmatter so sidebar list items display correct titles and status indicators for all artifact types.
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-002
acceptance:
  - DocNode struct includes optional status field populated from YAML frontmatter
  - DocNode label always uses frontmatter title when available (never raw filename like TASK-002)
  - ArtifactListItem receives status from DocNode and renders StatusIndicator dot
  - All tasks
  - epics
  - ideas
  - milestones show status dots in sidebar list
  - TypeScript NavTree types updated to match Rust struct
relationships:
  - target: EPIC-043
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-333
    type: depended-on-by
---
## Findings Addressed

- **F19**: Tasks showing [TASK-002](TASK-002) instead of title
- **F21**: Status dots not showing on list items

## Root Cause

The Rust `DocNode` struct has `label`, `path`, `children`, `description` but no `status` field. The scanner extracts `title` and `description` from frontmatter but not `status`. The label fallback to humanized filename sometimes produces raw IDs.

## Fix

1. Add `pub status: Option<String>` to `DocNode` in `artifact_reader.rs`
2. Extract `status` from YAML frontmatter during scanning (same as title/description extraction)
3. Ensure `label` always prefers frontmatter `title` over filename — verify the existing logic handles all-caps IDs correctly
4. Update `DocNode` TypeScript interface in `nav-tree.ts`
5. Pass `status` to `ArtifactListItem` in `ArtifactNav.svelte`

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
