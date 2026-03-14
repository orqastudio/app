---
id: IDEA-091
title: "Rethink knowledge pipeline flow model"
description: "The current pipeline widget assumes strict left-to-right flow (lesson→research→decision→skill→rule) but real knowledge flows in multiple directions based on context. Needs a new model that reflects how artifacts actually connect rather than enforcing a single pipeline direction."
status: captured
created: "2026-03-14"
updated: "2026-03-14"
pillars:
  - PILLAR-001
  - PILLAR-002
milestone: null
horizon: later
research-needed:
  - "What flow patterns actually occur in OrqaStudio's artifact graph? Map real edge directions."
  - "What dashboard signal would be most useful — coverage, connectivity, maturity distribution, or something else?"
  - "Should the pipeline be directional at all, or is it better modeled as a network?"
promoted-to: null
spun-off-from: null
relationships:
  - target: EPIC-060
    type: informed-by
    rationale: "Pipeline widget was built in EPIC-060, UAT revealed the flow model is wrong"
---

## Motivation

The knowledge pipeline widget shows stages (Observation → Understanding → Principle → Practice → Enforcement → Verification) with flow rates between them. The flow rate is calculated by checking if each artifact has a relationship to an artifact in the next stage's type.

This model is wrong because:
- Knowledge doesn't flow in one direction — a lesson can inform a rule directly, skipping decisions and skills
- Not every artifact *should* connect forward — a standalone decision is fine
- The widget reports "stuck" and "bottleneck" for stages that are healthy, creating noise
- It inflates problems that aren't problems

## Sketch

Possible replacement approaches:
1. **Coverage model** — % of each type with any relationship at all (orphan detection)
2. **Network view** — show actual edge counts between types, no directionality judgement
3. **Maturity distribution** — for lessons: how many are observation vs understanding vs promoted
4. **Recently changed** — timeline of artifact modifications, more actionable than static counts
