---
id: IMPL-029
title: Orchestrator writes governance artifacts directly instead of delegating to Writer
description: "The orchestrator is creating IMPL, IDEA, and TASK artifacts itself rather than delegating to a Writer agent. This violates RULE-001 in spirit — while governance artifacts are in the orchestrator's exception list, the volume of artifact creation during this session is implementation work that could be parallelised. Delegating artifact writes to a Writer agent would free the orchestrator to continue the design discussion without blocking on file creation."
status: completed
created: 2026-03-13
updated: 2026-03-13
maturity: understanding
recurrence: 2
relationships: []
---
## Pattern

During the [EPIC-059](EPIC-059) design discussion, the orchestrator has created 12 artifacts (IMPL-018 through [IMPL-029](IMPL-029), [IDEA-069](IDEA-069), [TASK-302](TASK-302) through TASK-309) directly. Each creation blocks the conversation for the time it takes to write the file. A Writer agent could handle artifact creation in parallel while the orchestrator continues the design discussion with the user.

The [RULE-001](RULE-001) exception for governance artifacts was designed for occasional, lightweight edits — not for a session where artifact creation IS the primary output.

## Fix

When multiple artifacts need creating during a design discussion:
1. Batch the artifact descriptions
2. Delegate to a Writer agent running in background
3. Continue the conversation while artifacts are written
4. Verify artifacts on completion

## Triage

Promoted — [RULE-001](RULE-001) already enforces this. At recurrence 2, the pattern is confirmed: when creating multiple artifacts during a design session, delegate to a background Writer agent.
