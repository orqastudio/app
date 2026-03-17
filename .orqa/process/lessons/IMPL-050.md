---
id: IMPL-050
title: Enforcement gaps are always CRITICAL on a product that enforces its own principles
description: "On OrqaStudio specifically, any discovered gap between a declared principle and its mechanical enforcement is a CRITICAL issue addressed immediately — not deferred, not scoped out. We can't dogfood our own product until the underlying enforcement structure is in place. This is a project-level priority principle that should be encoded in the pillar framework and priority system."
status: active
created: 2026-03-13
updated: 2026-03-13
maturity: understanding
recurrence: 1
relationships:
  - target: PILLAR-001
    type: informs
    rationale: "Structure without enforcement is aspirational, not structural — the pillar's promise depends on enforcement being real"
  - target: PILLAR-002
    type: informs
    rationale: "The learning loop only works if gaps discovered through reflection are acted on immediately, not deferred"
  - target: PILLAR-003
    type: informs
    rationale: "Purpose is maintained by closing the gap between declared intent and actual enforcement — deferring gaps IS drift"
  - target: IMPL-049
    type: informed-by
    rationale: "Emerged from the pattern of scope decisions being made without user input — on THIS project, enforcement gaps are never out of scope"
  - target: AD-048
    type: informs
    rationale: "This lesson's principle (enforcement gaps are critical) informed the decision that rule promotion requires enforcement"
  - target: RULE-046
    type: informs
    rationale: "Auto-generated inverse of informs relationship from RULE-046"
  - target: IMPL-054
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-054"
---
## Pattern

OrqaStudio is a product that helps people enforce structured thinking through governance artifacts, rules, and pipeline automation. If the product's own governance has enforcement gaps, the product is not credible — we're shipping a system that doesn't follow its own principles.

The user established a clear principle: on this project, when an enforcement gap is discovered, it is immediately CRITICAL. It is never deferred, never scoped out, never "addressed in a future epic." The product cannot be dogfooded until the enforcement structure it advocates is in place for itself.

This cuts across all three pillars:
- **Pillar 1**: Structure without enforcement is decoration
- **Pillar 2**: Learning that doesn't result in immediate action isn't learning
- **Pillar 3**: Deferring enforcement gaps IS the scope drift this pillar prevents

## Fix

Encode this as:
1. A **pillar gate question** — "Does the system enforce its own declared principles?" applies to all three pillars
2. A **priority auto-classification rule** — enforcement gap detected → CRITICAL
3. A **dogfood-mode principle** — when `dogfood: true`, enforcement gaps bypass normal prioritization and become immediate work
