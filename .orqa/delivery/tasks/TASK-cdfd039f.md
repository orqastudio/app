---
id: "TASK-cdfd039f"
title: "Triage EPIC-942c7678 observations (IMPL-c306b136 through IMPL-257c8303)"
description: "Review all 8 observations logged during EPIC-942c7678 implementation. For each: implement now if needed for epic completion, promote to rule/skill if at understanding maturity, or defer to a new IDEA if out of scope. No observation may remain untriaged when the epic closes."
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
acceptance:
  - "Every IMPL-c306b136 through IMPL-257c8303 has a documented triage outcome"
  - "Implement-now items have tasks created within EPIC-942c7678"
  - "Promote items have rule/skill/AD updates completed or tasks created"
  - "Defer items have IDEA-NNN entries created with relationship back to the observation"
  - "No observation left at maturity: observation without a forward path"
relationships:
  - target: "EPIC-942c7678"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-bd4fb2bc"
    type: "depends-on"
  - target: "TASK-cea1bc37"
    type: "depended-on-by"
---
## What

Triage each observation from [EPIC-942c7678](EPIC-942c7678) into one of three outcomes:

| Outcome | Action | When |
|---------|--------|------|
| **Implement now** | Create task in [EPIC-942c7678](EPIC-942c7678), do the work | Observation reveals a gap that blocks or undermines epic goals |
| **Promote** | Update to understanding, create rule/skill/AD | Observation is mature enough to become enforcement |
| **Defer to idea** | Create IDEA-NNN, add deferred-to relationship | Valid but out of scope for this epic |

## How

1. Read each [IMPL-c306b136](IMPL-c306b136) through [IMPL-257c8303](IMPL-257c8303)
2. For each, determine: does this need to be resolved for [EPIC-942c7678](EPIC-942c7678) to be honestly complete?
3. Apply the appropriate triage outcome
4. Present triage results to user for approval before executing

## Verification

- `grep -l "maturity: observation" .orqa/process/lessons/IMPL-{018..025}.md` returns zero results (all have moved forward)
- Each observation has either: a task, a promotion, or an IDEA with relationship edge
