---
id: EPIC-056
title: "Repository Directory Reorganisation"
description: |
  Restructure the repository so that frontend, backend, sidecar, and debugger code
  each live in their own top-level directory. Watchers then target only their specific
  directory, eliminating unnecessary rebuilds when unrelated files change.
status: done
priority: P1
created: "2026-03-12"
updated: "2026-03-12"
deadline: null
milestone: MS-001
horizon: null
pillars:
  - PILLAR-001
depends-on: []
blocks: []
research-refs:
  - RES-044
docs-required:
  - RES-044
docs-produced: []
scoring:
  dogfood-value: 5
  user-facing: 1
  foundation: 5
---

## Context

Both Vite and Rust file watchers currently watch the entire repository root. This causes
unnecessary rebuilds — editing a Rust file triggers Vite HMR, editing a Svelte file
triggers Cargo recompilation, and editing `.orqa/` governance artifacts triggers both.
The root cause is that source code for all layers lives at the top level without clear
directory boundaries that watchers can target.

**Current structure:**

```
orqa-studio/
  src-tauri/          # Rust backend
  ui/                 # Svelte frontend
  sidecar/            # Bun sidecar
  scripts/            # Dev controller + dashboard
  .orqa/              # Governance artifacts
  ...config files...
```

**Proposed structure:**

```
orqa-studio/
  backend/
    src-tauri/        # Rust backend (moved)
  ui/
    src/              # Frontend source (current ui/ contents nested)
  sidecars/
    claude-agentsdk-sidecar/     # Sidecar (moved from sidecar/)
  debugger/
    dev.mjs           # Dev controller (moved from scripts/)
    dev-dashboard.html
  .orqa/              # Governance (unchanged)
  ...config files...
```

Watchers can then be scoped:
- Vite watches `ui/` only
- Cargo watches `backend/` only
- Neither triggers on `.orqa/`, `debugger/`, or `sidecars/` changes

## Implementation Design

This is a large cross-cutting reorganisation. Every path reference in config files, import
statements, build scripts, and documentation must be updated atomically. The research task
([RES-044](RES-044)) must be completed first to map all affected references.

### Phase 1: Research (TASK-251)

Comprehensive audit of every file and config that references current directory paths.
Map all cross-cutting concerns before any moves happen.

### Phase 2: Implementation

TBD — tasks will be created after research findings are reviewed. Likely approach:
sequential directory moves with atomic config updates per move, verified by `make check`
after each step.

## Tasks

### Phase 1: Research

| ID | Title |
|----|-------|
| [TASK-251](TASK-251) | Research: cross-cutting concerns of directory restructure |

### Phase 2: Implementation

| ID | Title |
|----|-------|
| [TASK-252](TASK-252) | Update documentation paths for directory reorganisation |
| [TASK-253](TASK-253) | Move sidecar to sidecars/claude-agentsdk-sidecar/ |
| [TASK-254](TASK-254) | Move backend to backend/src-tauri/ |
| [TASK-255](TASK-255) | Nest frontend source into ui/src/ |
| [TASK-256](TASK-256) | Move dev controller to debugger/ |

### Phase 3: Verification

| ID | Title |
|----|-------|
| [TASK-257](TASK-257) | Full integration test of reorganised repository |

## Out of Scope

- Changing the Tauri app name or package identity
- Restructuring code within `backend/src-tauri/src/` (internal Rust module layout stays as-is)
- Restructuring code within `ui/lib/` (internal frontend layout stays as-is)
- CI/CD pipeline changes (no CI exists yet)
