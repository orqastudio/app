---
id: TASK-44de1cec
title: Create rule for session management and overnight mode
description: Formalize the session management and overnight mode conventions from orchestrator.md into a dedicated rule with enforceable constraints.
status: completed
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - New rule created covering session management lifecycle
  - Overnight mode protocol formalized with enforceable steps
  - Session state file format and location documented
  - FORBIDDEN section with concrete violations
  - Orchestrator.md references the rule
relationships:
  - target: EPIC-4a7aeacb
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-ec136ce9
    type: depended-on-by
---
## What

Session management and overnight mode are currently conventions in orchestrator.md — writing session state to `tmp/session-state.md`, committing work before stepping away, not continuing without user oversight. These should be a rule so they're enforceable and visible in the governance framework.

## How

1. Read the Session Management section in orchestrator.md
2. Determine the next available RULE number
3. Create the rule covering: session state file requirements (what to capture, where to write, when to write), overnight mode protocol (commit, write state, stop), session resume protocol (read state, verify branch, check stashes), FORBIDDEN patterns (ending with uncommitted changes, stale session state from prior sessions)
4. Update orchestrator.md to reference the rule
5. Keep a brief summary in the orchestrator for quick reference

## Verification

- [ ] New rule created covering session lifecycle
- [ ] Overnight mode steps are enforceable constraints
- [ ] Session state format and content requirements defined
- [ ] FORBIDDEN section with concrete anti-patterns
- [ ] Orchestrator.md references the rule
