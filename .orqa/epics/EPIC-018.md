---
id: EPIC-018
title: "Sub-Agent Support"
status: draft
priority: P2
milestone: MS-002
created: 2026-03-07
updated: 2026-03-07
deadline: null
plan: null
depends-on: []
blocks: []
assignee: null
pillar:
  - clarity-through-structure
scoring:
  pillar: 3
  impact: 4
  dependency: 2
  effort: 4
score: 4.8
roadmap-ref: "M8"
docs-required:
  - docs/architecture/sub-agents.md
  - .orqa/plans/ (plan required before implementation)
docs-produced:
  - .orqa/plans/ (sub-agent plan)
  - docs/architecture/sub-agents.md (update with implementation details)
  - docs/architecture/ipc-commands.md (new spawn_agent commands)
tags: [agents, sub-agents, delegation]
---

# Sub-Agent Support

## Tasks

- [ ] Agent registry — reads `.claude/agents/*.md`, indexes capabilities
- [ ] `spawn_agent` tool — spawns sub-agent with role and instructions
- [ ] Explore mode — lightweight codebase exploration agent (no tool approval)
- [ ] Output aggregation — child tool calls collected, summary card with expandable detail
- [ ] Turn limits — configurable max turns per sub-agent invocation
