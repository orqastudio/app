---
id: EPIC-018
title: Sub-Agent Support
description: Build agent registry, spawn_agent tool, explore mode, output aggregation, and turn limits for sub-agent delegation.
status: draft
priority: P2
created: "2026-03-07"
updated: "2026-03-07"
milestone: MS-002
pillars:
  - PILLAR-001
depends-on: []
blocks: []
docs-required:
  - DOC-015
docs-produced:
  - DOC-015
  - DOC-005
scoring:
  pillar: 3
  impact: 4
  dependency: 2
  effort: 4
  score: 5.8
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
