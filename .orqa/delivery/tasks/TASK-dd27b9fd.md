---
id: TASK-dd27b9fd
type: task
title: "Autocomplete for artifact IDs and statuses"
description: "Implement LSP completion provider that suggests artifact IDs, status values, and relationship types based on plugin schemas and the daemon's artifact index."
status: ready
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: EPIC-3ecc76ff
    type: delivers
    rationale: "Autocomplete is a core editor intelligence feature"
  - target: TASK-061b5052
    type: depends-on
    rationale: "Needs plugin schemas for valid statuses and relationship types"
---

# Autocomplete for Artifact IDs and Statuses

## What to Implement

The LSP currently does not register a completion provider. This task adds one that provides context-aware completions in YAML frontmatter.

### Steps

1. **Register completion provider** in `initialize` response — set `completion_provider` with trigger characters `["-", ":"]`.

2. **Add `GET /artifacts` endpoint to the daemon** — returns all artifact IDs with title, type, status, and file path. This powers ID completion.

3. **Implement `textDocument/completion` handler** — determine cursor context (which frontmatter field the cursor is in) and return appropriate completions:
   - **`status:` field** — complete with valid statuses for this artifact type (from plugin schema `status_transitions`)
   - **`target:` field** (in relationships) — complete with all known artifact IDs, filtered by type if a `type:` constraint is nearby
   - **`type:` field** (in relationships) — complete with valid relationship types from plugin schemas
   - **Body content** — complete artifact IDs when typing `[` followed by an ID prefix pattern

4. **Add completion detail** — each completion item includes the artifact title and type as detail/documentation.

## Acceptance Criteria

- [ ] Completion provider is registered in LSP capabilities
- [ ] Daemon exposes `GET /artifacts` returning all artifact summaries
- [ ] `status:` field completions show valid statuses for the current artifact type
- [ ] `target:` field completions show matching artifact IDs with titles
- [ ] `type:` field completions show valid relationship types
- [ ] Body content completions suggest artifact IDs when typing link syntax
- [ ] Completions include detail text (title, type) for each suggestion
- [ ] No `unwrap()` / `expect()` / `panic!()` in new code
- [ ] `make lint-backend` passes with zero warnings
