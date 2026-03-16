---
id: TASK-077
title: Broken link styling and path validation
description: Style unresolvable artifact IDs as broken links with warning colour and broken-link icon. Validate docs-required/docs-produced paths against disk during scan.
status: completed
created: 2026-03-10
updated: 2026-03-10
assignee: AGENT-002
acceptance:
  - ArtifactLink renders broken state when artifactGraph.resolve(id) returns undefined
  - Broken links use broken-link Lucide icon and app warning colour token
  - docs-required/docs-produced paths validated against disk — broken paths flagged visually
  - Working links unchanged in appearance
relationships:
  - target: EPIC-048
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-083
    type: depends-on
  - target: SKILL-030
    type: grounded-by
  - target: SKILL-031
    type: grounded-by
  - target: TASK-338
    type: depended-on-by
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
