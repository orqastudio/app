---
id: TASK-f9f933b5
title: Connect documentation to artifact graph via relationships
description: "Backfill relationships on documentation files, skills, and agents so documentation participates in graph traversal. Skills link to their documentation, docs link to the rules/decisions they describe, agents link to their grounding."
status: completed
priority: P1
scoring:
  impact: 5
  urgency: 5
  complexity: 3
  dependencies: 5
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Every skill that references documentation in body text has an informed-by relationship to that doc
  - Every documentation page that describes a rule or decision has a documents relationship
  - Every agent definition has grounded-by relationships to its grounding documents
  - Wireframe docs linked to relevant epics via docs-required
  - "Orphaned documentation rate drops from 55% to under 15%"
  - All relationships are bidirectional (inverses present on both sides)
  - make verify passes clean
relationships:
  - target: EPIC-915291e7
    type: delivers
    rationale: Phase 2 — graph connectivity is the structural foundation for grounding injection
  - target: TASK-ca282ce0
    type: depends-on
  - target: TASK-269b3f8f
    type: depended-on-by
---

## Scope

### Skill → Documentation Relationships

For each skill in `.orqa/process/skills/`, check if the body text references documentation paths or DOC-NNN IDs. For each reference, add an `informed-by` relationship in the skill's frontmatter and a `informs` inverse on the documentation.

### Documentation → Rules/Decisions Relationships

For each documentation page, check if it describes or implements a rule (RULE-NNN) or decision (AD-NNN). Add `informs`/`informed-by` relationships.

### Agent → Grounding Relationships

Add `grounded-by` relationships from each agent definition to its grounding document(s):

| Agent | Grounding Docs |
|-------|---------------|
| Orchestrator | product-purpose, delegation reference |
| Implementer | code-principles |
| Reviewer | code-principles |
| Writer | artifact-principles |
| Designer | design-principles |
| Researcher | research-principles |
| Planner | product-purpose |
| Governance Steward | artifact-principles, product-purpose |

### Wireframes → Epics

Link wireframe docs (DOC-99d7fbe5 through DOC-53b07f55) to relevant epics via `docs-required` where appropriate.

### Verification

Run `make verify-links` after all changes to confirm zero broken references and full bidirectional coverage.
