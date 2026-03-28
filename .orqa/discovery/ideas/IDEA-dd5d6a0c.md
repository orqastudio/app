---
id: IDEA-dd5d6a0c
type: discovery-idea
title: "Zero tech debt enforcement — automated detection and prevention"
status: captured
priority: P1
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: "Clarity Through Structure — tech debt obscures structure"
  - target: PERSONA-477971bf
    type: benefits
    rationale: "Practitioners benefit from automated tech debt prevention in AI-assisted workflows"
---

# Zero Tech Debt Enforcement

## Problem

OrqaStudio has a zero tech debt policy: no TODO comments, no commented-out code, no stubs, no deferred deliverables. This policy exists as rules (RULE-af5771e3, RULE-8ee65d73) but enforcement is behavioral — agents are told not to create debt, but nothing mechanically prevents it.

## Potential Enforcement Mechanisms

### 1. Pre-commit hook: TODO/FIXME scanner

- Scan staged files for `TODO`, `FIXME`, `HACK`, `XXX`, `TEMP` patterns
- Block commit with message directing user to create a task artifact instead
- Mechanism: `pre-commit` (strength 7)

### 2. Pre-commit hook: commented-out code detector

- Detect blocks of commented-out code (3+ consecutive lines of comments that look like code)
- Block commit with message: "Delete dead code — git history preserves it"
- Could use ONNX model to distinguish explanatory comments from dead code
- Mechanism: `pre-commit` (strength 7)

### 3. Enforcement engine: stub/placeholder detector

- Scan for common stub patterns: `throw new Error("not implemented")`, `todo!()`, `unimplemented!()`
- Also detect: `console.log("TODO")`, placeholder return values in non-test code
- Mechanism: `hook` (strength 5)

### 4. Agent behavioral rule: no deferred deliverables

- Already exists as RULE-8ee65d73
- Could be strengthened with ONNX classification: detect when an agent response contains language like "we'll handle this later", "in a future epic", "deferred to..."
- Mechanism: `onnx` (strength 4)

### 5. CI/CD gate: tech debt score

- Aggregate all tech debt indicators into a score
- Block merge if score increases
- Dashboard widget showing tech debt trend
- Mechanism: `lint` (strength 6)

## Questions to Explore

1. What's the false positive rate for TODO scanning? (e.g., TODO in test descriptions, documentation about TODO patterns)
2. Can we distinguish "explanatory comment" from "commented-out code" reliably?
3. Should tech debt enforcement be a separate plugin or part of coding-standards?
4. How do we handle legacy code that already has debt?

## Implementation Notes

- Phase 1: TODO/FIXME pre-commit hook (quick win, low risk)
- Phase 2: Stub detector in enforcement engine
- Phase 3: ONNX-based deferred deliverable detection
- Phase 4: CI dashboard and trend tracking
