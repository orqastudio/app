---
id: IMPL-e7f9a1b3
title: Session state with next priorities not written proactively at session end
category: process
status: promoted
recurrence: 1
promoted-to: RULE-e3f5a7b9
created: 2026-03-23
tags: [dogfooding, session-management, proactive-behavior]
---

## Observation

When the session was wrapping up (all tasks done, team shut down, commits pushed), the orchestrator presented a verbal summary but did not write session state to disk. The user had to prompt "that should be written to session state as the next priorities."

## Root Cause

RULE-4f7e2a91 (session state management) and RULE-e352fd0a (session management) both require proactive state writing. The rules are in context but not mechanically enforced at session end. The Stop hook writes a basic auto-generated summary but doesn't check if the orchestrator wrote proper priorities.

## Recommendation

The Stop hook should check `tmp/session-state.md` for a "Next Session Priorities" section. If missing, inject a warning into the stop output reminding the orchestrator to write priorities before ending. This is a knowledge injection enforcement (RULE-f9d0279c strategy) rather than a hard block.
