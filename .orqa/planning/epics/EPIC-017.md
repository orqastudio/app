---
id: EPIC-017
title: Enforcement & Continuity
description: Add real-time violation detection during streaming, hook-based rule injection, compliance dashboard, and session handoff continuity.
status: draft
priority: P2
created: 2026-03-07
updated: 2026-03-07
milestone: MS-002
pillars:
  - PILLAR-001
  - PILLAR-002
depends-on: []
blocks: []
docs-required:
  - docs/architecture/enforcement.md
  - docs/architecture/streaming-pipeline.md
  - .orqa/plans/ (plan required before implementation)
docs-produced:
  - .orqa/plans/ (enforcement plan)
  - docs/architecture/enforcement.md (update with real-time violation detection)
scoring:
  pillar: 5
  impact: 3
  dependency: 2
  effort: 4
  score: 6.8
---
## Tasks

- [ ] Hooks that inject relevant rules into conversations based on file context
- [ ] Real-time violation detection during streaming (pattern matching on streamed tokens)
- [ ] Visual compliance dashboard
- [ ] Session handoff and continuity — cross-session search, handoff summaries
