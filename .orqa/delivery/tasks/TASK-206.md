---
id: TASK-206
title: Create RULE-041, RULE-042, RULE-043
description: |
  Create three new rules documenting the enforcement layers: data persistence
  boundaries, automated skill injection, and tooling ecosystem management.
status: done
created: "2026-03-11"
updated: "2026-03-12"
epic: EPIC-052
depends-on:
  - TASK-193
  - TASK-194
---

## What

Three new governance rules:
- **[RULE-041](RULE-041) (data persistence)**: Documents which data belongs in SQLite vs
  file-based artifacts vs ephemeral state
- **[RULE-042](RULE-042) (skill injection)**: Documents the automated skill injection
  system — when skills are injected, deduplication, path-to-skill mapping
- **[RULE-043](RULE-043) (tooling ecosystem)**: Documents that OrqaStudio manages linter
  config to match documented standards, not replicate linter functionality

## How

1. Create `.orqa/process/rules/[RULE-041](RULE-041).md` with proper frontmatter
2. Create `.orqa/process/rules/[RULE-042](RULE-042).md` with proper frontmatter
3. Create `.orqa/process/rules/[RULE-043](RULE-043).md` with proper frontmatter
4. Follow existing rule format (schema-compliant frontmatter, body sections)

## Verification

- All three rules pass schema validation
- Each rule has clear enforcement entries where applicable
- Rules reference each other and related rules appropriately
