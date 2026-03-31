---
id: IMPL-d66af988
type: lesson
title: Learning loop broken — lessons not created on first failure occurrence
category: process
status: active
recurrence: 1
created: 2026-03-23
tags: [dogfooding, learning-loop, enforcement-gap]
relationships:
  - type: cautions
    target: EPIC-3e6cad90
    rationale: "Identifies self-review bypass gap in learning pipeline — orchestrator skips lesson creation when fixing its own mistakes"
---

## Observation

When the orchestrator fixed invalid relationship types (first occurrence), no lesson was created. The fix was applied and work continued. On the second occurrence, the lesson was created retroactively at recurrence 2. The learning loop requires lessons at recurrence 1 — the first fix is the signal to capture the pattern.

## Root Cause

The orchestrator was acting as its own reviewer and skipped the lesson step. RULE-c603e90e requires review agents to log lessons on FAIL verdicts, but when the orchestrator self-reviews (fixes its own mistakes), the lesson creation step is dropped in favour of moving forward.

## Impact

Without a lesson at recurrence 1, recurrence 2 isn't flagged for promotion — the system only catches it when a human notices the repeat. The governance pipeline's ability to learn is dependent on discipline that the hooks should enforce.

## Mitigation

The hook enforcement restructuring (task #8) should inject "create a lesson when fixing a recurring pattern" as a critical behavioral rule. The pre-commit hook could also check: if a commit fixes a file that was fixed in a recent prior commit for the same reason, flag it as a potential recurring pattern.
