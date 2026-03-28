---
id: "TASK-d8f219b7"
type: "task"
title: "Classify agents with layer and scope fields"
description: "Adds layer and scope classification fields to all 16 agent definitions, distinguishing canon agents from project agents and categorising each by domain (software-engineering, governance, or general)."
status: archived
created: 2026-03-09T00:00:00.000Z
updated: 2026-03-09T00:00:00.000Z
acceptance:
  - "All 16 agent definitions have `layer:` field (canon/project/plugin)"
  - "All 16 agent definitions have `scope:` changed from `system` to one of software-engineering, governance, general"
  - "Classification is consistent with agent purpose"
relationships:
  - target: "EPIC-4ce64ab0"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## Classification Plan

| Agent | Layer | Scope |
| ------- | ------- | ------- |
| orchestrator | canon | general |
| agent-maintainer | canon | governance |
| backend-engineer | canon | software-engineering |
| frontend-engineer | canon | software-engineering |
| designer | canon | software-engineering |
| data-engineer | canon | software-engineering |
| debugger | canon | software-engineering |
| devops-engineer | canon | software-engineering |
| test-engineer | canon | software-engineering |
| refactor-agent | canon | software-engineering |
| code-reviewer | canon | general |
| qa-tester | canon | general |
| ux-reviewer | canon | general |
| systems-architect | canon | general |
| documentation-writer | canon | general |
| security-engineer | canon | general |

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
