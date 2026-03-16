---
id: TASK-443
title: Knowledge pipeline flow model rethink
description: Revise the pipeline widget algorithm to accurately represent knowledge flow through the artifact graph.
status: completed
priority: P2
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Pipeline widget uses a revised flow model that accurately represents how knowledge moves through the artifact graph
  - Not always showing stuck/bottleneck
relationships:
  - target: EPIC-068
    type: delivers
    rationale: Accurate pipeline flow model prevents false bottleneck signals
---

## Scope

Rewrite PipelineWidget.svelte algorithm to better model knowledge flow. Links to IDEA-091 for broader pipeline thinking. Focus on reducing false stuck/bottleneck indicators.
