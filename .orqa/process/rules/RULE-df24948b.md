---
id: RULE-df24948b
type: rule
title: Context Window Management
description: "The orchestrator must protect its context window by delegating, minimizing output, summarizing results, and using session state files."
status: active
created: 2026-03-11
updated: 2026-03-11
enforcement:
  - mechanism: behavioral
    message: "The orchestrator must protect its context window by delegating, minimizing output, summarizing results, and using session state files; never accumulate implementation details in the orchestrator context"
relationships:
  - target: AD-20d6719d
    type: enforces
  - target: AD-29b5eb06
    type: enforces
---
The orchestrator's context window is a finite resource. Filling it with implementation details, full file contents, or raw agent output degrades coordination quality. These constraints are mandatory.

## Constraints

1. **Delegate, don't accumulate.** When a task requires reading many files, delegate to an agent. The agent's context is separate from yours.
2. **Minimize tool output.** Use targeted reads (offset + limit) instead of reading entire files. Use `head_limit` on search results.
3. **Summarize, don't echo.** When an agent returns results, summarize the key findings for the user. Do not paste the full output.
4. **One task at a time.** Complete and close a task before starting the next. Do not interleave implementation across multiple tasks.
5. **Use session state.** Write intermediate results to `tmp/session-state.md` rather than holding them in context across turns.

## FORBIDDEN

- Reading entire large files (>200 lines) into orchestrator context when a targeted read would suffice
- Echoing full agent output back to the user without summarizing
- Running multiple implementation tasks simultaneously (parallel research is acceptable; parallel implementation is not)
- Holding task-specific implementation details across multiple turns instead of delegating to an agent
- Reading more than 3 files directly when a search or agent delegation would be more efficient

## Related Rules

- [RULE-532100d9](RULE-532100d9) (agent-delegation) — orchestrator delegates, doesn't implement
- [RULE-5e03e67b](RULE-5e03e67b) (code-search-usage) — use search tools instead of reading files