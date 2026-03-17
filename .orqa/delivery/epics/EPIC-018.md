---
id: EPIC-018
title: Sub-Agent Support
description: "Build agent registry, spawn_agent tool, explore mode, output aggregation, and turn limits for sub-agent delegation."
status: captured
priority: P2
created: 2026-03-07
updated: 2026-03-07
horizon: next
scoring:
  impact: 3
  urgency: 2
  complexity: 4
  dependencies: 2
relationships:
  - target: MS-002
    type: delivers
    rationale: Epic belongs to this milestone
  - target: PILLAR-001
    type: grounded-by
---
## Tasks

- [ ] Agent registry — reads `.orqa/agents/*.md`, indexes capabilities
- [ ] `spawn_agent` tool — spawns sub-agent with role and instructions
- [ ] Explore mode — lightweight codebase exploration agent (no tool approval)
- [ ] Output aggregation — child tool calls collected, summary card with expandable detail
- [ ] Turn limits — configurable max turns per sub-agent invocation

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.
