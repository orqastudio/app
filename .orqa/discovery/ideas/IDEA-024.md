---
id: IDEA-024
title: "Git Integration & Worktree-Aware Workspace"
description: "Git awareness for OrqaStudio including branch status, worktree visibility, and version control operations surfaced through the app UI."
status: review
created: 2026-03-07
updated: 2026-03-13
horizon: active
research-needed:
  - "What git operations does the user need visibility into from the app?"
  - "How should branch status, diffs, and commit history surface in the UI?"
  - "How does the app detect and display active worktrees for parallel agent work?"
  - "Should the app manage git operations or just provide visibility?"
  - "How do branches/worktrees relate to the artifact lifecycle (tasks, epics)?"
  - "What git state is relevant for dogfooding (uncommitted changes, stale worktrees, merge conflicts)?"
relationships:
  - target: PILLAR-001
    type: grounded
  - target: PERSONA-001
    type: benefits
---
## Problem

The app has zero git awareness. During dogfooding, all version control operations happen in the terminal — branching, committing, worktree management, merge conflict resolution, stale worktree cleanup. This is a significant gap for the "use the app instead of the terminal" dogfooding goal.

Worktrees are particularly important because the CLI uses them to give agents isolated copies of the repo for parallel work. The app can't show what agents are working on, which worktrees exist, or what state they're in.

## Why It Matters for Dogfooding

The gate question for [MS-001](MS-001) is: "Can we use this app instead of the terminal?" Git integration is a major piece of the CLI workflow. Without it, the user still needs the terminal for:

- Seeing uncommitted changes, branch status, stale stashes
- Seeing active worktrees and their branches
- Understanding what agents are doing in parallel
- Committing completed work
- Merging worktree branches back to main
- Cleaning up stale worktrees

## Possible Scope Levels

1. **Status visibility** — Git status panel showing branch, uncommitted changes, stashes, worktree list
2. **Worktree awareness** — Detect worktrees, show associated tasks/epics, highlight stale ones
3. **Basic operations** — Commit, branch switch, worktree create/merge/cleanup via Tauri commands
4. **Full integration** — Diff viewer, merge conflict resolution, commit history, interactive staging

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | Makes parallel agent work visible and structured — the user can see what's happening across all worktrees instead of relying on terminal commands |
| Learning Through Reflection | N/A |
