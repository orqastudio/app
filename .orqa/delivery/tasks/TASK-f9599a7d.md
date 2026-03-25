---
id: "TASK-f9599a7d"
type: task
title: "Create RULE-09a238ab, RULE-e1f1afc1, RULE-42d17086"
description: "Create three new rules documenting the enforcement layers: data persistence

  boundaries, automated skill injection, and tooling ecosystem management.\n"
status: "completed"
created: "2026-03-11"
updated: "2026-03-12"
acceptance:
  - "RULE-09a238ab, RULE-e1f1afc1, RULE-42d17086 created with valid frontmatter"
  - "All three rules pass schema validation"
  - "Rules reference each other and related rules appropriately"
relationships:
  - target: "EPIC-56940fa8"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-a7bc8368"
    type: "depends-on"
  - target: "TASK-7eabe1a5"
    type: "depends-on"
  - target: "TASK-7382b265"
    type: "depended-on-by"
  - target: "TASK-761fe808"
    type: "depended-on-by"
---
## What

Three new governance rules:
- **[RULE-09a238ab](RULE-09a238ab) (data persistence)**: Documents which data belongs in SQLite vs
  file-based artifacts vs ephemeral state
- **[RULE-e1f1afc1](RULE-e1f1afc1) (skill injection)**: Documents the automated skill injection
  system — when skills are injected, deduplication, path-to-skill mapping
- **[RULE-42d17086](RULE-42d17086) (tooling ecosystem)**: Documents that OrqaStudio manages linter
  config to match documented standards, not replicate linter functionality

## How

1. Create `.orqa/process/rules/[RULE-09a238ab](RULE-09a238ab).md` with proper frontmatter
2. Create `.orqa/process/rules/[RULE-e1f1afc1](RULE-e1f1afc1).md` with proper frontmatter
3. Create `.orqa/process/rules/[RULE-42d17086](RULE-42d17086).md` with proper frontmatter
4. Follow existing rule format (schema-compliant frontmatter, body sections)

## Verification

- All three rules pass schema validation
- Each rule has clear enforcement entries where applicable
- Rules reference each other and related rules appropriately