---
id: TASK-309
title: "Triage EPIC-059 observations (IMPL-018 through IMPL-025)"
description: "Review all 8 observations logged during EPIC-059 implementation. For each: implement now if needed for epic completion, promote to rule/skill if at understanding maturity, or defer to a new IDEA if out of scope. No observation may remain untriaged when the epic closes."
status: todo
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-059
depends-on: [TASK-308]
acceptance:
  - "Every IMPL-018 through IMPL-025 has a documented triage outcome"
  - "Implement-now items have tasks created within EPIC-059"
  - "Promote items have rule/skill/AD updates completed or tasks created"
  - "Defer items have IDEA-NNN entries created with relationship back to the observation"
  - "No observation left at maturity: observation without a forward path"
---

## What

Triage each observation from EPIC-059 into one of three outcomes:

| Outcome | Action | When |
|---------|--------|------|
| **Implement now** | Create task in EPIC-059, do the work | Observation reveals a gap that blocks or undermines epic goals |
| **Promote** | Update to understanding, create rule/skill/AD | Observation is mature enough to become enforcement |
| **Defer to idea** | Create IDEA-NNN, add deferred-to relationship | Valid but out of scope for this epic |

## How

1. Read each IMPL-018 through IMPL-025
2. For each, determine: does this need to be resolved for EPIC-059 to be honestly complete?
3. Apply the appropriate triage outcome
4. Present triage results to user for approval before executing

## Verification

- `grep -l "maturity: observation" .orqa/process/lessons/IMPL-{018..025}.md` returns zero results (all have moved forward)
- Each observation has either: a task, a promotion, or an IDEA with relationship edge
