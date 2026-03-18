---
id: IDEA-035
title: Configurable Priority Scoring
description: "Make the epic scoring/prioritisation system configurable per project — custom dimensions, formulas, and priority bands — with the ability to disable scoring entirely for projects that don't need it."
status: captured
created: 2026-03-07
updated: 2026-03-13
horizon: later
research-needed:
  - "Current scoring model audit (dimensions, formula, band thresholds)"
  - Configuration schema for custom dimensions and formulas in project.json
  - "What happens to priority field when scoring is disabled (manual P1/P2/P3? removed?)"
  - UI for configuring scoring dimensions and previewing priority distribution
  - Migration path for existing epics when scoring config changes
relationships:
  - target: PILLAR-001
    type: grounded
  - target: PERSONA-001
    type: benefits
---
## Problem

The current scoring system is hardcoded: four dimensions (`pillar`, `impact`, `dependency`, `effort`), one formula, fixed P1/P2/P3 bands. This works for OrqaStudio but won't suit every project. Some projects don't need scoring at all — they have three epics and manual prioritisation is fine. Others may want different dimensions (e.g. `revenue_impact`, `tech_debt`, `user_requests`).

## Concept

Make scoring a project-level configuration:

```jsonc
// project.json
{
  "scoring": {
    "enabled": true,
    "dimensions": [
      { "key": "pillar", "label": "Pillar Alignment", "weight": 3, "scale": [1, 5] },
      { "key": "impact", "label": "User Impact", "weight": 2, "scale": [1, 5] },
      { "key": "dependency", "label": "Dependency Factor", "weight": 3, "scale": [1, 5] },
      { "key": "effort", "label": "Effort", "weight": 1, "scale": [1, 5], "divisor": true }
    ],
    "bands": { "P1": [15, null], "P2": [8, 15], "P3": [0, 8] }
  }
}
```

When `scoring.enabled` is `false`, the `scoring` block becomes optional on epics and `priority` can be set manually or omitted entirely.

## Related

- [IDEA-033](IDEA-033) (Live Roadmap View) — roadmap sorting benefits from configurable priority
- [IDEA-034](IDEA-034) (Structured Rule Enforcement) — scoring validation could be a rule check
