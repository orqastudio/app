---
id: "IMPL-ca2f3f46"
type: lesson
title: "Orchestrator writes governance artifacts directly instead of delegating to Writer"
description: "The orchestrator is creating IMPL, IDEA, and TASK artifacts itself rather than delegating to a Writer agent. This violates RULE-87ba1b81 in spirit — while governance artifacts are in the orchestrator's exception list, the volume of artifact creation during this session is implementation work that could be parallelised. Delegating artifact writes to a Writer agent would free the orchestrator to continue the design discussion without blocking on file creation."
status: completed
created: "2026-03-13"
updated: "2026-03-13"
maturity: "understanding"
recurrence: 2
relationships:
  - type: cautions
    target: EPIC-281f7857
    rationale: "Orchestrator writing governance artifacts directly instead of delegating — cautions agent lifecycle"
---

## Pattern

During the [EPIC-88f359b0](EPIC-88f359b0) design discussion, the orchestrator has created 12 artifacts (IMPL-a73db2e6 through [IMPL-ca2f3f46](IMPL-ca2f3f46), [IDEA-d2a429c3](IDEA-d2a429c3), [TASK-9d1e01d7](TASK-9d1e01d7) through TASK-362c7937) directly. Each creation blocks the conversation for the time it takes to write the file. A Writer agent could handle artifact creation in parallel while the orchestrator continues the design discussion with the user.

The [RULE-87ba1b81](RULE-87ba1b81) exception for governance artifacts was designed for occasional, lightweight edits — not for a session where artifact creation IS the primary output.

## Fix

When multiple artifacts need creating during a design discussion:

1. Batch the artifact descriptions
2. Delegate to a Writer agent running in background
3. Continue the conversation while artifacts are written
4. Verify artifacts on completion

## Triage

Promoted — [RULE-87ba1b81](RULE-87ba1b81) already enforces this. At recurrence 2, the pattern is confirmed: when creating multiple artifacts during a design session, delegate to a background Writer agent.
