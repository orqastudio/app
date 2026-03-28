---
id: "TASK-5fdbf116"
type: "task"
title: "Verify full pipeline and archive original repos"
status: archived
description: "End-to-end verification: clean clone, make install, make check, make build. Then archive original GitHub repos with redirect READMEs."
relationships:
  - target: "EPIC-2f720d43"
    type: "delivers"
    rationale: "Phase 1 — monorepo consolidation"
  - target: "TASK-a1ef2aad"
    type: "depends-on"
    rationale: "Install pipeline must be updated first"
  - target: "TASK-58a0bdf0"
    type: "depends-on"
    rationale: "Licensing must be in place"
acceptance:
  - "git clone <monorepo> && make install succeeds from scratch"
  - "make check passes (lint, typecheck, test)"
  - "make build produces production Tauri bundle"
  - "Original GitHub repos archived with README pointing to monorepo"
  - "No broken cross-references in .orqa/ artifacts"
---

## Scope

### Verification checklist

1. Fresh clone on a clean machine (or clean directory)
2. `make install` — npm install (workspaces), cargo fetch, plugin sync
3. `make check` — format, lint, typecheck, test (frontend + backend)
4. `make build` — Tauri production build
5. `make dev` — dev server starts and app launches

### Archive original repos

For each of the 30 GitHub repos:

1. Update README to say "This repo has moved to \<monorepo-url\>"
2. Archive the repo via `gh repo archive`
3. Do NOT delete — archived repos remain accessible for historical links

### Update references

- `.orqa/` artifacts referencing old repo URLs
- Registry entries pointing to individual repos
- CI workflows referencing individual repos
- CONTRIBUTING.md with updated contribution instructions
