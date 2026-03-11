---
id: TASK-090
title: Add Decision type to artifact framework
description: Added Decision (AD-NNN) type schema to artifact-framework.md, decision creation section to artifact-workflow.md, decision enforcement to RULE-004, and decision directory to orchestrator resources.
status: done
created: "2026-03-08"
updated: "2026-03-08"
epic: EPIC-032
depends-on: []
scope:
  - Define Decision type schema in artifact-framework.md
  - Add Decision creation section to artifact-workflow.md
  - Add Decision lifecycle to RULE-004 (status transitions, supersession rules)
  - Update RULE-002 to reference individual AD artifacts
  - Add .orqa/governance/decisions/ to CLAUDE.md resources
acceptance:
  - artifact-framework.md defines the Decision type with schema and status workflow
  - artifact-workflow.md includes Decision creation guidance
  - RULE-004 enforces Decision status transitions and supersession rules
  - RULE-002 references individual decision artifacts as source of truth
---
## What

Established the Decision artifact type as a first-class citizen in the governance framework.

## How

Updated artifact-framework.md with Decision schema, artifact-workflow.md with creation workflow, [RULE-004](RULE-004) with lifecycle enforcement, and [RULE-002](RULE-002) with source-of-truth directive.

## Verification

Decision type is defined, enforceable, and discoverable through the artifact framework.
