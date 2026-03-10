---
id: TASK-046
title: Epic title naming convention cleanup
description: Renames all epic titles that use process words instead of describing outcomes, and adds a naming convention to the artifact lifecycle rule to prevent recurrence.
status: done
created: 2026-03-09
updated: 2026-03-09
epic: EPIC-043
assignee: agent-maintainer
skills:
  - orqa-governance
scope:
  - .orqa/planning/epics/
  - .orqa/governance/rules/artifact-lifecycle.md
acceptance:
  - All epic titles describe outcomes/content
  - not process activities
  - No process words (UAT
  - Phase
  - Sprint
  - Round
  - Audit) in epic titles unless they describe the actual content
  - artifact-lifecycle.md updated with naming convention for epic titles
  - IMPL-014 lesson referenced
---
## Findings Addressed

- **F20**: Epic titles contain process words instead of describing outcomes

## Examples to Fix

- "UAT Round 1 — Dogfood Readiness Verification" → "Dogfood Readiness Verification"
- Review all EPIC-*.md files for similar patterns

## Enforcement

Add to artifact-lifecycle.md under Epic creation standards:
> Epic titles describe what is achieved, not how work is organised. Avoid process words (UAT, Phase, Sprint, Round) unless they describe the actual deliverable.
