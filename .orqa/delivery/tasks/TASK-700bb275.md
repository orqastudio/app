---
id: TASK-700bb275
title: "Write P1 how-to guides (plugin SDK, Rust testing, frontend testing)"
description: "Author the three P1 how-to guides that are new content deliverables for the reorganised documentation structure. Each guide must have complete frontmatter with a DOC ID, proper relationships, and substantive content."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 2
created: 2026-03-14
updated: 2026-03-14
acceptance:
  - "how-to/plugin-sdk.md created with SDK overview, plugin.json schema, hooks, and commands sections"
  - "how-to/testing-rust.md created with Rust test patterns, cargo test usage, and coverage guidance"
  - "how-to/testing-frontend.md created with Vitest patterns, component testing, and store testing patterns"
  - All three guides have frontmatter with DOC IDs and relationships fields
relationships:
  - target: EPIC-2a6e2567
    type: delivers
    rationale: New how-to guide content deliverables for the reorganised documentation
  - target: TASK-96db8fb0
    type: depends-on
---
