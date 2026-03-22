---
id: TASK-181675c6
title: Knowledge pipeline flow model rethink
description: Revise the pipeline widget algorithm to accurately represent knowledge flow through the artifact graph.
status: completed
priority: P2
scoring:
  impact: 3
  urgency: 2
  complexity: 3
  dependencies: 1
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Pipeline widget uses a revised flow model that accurately represents how knowledge moves through the artifact graph
  - Not always showing stuck/bottleneck
relationships:
  - target: EPIC-6e774e50
    type: delivers
    rationale: Accurate pipeline flow model prevents false bottleneck signals
---

## Scope

Rewrite PipelineWidget.svelte algorithm to better model knowledge flow. Links to IDEA-e41039bc for broader pipeline thinking. Focus on reducing false stuck/bottleneck indicators.
