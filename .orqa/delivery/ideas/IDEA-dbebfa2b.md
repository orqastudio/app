---
id: IDEA-dbebfa2b
type: idea
title: "Artifact system review — state machine, canonical definitions, then audit"
description: "Before auditing individual artifacts, review the foundations: (1) state machine — are statuses, transitions, and lifecycles correct for how work actually flows? (2) canonical definitions — what IS each artifact type, with clear unambiguous criteria? (3) audit against those definitions. Top-down, not bottom-up."
status: captured
priority: P1
created: 2026-03-24
updated: 2026-03-24
horizon: active
relationships:
  - target: PILLAR-569581e0
    type: grounded
    rationale: "Clarity Through Structure — artifacts in wrong categories undermines the entire governance model"
  - target: PERSONA-015e8c2c
    type: benefits
    rationale: "Practitioners get clearer, more relevant context when artifacts are correctly categorized"
---

## Approach (top-down)

### Phase 1: State machine review
- Are the statuses for each artifact type correct?
- Do the transitions reflect how work actually flows?
- Are there statuses that don't get used, or transitions that happen in practice but aren't modelled?
- Does the schema enforce the right transitions?

### Phase 2: Canonical definitions
- What IS a rule? What IS knowledge? What IS a decision? What IS documentation?
- Clear, unambiguous criteria — if someone creates a new artifact, they should know exactly which type it is
- Boundary cases: where does a rule end and knowledge begin? Where does knowledge end and documentation begin?

### Phase 3: Audit against definitions
- Only after phases 1 and 2 are solid
- Check every artifact against the canonical definitions
- Reclassify, merge, or delete as needed

### Token impact
58 rule files loaded every prompt. If the definitions are wrong, the wrong artifacts are loaded. Getting this right directly reduces cost.
