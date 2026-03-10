---
id: EPIC-016
title: Prioritisation Framework
status: draft
priority: P1
milestone: MS-002
pillars: [clarity-through-structure, learning-through-reflection]
description: Build the scoring model from the roadmap into the app with configurable dimensions, weights, and priority bands.
created: 2026-03-07
updated: 2026-03-07
docs-required: [docs/product/artifact-framework.md (prioritisation framework section), .orqa/plans/ (plan required before implementation)]
docs-produced: [.orqa/plans/ (prioritisation plan), docs/architecture/project-configuration.md (update with priority config schema), docs/architecture/decisions.md (AD for scoring formula and configurability)]
depends-on: [EPIC-005]
blocks: []
scoring:
  pillar: 5
  impact: 4
  dependency: 2
  effort: 4
---

## Why P1

OrqaStudio is a structured thinking tool. Prioritisation is a core structured thinking capability. The dimensions and weights are configurable per project — different projects have different prioritisation needs.

## Tasks

- [ ] Priority dimensions — user configures dimensions and weights in `.orqa/project.json`
- [ ] Default dimensions: Impact (x2), Effort (x2, inverted), Urgency (x2), Pillar Alignment (x3)
- [ ] Composite priority score — weighted sum produces comparable number across all item types
- [ ] Priority bands — score ranges map to P1-P3 labels
- [ ] Auto-scoring — suggest dimension scores based on frontmatter when items are created
- [ ] Manual override with tracking
- [ ] Priority views — backlog sorted by composite score, filterable by band
- [ ] Custom dimensions — users add project-specific scoring dimensions
