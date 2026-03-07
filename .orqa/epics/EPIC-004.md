---
id: EPIC-004
title: "Artifact Editing UI"
status: draft
priority: P1
milestone: MS-001
created: 2026-03-07
updated: 2026-03-07
deadline: null
plan: null
depends-on: []
blocks: []
assignee: null
pillar:
  - clarity-through-structure
scoring:
  pillar: 5
  impact: 5
  dependency: 2
  effort: 3
score: 10.3
roadmap-ref: "D4"
docs-required:
  - docs/wireframes/artifact-browser.md
  - docs/architecture/ipc-commands.md
docs-produced:
  - docs/wireframes/artifact-browser.md (update with edit mode wireframes)
  - docs/architecture/ipc-commands.md (verify artifact CRUD commands documented)
tags: [artifacts, editing, governance]
---

# Artifact Editing UI

Backend CRUD exists (`artifact_create`, `artifact_update`, `artifact_delete`). Read-only viewers exist. Missing: an editor component for `.claude/` governance artifacts.

## Why P1

Can't manage governance in-app without editing. Currently requires switching to a text editor.

## Tasks

- [ ] CodeMirror 6 editor component for markdown/YAML editing
- [ ] Edit mode toggle on artifact viewers (view -> edit)
- [ ] Create new artifact from template (agents, rules, skills, hooks)
- [ ] Delete artifact with confirmation dialog
- [ ] Wire artifact store methods to backend CRUD commands
