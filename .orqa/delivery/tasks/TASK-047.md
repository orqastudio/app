---
id: TASK-047
title: Classify agents with layer and scope fields
description: "Adds layer and scope classification fields to all 16 agent definitions, distinguishing canon agents from project agents and categorising each by domain (software-engineering, governance, or general)."
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-003
acceptance:
  - "All 16 agent definitions have `layer:` field (canon/project/plugin)"
  - "All 16 agent definitions have `scope:` changed from `system` to one of software-engineering, governance, general"
  - Classification is consistent with agent purpose
relationships:
  - target: EPIC-044
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-334
    type: depended-on-by
---
## Classification Plan

| Agent | Layer | Scope |
|-------|-------|-------|
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
