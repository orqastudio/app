---
id: RULE-032
slug: artifact-schema-compliance
layer: canon
status: active
scope: artifact
title: "Artifact Schema Compliance"
description: "Every artifact's YAML frontmatter must match the canonical schema defined in artifact-framework.md. Missing required fields or extra undefined fields are commit rejections."
created: 2026-03-10
updated: 2026-03-10
---

Every artifact in `.orqa/` must have YAML frontmatter that matches the canonical schema defined in `.orqa/documentation/product/artifact-framework.md`. This rule is enforced by pre-commit hook validation and (in the app) by the artifact scanner.

## Source of Truth

The per-type field tables in `artifact-framework.md` are the single source of truth for:

- Which fields are required vs optional
- Valid values for enum fields (status, priority, layer)
- Field naming (e.g. `pillars` not `pillar`, `created` not `date`)
- Field ordering convention

## Required Fields by Type

| Type | Required Fields |
|------|----------------|
| **Pillar** | id, title, status, description, test-questions, created, updated |
| **Milestone** | id, title, status, description, created, updated, gate |
| **Epic** | id, title, status, priority, milestone, pillars, description, created, updated, scoring |
| **Task** | id, title, status, epic, description, created, updated |
| **Idea** | id, title, status, pillars, description, created, updated |
| **Lesson** | id, title, status, description, created, updated, recurrence |
| **Research** | id, title, status, description, created, updated |
| **Decision** | id, title, status, description, created, updated |
| **Rule** | id, slug, layer, status, scope, title, description, created, updated |

## Valid Status Values

| Type | Valid Statuses |
|------|--------------|
| Pillar | `active`, `inactive` |
| Milestone | `planning`, `active`, `complete` |
| Epic | `draft`, `ready`, `in-progress`, `review`, `done` |
| Task | `todo`, `in-progress`, `done` |
| Idea | `captured`, `exploring`, `shaped`, `promoted`, `archived` |
| Lesson | `active`, `recurring`, `promoted` |
| Research | `draft`, `complete`, `surpassed` |
| Decision | `proposed`, `accepted`, `superseded`, `deprecated` |
| Rule | `active`, `inactive` |

## What Is Forbidden

- Creating an artifact with missing required fields
- Using field names not defined in the schema (e.g. `tags`, `category`, `type`, `date`)
- Using invalid status values (e.g. `done` on an Idea)
- Using singular `pillar` instead of plural `pillars`
- Duplicate frontmatter keys (e.g. two `status:` lines)

## Enforcement

1. **Pre-commit hook** — `.githooks/pre-commit` validates frontmatter of staged `.orqa/` files before allowing the commit
2. **Agent self-compliance** — agents check the schema before creating or modifying artifacts
3. **App scanner** (future) — the artifact scanner validates frontmatter during scans and reports violations in the UI

## Related Rules

- RULE-004 (artifact-lifecycle) — status transitions and promotion gates
- RULE-003 (artifact-config-integrity) — config paths must match disk
- RULE-027 (structure-before-work) — artifacts must exist before implementation
