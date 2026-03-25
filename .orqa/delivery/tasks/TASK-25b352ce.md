---
id: "TASK-25b352ce"
type: "task"
title: "Milestone dependency mapping"
status: "captured"
priority: "P2"
created: 2026-03-25T00:00:00.000Z
updated: 2026-03-25T00:00:00.000Z
acceptance:
  - "Current milestones reviewed and validated"
  - "Epics mapped to milestones with explicit dependencies"
  - "Path from current state to each milestone documented with what's needed"
  - "Dependencies between epics captured as depends-on relationships"
relationships:
  - target: "EPIC-c828007a"
    type: "delivers"
  - target: "TASK-bb5f9ff3"
    type: "depends-on"
    rationale: "Artifact system must be stable before mapping milestones against it"
---
## What

After stabilising the connector, governance process, agent teams, and state machine — map out what's needed to reach each milestone. Use artifacts with explicit depends-on relationships between epics.

Depends on artifact system review (TASK-bb5f9ff3) because milestone mapping relies on the artifact graph being accurate.
