---
id: IMPL-e3399a4d
type: lesson
title: Enforcement gaps are always CRITICAL on a product that enforces its own principles
description: "On OrqaStudio specifically, any discovered gap between a declared principle and its mechanical enforcement is a CRITICAL issue addressed immediately — not deferred, not scoped out. We can't dogfood our own product until the underlying enforcement structure is in place. This is a project-level priority principle that should be encoded in the pillar framework and priority system."
status: active
created: 2026-03-13
updated: 2026-03-13
maturity: understanding
recurrence: 1
relationships:
  - target: AD-c6c2d9fb
    type: teaches
    rationale: "This lesson's principle (enforcement gaps are critical) informed the decision that rule promotion requires enforcement"
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