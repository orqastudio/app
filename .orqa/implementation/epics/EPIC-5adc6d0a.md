---
id: "EPIC-5adc6d0a"
type: epic
title: "Repository Directory Reorganisation"
description: "Restructure the repository so that frontend, backend, sidecar, and debugger code\neach live in their own top-level directory. Watchers then target only their specific\ndirectory, eliminating unnecessary rebuilds when unrelated files change.\n"
status: archived
priority: "P1"
created: "2026-03-12"
updated: "2026-03-12"
deadline: null
horizon: null
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 3
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

## Context

Both Vite and Rust file watchers currently watch the entire repository root. This causes
unnecessary rebuilds — editing a Rust file triggers Vite HMR, editing a Svelte file
triggers Cargo recompilation, and editing `.orqa/` governance artifacts triggers both.
The root cause is that source code for all layers lives at the top level without clear
directory boundaries that watchers can target.

**Current structure:**

```text
orqa-studio/
  src-tauri/          # Rust backend
  ui/                 # Svelte frontend
  sidecar/            # Bun sidecar
  scripts/            # Dev controller + dashboard
  .orqa/              # Governance artifacts
  ...config files...
```

**Proposed structure:**

```text
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
([RES-4dbf04d7](RES-4dbf04d7)) must be completed first to map all affected references.

### Phase 1: Research (TASK-1af78c36)

Comprehensive audit of every file and config that references current directory paths.
Map all cross-cutting concerns before any moves happen.

### Phase 2: Implementation

TBD — tasks will be created after research findings are reviewed. Likely approach:
sequential directory moves with atomic config updates per move, verified by `make check`
after each step.

## Tasks

### Phase 1: Research

| ID | Title |
| ---- | ------- |
| [TASK-1af78c36](TASK-1af78c36) | Research: cross-cutting concerns of directory restructure |

### Phase 2: Implementation

| ID | Title |
| ---- | ------- |
| [TASK-2a557489](TASK-2a557489) | Update documentation paths for directory reorganisation |
| [TASK-a77fcf2e](TASK-a77fcf2e) | Move sidecar to sidecars/claude-agentsdk-sidecar/ |
| [TASK-1a134716](TASK-1a134716) | Move backend to backend/src-tauri/ |
| [TASK-c0b200c5](TASK-c0b200c5) | Nest frontend source into ui/src/ |
| [TASK-b7a65fee](TASK-b7a65fee) | Move dev controller to debugger/ |

### Phase 3: Verification

| ID | Title |
| ---- | ------- |
| [TASK-f47db62a](TASK-f47db62a) | Full integration test of reorganised repository |

## Out of Scope

- Changing the Tauri app name or package identity
- Restructuring code within `backend/src-tauri/src/` (internal Rust module layout stays as-is)
- Restructuring code within `ui/lib/` (internal frontend layout stays as-is)
- CI/CD pipeline changes (no CI exists yet)
