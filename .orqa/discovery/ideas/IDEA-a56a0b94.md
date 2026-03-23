---
id: IDEA-a56a0b94
type: idea
title: Acknowledged constraints as a decision sub-category with impact tracking
description: "Decisions that acknowledge known limitations, trade-offs, or temporary compromises should be tracked as a distinct sub-category with impact assessment and resolution priority. Ensures acknowledged constraints don't get forgotten by surfacing them and their impact during planning."
status: captured
created: 2026-03-13
updated: 2026-03-13
horizon: active
research-needed:
  - "What is the right vocabulary? 'Tech debt' is software-specific — 'acknowledged constraint' or 'known limitation' is domain-agnostic"
  - "How should impact be assessed? Frequency of encounter, scope of workaround, cost of resolution?"
  - "Should constraints surface during milestone planning? Epic creation? Both?"
  - "How does this interact with the integrity engine? New check category for aging unresolved constraints?"
  - "Should there be a maturity progression? Acknowledged → assessed → prioritised → resolved?"
  - "How does this relate to lessons? A constraint is different from a lesson — a lesson captures understanding, a constraint acknowledges a gap"
  - "What does the dashboard widget look like? Aging constraints sorted by impact?"
relationships:
  - target: AD-aa6b409a
    type: crystallises
    rationale: "AD-aa6b409a's scanner is acknowledged as a known constraint (stopgap before linter plugins)"
  - target: PILLAR-cdf756ff
    type: grounded
  - target: PERSONA-cda6edd6
    type: benefits
---

## Motivation

Decisions sometimes acknowledge trade-offs: "we're doing X now because Y isn't ready yet" or "this approach has limitation Z that we accept for now." These acknowledged constraints are invisible once the decision is accepted — they sit in the decision body but don't surface during planning or milestone reviews.

The result: constraints accumulate silently. No one notices the workaround has been in place for 6 months and the blocker it was working around has been resolved. The constraint becomes permanent not because it's still necessary, but because it's forgotten.

A constraint sub-category on decisions would:
- Track impact (how often is this constraint encountered, how painful is the workaround)
- Surface during planning (when reviewing milestones, show unresolved high-impact constraints)
- Trigger integrity checks (constraints older than N months without reassessment get flagged)
- Close the loop (when a constraint is resolved, the decision is updated and the constraint is marked resolved)

This is framed domain-agnostically — "constraint" works for any field, not just software. A consulting firm's process constraint, a research team's methodology limitation, a product team's market trade-off — all fit the same pattern.

## Sketch

Decision frontmatter extension:
```yaml
constraint:
  type: temporary  # temporary | accepted | resolved
  impact: high     # high | medium | low
  workaround: "Pre-commit scanner instead of linter plugin"
  resolution-condition: "Linter integration plugins built (IDEA-cc13aea9)"
  last-assessed: "2026-03-13"
```

Integrity check: flag constraints with `last-assessed` > 3 months ago as needing reassessment.
Dashboard widget: "Open Constraints" showing count by impact, sorted by age.