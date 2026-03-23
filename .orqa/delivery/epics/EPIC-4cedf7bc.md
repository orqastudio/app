---
id: EPIC-4cedf7bc
type: epic
title: "Artifact ID migration + mandatory skill documentation"
description: "Implements AD-05756de7 (type-prefixed 8-char hex IDs) and AD-6f6ff517 (skills require human-facing docs). Migrates all existing artifact IDs, adds ID generation tooling, enforces synchronised-with constraint on skills, creates missing documentation artifacts, and adds agent skills for working with the new ID format."
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: MS-654badde
    type: fulfils
  - target: AD-05756de7
    type: driven-by
  - target: AD-6f6ff517
    type: driven-by
  - target: TASK-0b703fd2
    type: delivered-by
  - target: TASK-ad41ea9b
    type: delivered-by
  - target: TASK-db0e3e3c
    type: delivered-by
  - target: TASK-2eeb847c
    type: delivered-by
  - target: TASK-8f8b1dba
    type: delivered-by
  - target: TASK-cae3c8c7
    type: delivered-by
  - target: TASK-bd4e7250
    type: delivered-by
---

# EPIC-4cedf7bc: Artifact ID Migration + Mandatory Skill Documentation

## Goal

Two related changes that improve artifact identity and governance transparency:

1. **AD-05756de7** — Migrate artifact IDs from sequential `TYPE-NNN` to location-independent `TYPE-XXXXXXXX` (8-char hex). Eliminates rename cascades when artifacts move between core, plugins, and projects.

2. **AD-6f6ff517** — Enforce that every skill has at least one `synchronised-with` relationship to a human-facing documentation artifact. Makes the app's capabilities self-documenting.

## Deliverables

### ID Migration (AD-05756de7)
1. ID generation utilities in Rust and TypeScript
2. `orqa id generate <type>` CLI command
3. Bulk migration script (old sequential → new hex, updates all references)
4. Integrity scanner updated to accept both formats
5. Agent skill documenting the new ID format and how to use it

### Skill Documentation (AD-6f6ff517)
1. `core.json` constraint: `synchronised-with` required on skills with `minCount: 1`
2. Documentation artifacts created for all existing undocumented skills
3. Plugin skills documented (one doc per plugin covering all its skills)
4. Core skills documented (grouped by domain)
5. Integrity scanner reports missing skill docs as errors

### Verification Tooling
1. `orqa enforce` catches missing skill docs and invalid ID formats
2. Pre-commit hook validates new artifact IDs are hex format
3. Agent skill for creating artifacts with correct hex IDs