---
id: IDEA-068
title: AI-assisted artifact backfill tooling
description: "Tooling that reads artifacts, analyses content against other artifacts, proposes relationship connections, presents for human review, and batch-updates frontmatter. Built as a precursor to the AD-042 migration, reusable for any future schema evolution."
status: review
created: 2026-03-12
updated: 2026-03-13
horizon: active
research-needed:
  - "Should this be a CLI skill, a script, or an app feature?"
  - "How to analyse artifact content to propose relationships — semantic search, keyword matching, or AI classification?"
  - "Batch review UX — how does the human efficiently review and approve/reject proposals?"
  - Frontmatter update mechanism — safe in-place editing of YAML frontmatter without corrupting markdown body
relationships:
  - target: MS-001
    type: delivers
---

## Motivation

[EPIC-058](EPIC-058) Phase 2 requires backfilling relationships on ~150 artifacts. Doing this manually is error-prone and slow. Doing it with ad-hoc AI prompting loses consistency. Purpose-built tooling that can read an artifact, propose connections based on content analysis, and present them for human review makes the migration reliable AND creates a reusable capability for any future schema evolution.

## Sketch

The tool would:
1. Read an artifact and its schema
2. Identify required relationship types for that artifact type
3. Search other artifacts (via semantic search or content analysis) for likely connections
4. Propose relationships with draft rationale
5. Present proposals for human review (approve / reject / edit rationale)
6. Update the artifact's frontmatter with approved relationships
7. Track what's been processed and what's pending

This is immediately useful for [EPIC-058](EPIC-058) and becomes a permanent capability for governance maintenance.
