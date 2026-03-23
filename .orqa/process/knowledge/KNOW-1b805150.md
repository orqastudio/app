---
id: KNOW-1b805150
title: QA Verification
description: |
  End-to-end functional verification methodology: the "Would It Work" protocol,
  verification path tracing, persistence verification, common QA failure patterns,
  and structured QA report format.
  Use when: Verifying features work end-to-end, checking acceptance criteria,
  tracing data through system layers, or producing QA verdicts.
status: active
created: 2026-03-01
updated: 2026-03-10
category: methodology
version: 1.0.0
user-invocable: true
relationships:
  - target: DOC-421219ce
    type: synchronised-with
---


End-to-end functional verification methodology. This skill teaches the *verification process* — the specific layers and boundaries to trace through come from the project's architecture skills.

## "Would It Work" Protocol

For every feature under test, answer this question literally: **Would this actually work if a real user tried it right now?**

Do not trust:
- Function signatures (they describe intent, not implementation)
- Test names (they describe expectations, not reality)
- Comments (they describe what the author hoped, not what they built)

Instead, verify:
- The actual data flowing through each boundary
- The actual state changes in the UI after each action
- The actual records in storage after each mutation

## E2E Verification Path

For every user-facing feature, trace the full path through every system layer:

1. **User Action** — What component handles the interaction? What event fires?
2. **API/IPC Call** — What command is called? Do arguments match expected types?
3. **Command Handler** — Does the handler exist? Is it registered? Does it delegate properly?
4. **Domain Logic** — Does the function implement expected behavior? Are edge cases handled?
5. **Persistence** — Is the data actually written? Are constraints enforced?
6. **Response Path** — Does the response deserialize correctly? Does the UI update?

Each hop must be verified with evidence, not assumed.

## Persistence Verification

After any mutation (create, update, delete):

1. Verify tests cover the scenario
2. Check that the storage operation is tested
3. Verify that reading the data back produces the correct result in the UI
4. For streaming features, verify the full event pipeline delivers events

## Common QA Failures

| Pattern | Description |
|---------|-------------|
| **Optimistic UI without rollback** | UI updates immediately but doesn't revert on failure |
| **Missing loading state** | Action does nothing visible while waiting for backend |
| **Silent errors** | API call fails but no error shown to user |
| **Stale data after mutation** | Record updated in storage but UI shows old data |
| **Missing validation** | Frontend allows input that backend rejects |
| **Lost state on navigation** | Switching views loses unsaved state |
| **Unregistered command** | Handler exists but isn't registered/routed |
| **Type mismatch** | Backend types don't match frontend types |

## QA Report Format

```markdown
## QA Report: [Feature Name]

### Verification Path
- User Action: [component, event] — VERIFIED / ISSUE
- API Call: [command, args] — VERIFIED / ISSUE
- Handler: [function] — VERIFIED / ISSUE
- Domain Logic: [service] — VERIFIED / ISSUE
- Persistence: [storage, query] — VERIFIED / ISSUE
- Response Path: [store update, re-render] — VERIFIED / ISSUE

### Issues Found
1. [Severity] Description — Location — Expected vs Actual

### Test Coverage Gaps
- [Missing test description]

### Lessons Logged
- New entries: [list or none]
- Recurrence updates: [list or none]
- Checked lessons: YES

### Verdict: PASS / FAIL / CONDITIONAL PASS
```

## Critical Rules

- NEVER declare a feature "working" based only on reading the code — verify actual behavior
- NEVER skip the persistence verification step
- NEVER trust mocked tests as proof of real functionality
- NEVER fix findings one by one during QA — collect, systematize, then fix
- Always trace the complete path through all system layers
- Report findings with exact file paths and line numbers
