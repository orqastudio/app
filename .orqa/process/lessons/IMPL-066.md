---
id: IMPL-066
title: Acceptance criteria not verified line-by-line after implementation
description: Implementer agents deliver the structural change but miss specific acceptance criteria items. The orchestrator marks tasks done without checking each criterion against the actual output. UAT then catches items that were in scope but not delivered.
status: review
created: 2026-03-14
updated: 2026-03-14
maturity: observation
recurrence: 3
relationships:
  - target: RULE-015
    type: observes
    rationale: Honest reporting requires verifying acceptance criteria are actually met, not just that the task was attempted
  - target: RULE-022
    type: observes
    rationale: Verification gate protocol requires independent review against acceptance criteria
  - target: PILLAR-003
    type: observes
    rationale: Purpose through continuity — acceptance criteria ARE the stated intent, and they were lost during execution
---

## Pattern

Under task volume, the orchestrator:
1. Delegates a task with acceptance criteria to an implementer
2. The implementer returns a summary of what was done
3. The orchestrator marks the task done based on the summary
4. UAT later reveals specific acceptance criteria that were not met

Examples from EPIC-073 UAT round 3:
- TASK-469 acceptance: "Roadmap should be first item in Delivery" — roadmap remained top-level (F41)
- TASK-473 acceptance: "Column grouping/sorting configurable" — no sort dropdown exists (F38)
- TASK-470 acceptance: "Relationships no longer appear in metadata card" — relationship-specific fields (epic, milestone, depends-on) still show (F44)

## Root Cause

The orchestrator trusts the implementer's completion summary without independently verifying each acceptance criterion. RULE-022 requires an independent reviewer, but under volume pressure this step gets compressed to "did the agent say it was done?" rather than "did I check each criterion?"

## Fix

After each implementation task, the orchestrator must read the task's `acceptance` field and verify each item independently — not by re-reading the agent's output, but by checking the actual result (reading the changed files, testing the behavior). If an acceptance criterion can't be verified from the orchestrator's context, delegate verification to a reviewer agent with the specific criteria to check.
