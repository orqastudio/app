---
id: TASK-ec1e58dc
type: task
title: "Broken link detection with line-level positioning"
description: "Enhance broken link detection to report diagnostics at the exact line and column where the broken artifact reference appears in frontmatter or body."
status: ready
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: EPIC-3ecc76ff
    type: delivers
    rationale: "Broken link detection with precise positioning improves the existing capability"
  - target: TASK-061b5052
    type: depends-on
    rationale: "Needs plugin schemas to know valid artifact ID prefixes"
---

# Broken Link Detection with Line-Level Positioning

## What to Implement

Broken link detection already works at two levels: `check_relationship_targets()` in the LSP (text-level, uses local graph) and `BrokenLink` checks from the daemon's `/validate` endpoint (graph-level). Both currently report at line 0 or approximate positions.

### Steps

1. **Improve line positioning in `check_relationship_targets()`** — when a relationship target is not found in the graph, report the diagnostic at the exact line in frontmatter where the `target:` field appears, not at line 0.

2. **Map daemon BrokenLink diagnostics to source lines** — the daemon returns artifact IDs; the LSP must find where that ID appears in the document to position the diagnostic.

3. **Detect broken links in body content** — scan markdown body for `[TYPE-XXXXXXXX](TYPE-XXXXXXXX)` patterns and validate each referenced ID exists in the graph.

4. **Use daemon `/validate` for authoritative checks** — text-level checks provide fast feedback; daemon checks provide authoritative results when the full graph is available.

## Acceptance Criteria

- [ ] Broken relationship targets in frontmatter produce diagnostics at the exact `target:` line
- [ ] Broken artifact references in markdown body produce diagnostics at the reference position
- [ ] Diagnostics distinguish between "artifact not found" and "artifact exists but type mismatch"
- [ ] Line/column positions are accurate (not line 0)
- [ ] No `unwrap()` / `expect()` / `panic!()` in new code
- [ ] `make lint-backend` passes with zero warnings
