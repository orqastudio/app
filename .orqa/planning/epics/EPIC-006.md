---
id: EPIC-006
title: File Watcher for External Changes
description: Watch .claude/ and .orqa/ directories for external modifications and refresh artifact list and viewer when files change on disk.
status: draft
priority: P2
created: 2026-03-07
updated: 2026-03-07
milestone: MS-001
pillars:
  - PILLAR-001
research-refs: []
docs-required:
  - docs/architecture/rust-modules.md
docs-produced:
  - docs/architecture/rust-modules.md (update with file watcher module)
  - docs/architecture/decisions.md (AD for file watching strategy and debouncing)
scoring:
  pillar: 3
  impact: 3
  dependency: 2
  effort: 2
  score: 10.5
---
## Why P2

Without this, CLI edits to governance files and `.orqa/` artifacts aren't reflected in the app until manual refresh. Important for dogfooding where both the CLI and app are used simultaneously.

## Tasks

- [ ] Add `notify` crate for filesystem watching
- [ ] Watch `.claude/` and `.orqa/` directories for external modifications
- [ ] Refresh artifact list and viewer when files change on disk
- [ ] Debounce rapid changes (editor auto-save)
