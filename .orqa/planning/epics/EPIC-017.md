---
id: EPIC-017
title: Enforcement & Continuity
status: draft
priority: P2
milestone: MS-002
pillars: [PILLAR-001, PILLAR-002]
description: Add real-time violation detection during streaming, hook-based rule injection, compliance dashboard, and session handoff continuity.
created: 2026-03-07
updated: 2026-03-07
docs-required: [docs/architecture/enforcement.md, docs/architecture/streaming-pipeline.md, .orqa/plans/ (plan required before implementation)]
docs-produced: [.orqa/plans/ (enforcement plan), docs/architecture/enforcement.md (update with real-time violation detection)]
depends-on: []
blocks: []
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
