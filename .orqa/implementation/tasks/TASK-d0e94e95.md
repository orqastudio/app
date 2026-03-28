---
id: TASK-d0e94e95
type: task
title: "Hover provider for artifact references"
description: "Implement LSP hover provider that shows artifact metadata (title, type, status, description, file path) when hovering over artifact IDs in frontmatter or body."
status: ready
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: EPIC-3a3e5aea
    type: delivers
    rationale: "Hover is a core editor intelligence feature"
  - target: TASK-47225043
    type: depends-on
    rationale: "Needs plugin schemas and daemon endpoints for artifact metadata"
  - target: TASK-d423f4f7
    type: depends-on
    rationale: "Shares the GET /artifacts and GET /artifact/:id daemon endpoints"
---

# Hover Provider for Artifact References

## What to Implement

The LSP currently does not register a hover provider. This task adds one that shows rich artifact metadata when hovering over artifact IDs.

### Steps

1. **Register hover provider** in `initialize` response — set `hover_provider: Some(true)`.

2. **Add `GET /artifact/:id` endpoint to the daemon** — returns full metadata for a single artifact (title, type, status, description, file path, relationships).

3. **Implement `textDocument/hover` handler** — detect if the cursor is on an artifact ID pattern (`&lt;TYPE&gt;-&lt;hex8&gt;`), fetch metadata from the daemon, and return formatted hover content.

4. **Format hover content as Markdown** — show title, type badge, status, description excerpt, file path, and relationship count.

5. **Handle hover in different contexts**:
   - Frontmatter `target:` values — show the referenced artifact's metadata
   - Frontmatter `id:` field — show this artifact's own metadata (useful for quick reference)
   - Body `[&lt;TYPE&gt;-&lt;hex8&gt;](&lt;TYPE&gt;-&lt;hex8&gt;)` links — show the referenced artifact's metadata

## Acceptance Criteria

- [ ] Hover provider is registered in LSP capabilities
- [ ] Daemon exposes `GET /artifact/:id` returning full artifact metadata
- [ ] Hovering over an artifact ID in frontmatter shows title, type, status, description
- [ ] Hovering over an artifact ID in body links shows the same metadata
- [ ] Hover content is formatted as readable Markdown
- [ ] Unknown/broken artifact IDs show "Artifact not found" hover
- [ ] No `unwrap()` / `expect()` / `panic!()` in new code
- [ ] `make lint-backend` passes with zero warnings
