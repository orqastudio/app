---
id: IDEA-002
title: Transportable Governance Format
description: |
  Move governance from .claude/-specific format to generic .orqa/governance/ with environment-specific adapters.
status: captured
created: "2026-03-07"
updated: "2026-03-07"
pillars:
  - PILLAR-001
research-needed:
  - Generic governance format design
  - Adapter pattern for .claude/, .continue/, .cursor/
  - Migration tooling and backwards compatibility
promoted-to: null
---## Candidate Items

- Generic governance format — agents, rules, skills, hooks in `.orqa/governance/` as canonical source
- Claude Code adapter — `.claude/` references `.orqa/governance/` content
- Continue adapter — `.continue/` directory adapter
- Cursor adapter — `.cursor/` directory adapter
- Other environment adapters — pattern for any AI tool with directory-based config
- Remove Claude-specific language from governance
- Migration tooling — automated migration from `.claude/` to `.orqa/` + adapter pattern
