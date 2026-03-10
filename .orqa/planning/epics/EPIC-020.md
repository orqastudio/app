---
id: EPIC-020
title: Discovery & Research Artifacts
status: draft
priority: P2
milestone: MS-002
pillars: [PILLAR-001, PILLAR-002]
description: Build structured research artifacts, decision traceability graph, research-to-AD promotion, and conversational research workflow.
created: 2026-03-07
updated: 2026-03-07
docs-required: [docs/product/artifact-framework.md, .orqa/research/README.md (existing research schema), .orqa/plans/ (plan required before implementation)]
docs-produced: [.orqa/plans/ (discovery artifacts plan), docs/architecture/decisions.md (AD for traceability graph data model)]
depends-on: [EPIC-005]
blocks: []
scoring:
  pillar: 5
  impact: 3
  dependency: 1
  effort: 4
  score: 6
---

## Tasks

- [ ] Research artifact type — structured, queryable, filterable
- [ ] Decision traceability graph (research -> AD -> feature -> implementation)
- [ ] Research-to-AD promotion workflow
- [ ] Discovery dashboard — open questions, pending decisions
- [ ] Conversational research workflow — Claude-assisted investigation producing structured artifacts
