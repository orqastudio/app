---
id: RULE-cd426b0d
type: rule
title: Session state with next priorities must be written proactively at session end
status: active
enforcement_type: advisory
description: The orchestrator MUST write .state/session-state.md with a Next Session Priorities section before ending any session. The Stop hook enforces this with a warning.
enforcement:

  - mechanism: behavioral

    message: "Write session state with Next Session Priorities before ending any session"
created: 2026-03-23
promoted-from: IMPL-30c3da78
summary: "Before ending any session, orchestrator must write .state/session-state.md with: what was completed, Next Session Priorities (explicit ordered list), environment state, open issues. Stop hook checks for 'Next Session' heading and warns if missing. Writing priorities proactively prevents blind session starts."
tier: always
roles: [orchestrator]
priority: P1
tags: [session-state, priorities, proactive-handoff]
relationships: []
---

## Rule (NON-NEGOTIABLE)

Before ending any session, the orchestrator MUST write `.state/session-state.md` containing:

1. What was completed this session
2. **Next Session Priorities** — explicit, ordered list of what to do next
3. Environment state (what's running, what needs starting)
4. Any open issues or blockers

The "Next Session Priorities" section is mandatory. Without it, the next session starts blind.

## Enforcement

**Stop hook** (`connectors/claude-code/hooks/scripts/stop-checklist.sh`):

- On session stop, reads `.state/session-state.md`
- Checks for a heading containing "Next Session" or "Next Priorities"
- If missing: injects `STOP: You have not written next session priorities` warning into the systemMessage
- Non-blocking (warns, doesn't prevent session end) — but the warning is directive

## FORBIDDEN

- Ending a session without writing session state
- Writing session state without a Next Session Priorities section
- Presenting priorities verbally to the user without writing them to disk
- Waiting to be prompted to write session state
