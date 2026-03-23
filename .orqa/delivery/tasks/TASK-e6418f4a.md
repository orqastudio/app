---
id: "TASK-e6418f4a"
type: task
title: "Add Decision type to artifact framework"
description: "Added Decision (AD-NNN) type schema to artifact-framework.md, decision creation section to artifact-workflow.md, decision enforcement to RULE-7b770593, and decision directory to orchestrator resources."
status: "completed"
created: "2026-03-08"
updated: "2026-03-08"
acceptance:
  - "artifact-framework.md defines the Decision type with schema and status workflow"
  - "artifact-workflow.md includes Decision creation guidance"
  - "RULE-7b770593 enforces Decision status transitions and supersession rules"
  - "RULE-65973a88 references individual decision artifacts as source of truth"
relationships:
  - target: "EPIC-766e2afa"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-953f7ac3"
    type: "depended-on-by"
  - target: "TASK-da84a27e"
    type: "depended-on-by"
---
## What

Established the Decision artifact type as a first-class citizen in the governance framework.

## How

Updated artifact-framework.md with Decision schema, artifact-workflow.md with creation workflow, [RULE-7b770593](RULE-7b770593) with lifecycle enforcement, and [RULE-65973a88](RULE-65973a88) with source-of-truth directive.

## Verification

Decision type is defined, enforceable, and discoverable through the artifact framework.