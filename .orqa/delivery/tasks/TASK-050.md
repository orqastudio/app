---
id: TASK-050
title: Document governance classification schema
description: "Updates the artifact framework documentation to formally define the three-layer classification model and agent scope categories, and updates the skill enforcement rule to reference layer-aware loading decisions."
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-003
acceptance:
  - artifact-framework.md documents the three-layer classification (canon/project/plugin)
  - artifact-framework.md documents agent scope categories
  - skill-enforcement.md updated with layer-aware loading rules
relationships:
  - target: EPIC-044
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-334
    type: depended-on-by
---
## Changes Needed

1. Add "Governance Classification" section to artifact-framework.md defining:
   - Three layers (canon, project, plugin)
   - Agent scope categories (software-engineering, governance, general)
   - Which frontmatter fields carry classification

2. Update skill-enforcement.md to reference layer classification for loading decisions

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
