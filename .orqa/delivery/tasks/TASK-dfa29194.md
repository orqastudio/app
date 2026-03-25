---
id: TASK-dfa29194
type: task
title: Audit for missing and miscategorised governance artifacts
description: "Identify governance concepts that lack artifacts (undocumented conventions, implicit rules) and artifacts that are miscategorised (rules that should be skills, skills that should be rules, lessons that should be promoted, patterns enforced but not captured)."
status: completed
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - Report listing all missing artifacts with suggested type and content outline
  - Report listing all miscategorised artifacts with recommended reclassification
  - Each recommendation includes rationale based on the governance concept taxonomy (CLAUDE.md)
  - "Where both rule and skill are needed, describe how framing differs"
relationships:
  - target: EPIC-5aa11e2f
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-1637bc63
    type: depends-on
  - target: TASK-d6030100
    type: depends-on
  - target: TASK-88e72cc1
    type: depends-on
  - target: TASK-698afd4c
    type: depended-on-by
---
## What

Audit the boundary between governance artifact types to find gaps and miscategorisations. A rule is a binary constraint (compliant or not). A skill is domain knowledge that shapes how work is done. A lesson is a learned pattern. Sometimes a concept needs both a rule (the constraint) and a skill (the knowledge to implement it correctly) — these should be identified.

## How

1. Read the governance concept taxonomy in CLAUDE.md (Agent, Skill, Rule, Hook, Lesson definitions)
2. For each rule, ask: "Is this a binary constraint, or is it domain knowledge?" If the latter, it may belong as a skill (or need a companion skill)
3. For each skill, ask: "Does this describe a constraint that must be followed?" If so, it may need a companion rule
4. For each lesson with recurrence >= 1, ask: "Should this be promoted?"
5. Search codebase and agent instructions for implicit conventions not captured anywhere
6. Check for enforcement gaps where a hook could automate compliance

## Verification

- Every recommendation is justified against the concept taxonomy
- No "false miscategorisations" (concepts that legitimately belong in their current type)
- Companion artifacts (rule + skill pairs) are identified where both framings are needed