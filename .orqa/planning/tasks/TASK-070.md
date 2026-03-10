---
id: TASK-070
title: Document body templates in artifact-framework.md and schema.json
description: Add bodyTemplate definitions to each artifact type's schema.json and update artifact-framework.md with the canonical body structure for each type.
status: done
created: "2026-03-10"
updated: "2026-03-10"
epic: EPIC-048
assignee: AGENT-003
skills:
  - SKILL-011
scope:
  - .orqa/documentation/product/artifact-framework.md
  - .orqa/**/schema.json
acceptance:
  - Each schema.json (except research) has a bodyTemplate key listing required section headings
  - artifact-framework.md updated with body template documentation for all 9 types
  - Schema bodyTemplate format is machine-parseable (array of heading strings)
---
## What

Define the minimum required body sections for each artifact type in a machine-readable format within schema.json, and document them in artifact-framework.md.

## How

Add a `bodyTemplate` key to each schema.json containing an array of objects with `heading` (required section name) and `required` (boolean). Update artifact-framework.md with human-readable documentation of each template.

## Verification

- Every schema.json (except research) has a `bodyTemplate` array
- artifact-framework.md documents all templates with examples
- Templates match the patterns identified in [RES-032](RES-032)
