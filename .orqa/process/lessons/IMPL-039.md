---
id: IMPL-039
title: Observation creation should be delegated to background agents
description: "When the orchestrator identifies something that needs to be recorded as an observation, it should delegate the creation to a background Writer agent rather than doing it inline. This keeps the orchestrator focused on coordination and prevents observation logging from blocking the primary work flow."
status: completed
created: 2026-03-13
updated: 2026-03-13
maturity: understanding
recurrence: 1
relationships: []
---

## Pattern

The orchestrator creates observation files (IMPL entries) inline during conversation, blocking the primary work flow. This is technically within the orchestrator's scope (governance artifacts are its domain), but the act of creating a well-structured lesson file takes time that could be parallelized. A background Writer agent can create the file while the orchestrator continues with coordination.

This pattern was demonstrated in the current session: the user pointed out that observations should be delegated to background agents, which itself became an observation — creating a self-referential example of the discipline.

## Fix

Observation: when an observation needs recording, the orchestrator should launch a background Writer agent with the details. The orchestrator continues its primary work. The agent creates the IMPL file. This should be verified against the learning loop implementation (TASK-308) to ensure the delegation model is compatible with the three-tier logging discipline.

## Triage

Promoted — extends [RULE-001](RULE-001) delegation principle. Observation creation delegated to background Writer agents to keep orchestrator focused on coordination.
