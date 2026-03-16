---
id: IDEA-064
title: Pipeline dashboard — thread visualization and flow analysis
description: "Dashboard rendering emergent threads by traversing relationships, showing bottleneck detection (stuck observations, disconnected enforcement), flow direction analysis (forwards vs backwards pipeline), and verification audit trails."
status: completed
created: 2026-03-12
updated: 2026-03-12
horizon: active
research-needed:
  - Graph traversal algorithm for emergent thread rendering
  - Bottleneck detection signals — what queries surface stuck/disconnected/backwards flow
  - Dashboard UX design — how to make pipeline health visible at a glance
  - Unresolved tension display (null targets with intended=false)
relationships:
  - target: EPIC-060
    type: evolves-into
---

## Motivation

[AD-042](AD-042) relies on structural visibility as the self-enforcement mechanism. The pipeline dashboard is how that visibility reaches the user. Without it, the relationships exist in frontmatter but the pipeline health signals ("16 observations never became principles", "enforcement created without observations") remain invisible.
