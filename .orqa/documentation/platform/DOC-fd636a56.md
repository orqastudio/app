---
id: DOC-fd636a56
type: doc
title: "Thinking Mode: Review"
domain: reference
description: "The user wants something checked, validated, or audited against standards — produces a PASS/FAIL verdict with evidence, not fixes."
category: platform
status: active
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: KNOW-fd636a56
    type: synchronised-with
---

## What This Mode Is

Review Mode is active when the user wants something checked, validated, or audited against standards. The reviewer produces a **verdict** — PASS or FAIL with specific evidence — not fixes. This separation is structural: a reviewer who fixes what they find has removed the independent perspective that makes review meaningful.

Reviews check three kinds of conformance:

1. **Code quality** — does the implementation follow coding standards (RULE-006)?
2. **Completeness** — does the feature satisfy the four-layer completeness rule (RULE-010)?
3. **Artifact integrity** — do the governance artifacts have correct structure and relationships?

---

## When It Activates

The orchestrator routes here when the user's request is about checking conformance, not producing work.

Typical signals:

- "review this code"
- "check if this implementation meets standards"
- "validate the artifact graph"
- "audit the plugin manifests"
- "does this comply with the rules"
- "is this task implementation complete"
- "check the four-layer completeness for this feature"

---

## What the Agent Needs

The reviewer needs the applicable standards loaded as explicit context:

- The relevant RULE artifacts (RULE-006, RULE-010, etc.)
- The acceptance criteria from the task artifact being reviewed
- The artifact graph relationships to check for structural gaps
- The four-layer checklist for IPC features

The verdict must be structured: state what was checked, what passed, what failed, and cite the specific rule or criterion that each violation breaks. Vague verdicts ("looks okay") are not acceptable.

---

## How It Connects to the Thinking Framework

Review Mode is the quality gate between implementation and completion:

- Every Implementation task routes through review before being marked done
- A FAIL verdict routes back to **Implementation Mode** for the implementer to fix
- A FAIL verdict that reveals a missing rule routes to **Learning Loop Mode**

The reviewer does not fix. This is not a limitation — it is the property that gives review its value. The orchestrator returns the verdict to the implementer.

---

## Governance

- RULE-001 (delegation): reviewers produce verdicts, implementers produce fixes — these roles never merge
- Verdicts are structured: PASS/FAIL, evidence list, rule citations
- Acceptance criteria come from the task artifact, not from the reviewer's personal judgement
