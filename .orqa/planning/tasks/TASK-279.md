---
id: TASK-279
title: "Remove deprecated fields from schemas and artifacts"
description: "Remove promoted-to from lessons, promoted-from from rules, research-refs from decisions. Data already migrated to relationships."
status: todo
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-058
depends-on: [TASK-278]
assignee: null
docs: []
skills: []
acceptance:
  - "promoted-to removed from lesson schema and all lesson frontmatter"
  - "promoted-from removed from rule schema and all rule frontmatter"
  - "research-refs removed from decision schema and all decision frontmatter"
  - "All data preserved in relationships (grounded-by, observes, informed-by)"
  - "Pre-commit hook passes on all artifacts"
  - "No references to deprecated fields remain in rules, skills, or orchestrator prompt"
rule-overrides: []
---

## What

Clean break — remove the old edge fields that have been fully migrated to the relationships array.

## How

1. Verify all deprecated field data is captured in relationships (spot-check each type)
2. Remove fields from schema.json files (propertyOrder, properties, required)
3. Remove fields from all artifact frontmatter
4. Search codebase for references to deprecated fields (pre-commit hook, app scanner, rules, skills)
5. Update any references found
6. Single commit for the entire removal

## Verification

- Search for `promoted-to`, `promoted-from`, `research-refs` across entire `.orqa/` — zero results
- All artifacts pass schema validation
- Pre-commit hook passes
