---
id: IDEA-87cd7dcb
type: planning-idea
title: "Resolve npm audit vulnerabilities before MVP"
description: "npm audit reports 8 vulnerabilities (6 low, 2 moderate) across the workspace. All must be resolved or documented as accepted risk before MVP release."
status: captured
priority: P2
created: 2026-03-24
updated: 2026-03-24
horizon: active
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: "Shipping with known vulnerabilities undermines trust in a governance tool"
  - target: PERSONA-477971bf
    type: benefits
    rationale: "Practitioners need confidence the tool they rely on is secure"
---

## What

`npm install` reports:

```text
8 vulnerabilities (6 low, 2 moderate)
```

Before MVP: run `npm audit`, triage each vulnerability, fix or document as accepted risk.
