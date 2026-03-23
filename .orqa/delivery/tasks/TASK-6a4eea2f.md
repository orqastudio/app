---
id: TASK-6a4eea2f
type: task
title: Add body template linting to pre-commit hook
description: Extend validate-schema.mjs to check that artifact bodies contain required section headings defined in the bodyTemplate schema key.
status: completed
created: 2026-03-10
updated: 2026-03-10
assignee: AGENT-1dab5ebe
acceptance:
  - Pre-commit hook reads bodyTemplate from schema.json for each artifact type
  - Hook checks that required headings exist in the markdown body
  - Validation errors list missing required sections
  - Research artifacts are exempt (intentionally freeform)
  - Optional sections are not enforced
relationships:
  - target: EPIC-0a8a5e72
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-f950424e
    type: depends-on
  - target: TASK-8c0e5f1d
    type: depended-on-by
  - target: TASK-12eec0f3
    type: depended-on-by
---
## What

The pre-commit hook already validates frontmatter against schema.json. Extend it to also validate body structure against the bodyTemplate definitions.

## How

After frontmatter validation passes, extract the markdown body (below closing `---`). For each required heading in bodyTemplate, check that `## Heading` exists in the body. Report missing required sections as errors.

## Verification

- Create a test artifact missing a required section — hook rejects it
- Create a valid artifact — hook passes
- Research artifacts with no template — hook skips body validation