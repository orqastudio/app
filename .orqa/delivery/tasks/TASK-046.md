---
id: TASK-046
title: Epic title naming convention cleanup
description: "Renames all epic titles that use process words instead of describing outcomes, and adds a naming convention to the artifact lifecycle rule to prevent recurrence."
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-003
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
relationships:
  - target: EPIC-043
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-333
    type: depended-on-by
---
## Findings Addressed

- **F20**: Epic titles contain process words instead of describing outcomes

## Examples to Fix

- "UAT Round 1 — Dogfood Readiness Verification" → "Dogfood Readiness Verification"
- Review all EPIC-*.md files for similar patterns

## Enforcement

Add to artifact-lifecycle.md under Epic creation standards:
> Epic titles describe what is achieved, not how work is organised. Avoid process words (UAT, Phase, Sprint, Round) unless they describe the actual deliverable.

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
