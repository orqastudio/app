---
id: IDEA-002
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
  - target: DOC-036
    type: informed-by
    rationale: Referenced in documentation page Artifact Framework
  - target: PILLAR-001
    type: grounded-by
---
## Candidate Items

- Generic governance format — agents, rules, skills, hooks in `.orqa/process/` as canonical source
- Claude Code adapter — `.claude/` references `.orqa/process/` content
- Continue adapter — `.continue/` directory adapter
- Cursor adapter — `.cursor/` directory adapter
- Other environment adapters — pattern for any AI tool with directory-based config
- Remove Claude-specific language from governance
- Migration tooling — automated migration from `.claude/` to `.orqa/` + adapter pattern
