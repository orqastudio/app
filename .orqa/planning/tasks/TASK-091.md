---
id: TASK-091
title: Convert monolithic decisions doc to individual artifacts
description: Converted the monolithic docs/architecture/decisions.md from full content to an index table, creating 20 individual AD-NNN.md artifacts with complete YAML frontmatter and cross-references.
status: done
created: "2026-03-08"
updated: "2026-03-08"
epic: EPIC-032
depends-on:
  - TASK-090
acceptance:
  - 20 individual AD-NNN.md files exist with complete frontmatter
  - Each decision has id, title, status, created, updated fields
  - Cross-references are populated and bidirectional
---
## What

Migrated architecture decisions from a single monolithic document to individual first-class artifacts.

## How

Extracted each decision section from the monolithic file into its own AD-NNN.md with structured YAML frontmatter. Populated cross-references from the original content.

## Verification

All 20 decisions exist as individual artifacts with valid frontmatter and bidirectional cross-references.
