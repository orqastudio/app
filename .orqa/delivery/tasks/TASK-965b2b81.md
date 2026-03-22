---
id: "TASK-965b2b81"
title: "Create RULE-c95f4444, RULE-f9d0279c, RULE-7f416d7d"
description: "Create three new rules documenting the enforcement layers: data persistence

  boundaries, automated skill injection, and tooling ecosystem management.\n"
status: "completed"
created: "2026-03-11"
updated: "2026-03-12"
acceptance:
  - "RULE-c95f4444, RULE-f9d0279c, RULE-7f416d7d created with valid frontmatter"
  - "All three rules pass schema validation"
  - "Rules reference each other and related rules appropriately"
relationships:
  - target: "EPIC-4440cdd4"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-49c63248"
    type: "depends-on"
  - target: "TASK-4f45e5b9"
    type: "depends-on"
  - target: "TASK-34007190"
    type: "depended-on-by"
  - target: "TASK-d6727d2f"
    type: "depended-on-by"
---
## What

Three new governance rules:
- **[RULE-c95f4444](RULE-c95f4444) (data persistence)**: Documents which data belongs in SQLite vs
  file-based artifacts vs ephemeral state
- **[RULE-f9d0279c](RULE-f9d0279c) (skill injection)**: Documents the automated skill injection
  system — when skills are injected, deduplication, path-to-skill mapping
- **[RULE-7f416d7d](RULE-7f416d7d) (tooling ecosystem)**: Documents that OrqaStudio manages linter
  config to match documented standards, not replicate linter functionality

## How

1. Create `.orqa/process/rules/[RULE-c95f4444](RULE-c95f4444).md` with proper frontmatter
2. Create `.orqa/process/rules/[RULE-f9d0279c](RULE-f9d0279c).md` with proper frontmatter
3. Create `.orqa/process/rules/[RULE-7f416d7d](RULE-7f416d7d).md` with proper frontmatter
4. Follow existing rule format (schema-compliant frontmatter, body sections)

## Verification

- All three rules pass schema validation
- Each rule has clear enforcement entries where applicable
- Rules reference each other and related rules appropriately
