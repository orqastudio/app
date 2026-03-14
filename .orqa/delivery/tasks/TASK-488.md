---
id: TASK-488
title: "Write P1 how-to guides (plugin SDK, Rust testing, frontend testing)"
description: "Author the three P1 how-to guides that are new content deliverables for the reorganised documentation structure. Each guide must have complete frontmatter with a DOC ID, proper relationships, and substantive content."
status: done
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-075
depends-on:
  - TASK-486
acceptance:
  - how-to/plugin-sdk.md created with SDK overview, plugin.json schema, hooks, and commands sections
  - how-to/testing-rust.md created with Rust test patterns, cargo test usage, and coverage guidance
  - how-to/testing-frontend.md created with Vitest patterns, component testing, and store testing patterns
  - All three guides have frontmatter with DOC IDs and relationships fields
relationships:
  - target: EPIC-075
    type: delivers
    rationale: New how-to guide content deliverables for the reorganised documentation
  - target: EPIC-075
    type: belongs-to
    rationale: Task belongs to this epic
---
