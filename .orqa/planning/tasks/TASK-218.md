---
id: TASK-218
title: Add sources field to research schema for web references
description: Add an optional sources field to the research artifact schema for tracking external URLs and references that informed the research.
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-053
depends-on: []
scope:
  - Add `sources` field to .orqa/planning/research/schema.json
  - Sources is an array of objects with url and description
  - Optional field — not all research uses external sources
  - Update artifact-framework.md Research schema section
acceptance:
  - Research schema accepts sources array
  - Existing research docs without sources still validate
  - Sources field documented in artifact framework
---
## What

Research documents often reference external sources (library docs, GitHub issues, blog posts, specifications). Currently these are just inline markdown links in the body. Adding a structured `sources` field to frontmatter makes them graph-queryable — the system can trace which external knowledge informed which decisions.

## How

1. Add `sources` array to `.orqa/planning/research/schema.json`
2. Each source has required `url` and `description`, optional `tier` (T1-T4)
3. Update artifact-framework.md Research schema section

## Verification

- Research schema accepts sources array with url, description, tier
- Existing research docs without sources still validate
- Artifact framework docs updated with sources field
