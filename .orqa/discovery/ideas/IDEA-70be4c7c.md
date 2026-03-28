---
id: IDEA-70be4c7c
type: discovery-idea
title: Transportable Governance Format
description: Move governance from .claude/-specific format to generic .orqa/process/ with environment-specific adapters.
status: captured
created: 2026-03-07
updated: 2026-03-13
horizon: someday
research-needed:
  - Generic governance format design
  - "Adapter pattern for .claude/, .continue/, .cursor/"
  - Migration tooling and backwards compatibility
relationships:
  - target: PILLAR-2acd86c1
    type: grounded
  - target: PERSONA-c4afd86b
    type: benefits
---

## Candidate Items

- Generic governance format — agents, rules, skills, hooks in `.orqa/process/` as canonical source
- Claude Code adapter — `.claude/` references `.orqa/process/` content
- Continue adapter — `.continue/` directory adapter
- Cursor adapter — `.cursor/` directory adapter
- Other environment adapters — pattern for any AI tool with directory-based config
- Remove Claude-specific language from governance
- Migration tooling — automated migration from `.claude/` to `.orqa/` + adapter pattern
