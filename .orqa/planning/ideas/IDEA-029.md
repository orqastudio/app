---
id: IDEA-029
title: Git-Based Artifact Change Tracking
description: |
  Use git as the change tracking layer for all governance artifacts. Git is initialised as part of project setup (not optional). Remote origins are optional for multi-user/multi-device portability. The UI provides a focused artifact history viewer — not a git client.
status: captured
created: "2026-03-09"
updated: "2026-03-09"
pillars:
  - PILLAR-001
  - PILLAR-002
research-needed:
  - Git init as part of project setup — UX for existing vs new repos
  - Artifact history viewer — commit log filtered to .orqa/ files
  - Per-file diff viewer for individual artifacts
  - Agent attribution via Co-Authored-By parsing
  - Auto-commit strategy — when should the app commit artifact changes?
  - Remote origin discovery and optional sync UI
promoted-to: null
---## Design Principles

### Git is local infrastructure, not an integration

- Every project gets `git init` at setup if not already initialised
- Existing `.git/` is respected — no reconfiguration
- Remote origin is NEVER required
- Remote is an optional portability mechanism for multi-user or cross-device

### What the UI provides (focused artifact viewer, not a git client)

- **Artifact history**: Commit log filtered to a specific `.orqa/` file
- **Diff viewer**: Side-by-side or inline diff for artifact changes
- **Attribution**: Who/what made each change (human vs agent via Co-Authored-By)
- **Timeline**: Visual history of how a rule, epic, or lesson evolved
- **Change context**: Commit message as the "why" for each change

### What the UI does NOT provide

- Branch management
- Merge/rebase
- Push/pull (beyond a simple "sync" button if remote exists)
- Conflict resolution
- Stash management

### Auto-commit strategy (needs research)

When should the app automatically commit artifact changes?

Options:
1. **On save** — every artifact edit creates a micro-commit
2. **On session end** — batch all artifact changes from a session
3. **On explicit action** — user/agent decides when to commit
4. **Hybrid** — auto-commit governance changes, manual for code changes

The right answer probably depends on whether the project is dogfooding
(editing its own code) vs pure governance (no code changes).

### Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | Makes artifact evolution visible — every change has a traceable history with attribution and rationale |
| Learning Through Reflection | Enables understanding of HOW governance evolved over time — which rules changed, why decisions were revised, how lessons were promoted |
