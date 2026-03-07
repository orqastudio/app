---
id: EPIC-017
title: "Enforcement & Continuity"
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
  - learning-through-reflection
scoring:
  pillar: 5
  impact: 3
  dependency: 2
  effort: 4
score: 5.8
roadmap-ref: "M7"
docs-required:
  - docs/architecture/enforcement.md
  - docs/architecture/streaming-pipeline.md
  - .orqa/plans/ (plan required before implementation)
docs-produced:
  - .orqa/plans/ (enforcement plan)
  - docs/architecture/enforcement.md (update with real-time violation detection)
tags: [enforcement, continuity, hooks]
---

# Enforcement & Continuity

## Tasks

- [ ] Hooks that inject relevant rules into conversations based on file context
- [ ] Real-time violation detection during streaming (pattern matching on streamed tokens)
- [ ] Visual compliance dashboard
- [ ] Session handoff and continuity — cross-session search, handoff summaries
