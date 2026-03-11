---
id: EPIC-004
title: Artifact Editing UI
description: Build an editor component for governance artifacts, connecting existing backend CRUD commands to a CodeMirror-based editing UI. Absorbed into EPIC-005.
status: draft
priority: P1
created: "2026-03-07"
updated: "2026-03-11"
milestone: MS-001
pillars:
  - PILLAR-001
research-refs: []
docs-required:
  - DOC-057
  - DOC-005
docs-produced:
  - DOC-057
  - DOC-005
scoring:
  pillar: 5
  impact: 5
  dependency: 2
  effort: 3
  score: 10.3
---
## Why P1

Can't manage governance in-app without editing. Currently requires switching to a text editor.

## Tasks

- [ ] CodeMirror 6 editor component for markdown/YAML editing
- [ ] Edit mode toggle on artifact viewers (view -> edit)
- [ ] Create new artifact from template (agents, rules, skills, hooks)
- [ ] Delete artifact with confirmation dialog
- [ ] Wire artifact store methods to backend CRUD commands

## Context

This epic's scope has been absorbed into [EPIC-005](EPIC-005) (Artifact Browser — Sort, Filter, Search, Edit), which covers in-app artifact editing as Phase 3 alongside sort/group/filter, AI search, and the references panel. Marked done as the scope is now tracked under [EPIC-005](EPIC-005).

## Implementation Design

N/A — scope absorbed into [EPIC-005](EPIC-005).
