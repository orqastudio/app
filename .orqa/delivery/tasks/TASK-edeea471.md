---
id: TASK-edeea471
type: task
title: Remove Plan type from artifact framework
description: "Removes the Plan artifact type from the framework schema and replaces it with a Research schema, updating the traceability chain to Task → Epic → Milestone."
status: completed
created: 2026-03-08
updated: 2026-03-08
assignee: AGENT-bbad3d30
acceptance:
  - No Plan type definition in artifact-framework.md
  - Research schema added with draft/complete/surpassed workflow
  - Traceability chain updated to Task → Epic → Milestone
relationships:
  - target: EPIC-57dd7d4c
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-dc793265
    type: depended-on-by
---
## What

Remove the Plan artifact type from artifact-framework.md. Add a Research schema. Update the traceability chain documentation to reflect Task → Epic → Milestone with research-refs for linking to investigation documents.

## Outcome

Completed by background agent. The Plan type was removed, Research schema added, and traceability chain updated. This task was part of the schema simplification sprint that was later continued directly by the orchestrator (no delegation) after the "don't refactor agentic structure while using agents" lesson was learned.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.