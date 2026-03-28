---
id: TASK-2d104e1f
type: task
title: "Go to definition for artifact IDs"
description: "Implement LSP definition provider that navigates to the source file of an artifact when the user invokes go-to-definition on an artifact ID."
status: ready
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: EPIC-3a3e5aea
    type: delivers
    rationale: "Go-to-definition is a core editor navigation feature"
  - target: TASK-47225043
    type: depends-on
    rationale: "Needs daemon endpoints to resolve artifact IDs to file paths"
  - target: TASK-d423f4f7
    type: depends-on
    rationale: "Shares the GET /artifacts daemon endpoint for ID-to-path resolution"
---

# Go to Definition for Artifact IDs

## What to Implement

The LSP currently does not register a definition provider. This task adds one that navigates to the source file when the user invokes go-to-definition (F12 / Ctrl+Click) on an artifact ID.

### Steps

1. **Register definition provider** in `initialize` response — set `definition_provider: Some(true)`.

2. **Implement `textDocument/definition` handler** — detect if the cursor is on an artifact ID pattern, resolve the ID to a file path via the daemon's `GET /artifacts` endpoint (or a cached version), and return a `Location` pointing to line 1 of the target file.

3. **Handle definition in different contexts**:
   - Frontmatter `target:` values — navigate to the referenced artifact
   - Frontmatter `depends-on:` values — navigate to the dependency
   - Body `[\<TYPE\>-<hex8>](\<TYPE\>-<hex8>)` links — navigate to the referenced artifact

4. **Cache artifact-to-path mapping** — avoid hitting the daemon on every go-to-definition request. Refresh the cache on `didSave` events or when the daemon signals a graph change.

## Acceptance Criteria

- [ ] Definition provider is registered in LSP capabilities
- [ ] Go-to-definition on a frontmatter `target:` value opens the referenced artifact file
- [ ] Go-to-definition on a body artifact link opens the referenced artifact file
- [ ] Navigation targets line 1 of the artifact file (the start of frontmatter)
- [ ] Unknown artifact IDs produce no navigation (graceful no-op)
- [ ] Artifact-to-path mapping is cached and refreshed on save events
- [ ] No `unwrap()` / `expect()` / `panic!()` in new code
- [ ] `make lint-backend` passes with zero warnings
