---
id: IDEA-104
title: Dedicated view for items needing attention — drill-down from dashboard health metrics
description: "Create a view accessible from the dashboard clarity card that shows all items contributing to health issues: orphaned artifacts, broken references, low-degree nodes. Each metric on the clarity card links to a filtered list of the specific artifacts needing addressing."
status: captured
created: 2026-03-15
updated: 2026-03-15
horizon: next
research-needed: []
relationships:
  - target: PILLAR-001
    type: grounded
  - target: PERSONA-001
    type: benefits
---

## Motivation

The dashboard clarity card shows aggregate health metrics (orphan count, clusters, avg degree) but there's no way to drill into WHICH artifacts are causing the issues. Clicking a metric should take you to a filtered view showing the specific artifacts that need fixing.
