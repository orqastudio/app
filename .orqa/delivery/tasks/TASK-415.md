---
id: TASK-415
title: "Wire Rust EnforcementEngine to agent tool approval pipeline"
description: "Connect the existing EnforcementEngine in the Rust backend to the agent execution pipeline so that enforcement entries are evaluated before tool execution in app context."
status: todo
priority: P2
created: "2026-03-14"
updated: "2026-03-14"
epic: EPIC-064
depends-on:
  - TASK-414
assignee: null
skills:
  - tauri-v2
  - orqa-domain-services
  - orqa-ipc-patterns
acceptance:
  - "EnforcementEngine.evaluate_file() called before file write operations in agent loop"
  - "EnforcementEngine.evaluate_bash() called before bash executions in agent loop"
  - "Block verdicts prevent tool execution and return violation message to agent"
  - "Warn verdicts allow execution but inject warning into agent context"
  - "Inject verdicts load skill content and inject into agent context"
  - "Evaluation adds <10ms latency per tool call"
relationships:
  - target: EPIC-064
    type: delivers
    rationale: "Phase 2 — app-context enforcement parity with CLI"
  - target: EPIC-050
    type: informed-by
    rationale: "EPIC-050 built the Rust engine; this task wires it to execution"
---

## Scope

### Integration Points

The agent execution pipeline in the app flows through the sidecar → Rust backend. Tool calls arrive as `ToolUse` events. The enforcement engine needs to be called:

1. **Pre-tool-use**: Before executing a tool, evaluate against enforcement entries
2. **Post-tool-use**: After execution, evaluate any post-conditions
3. **Stop**: At session end, evaluate stop entries

### EnforcementEngine API (already exists)

```rust
engine.evaluate_file(&file_path, &content) -> Vec<Verdict>
engine.evaluate_bash(&command) -> Vec<Verdict>
// Stop evaluation needs to be added
```

### Connection to Stream Loop

In `stream_loop.rs`, tool calls are dispatched. Add enforcement evaluation before dispatch:
- Read enforcement rules (cached, reloaded on file change)
- Evaluate against the tool call context
- If block verdict: skip tool execution, return violation as tool result
- If warn/inject verdict: execute tool, inject warning/skill into next turn context
