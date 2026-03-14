---
id: TASK-411
title: "Add stop event handling to rule-engine.mjs"
description: "Extend the plugin's rule engine to evaluate enforcement entries with event: stop. Currently only file/bash events are processed. Stop events need different context (session-level, no file path) and must fire when the Stop hook runs."
status: todo
priority: P1
created: "2026-03-14"
updated: "2026-03-14"
epic: EPIC-064
depends-on: []
assignee: null
skills:
  - orqa-governance
  - orqa-plugin-development
acceptance:
  - "rule-engine.mjs accepts and evaluates stop event context"
  - "Hard filter on line 199 expanded to include stop events"
  - "Stop hook (hooks.json) calls rule-engine.mjs in addition to stop-checklist.sh"
  - "RULE-001 stop enforcement entries (warn + inject) fire during Stop hook"
  - "RULE-044 stop enforcement entries fire during Stop hook"
  - "Stop event context shape documented (no file_path, session-level only)"
relationships:
  - target: EPIC-064
    type: delivers
    rationale: "Core task — enables stop event enforcement in CLI context"
  - target: RULE-001
    type: enforces
    rationale: "RULE-001's permission-seeking enforcement entries depend on stop event support"
---

## Scope

### rule-engine.mjs Changes

1. **Expand tool filter** (line 199): Add stop event detection — when called from Stop hook, process stop-type enforcement entries
2. **Stop event context**: No `file_path`, no `new_text`. Context is session-level: the hook receives `transcript` field per EPIC-050 design
3. **Evaluate stop entries**: Filter enforcement entries by `event: stop`, match against session context, return verdicts

### hooks.json Changes

Update Stop hook to call both scripts:
```json
"Stop": [{
  "matcher": "*",
  "hooks": [
    { "type": "command", "command": "node rule-engine.mjs --event stop" },
    { "type": "command", "command": "bash stop-checklist.sh" }
  ]
}]
```

### Testing

- Manually verify RULE-001 stop entries fire by simulating a stop event
- Verify stop-checklist.sh still runs independently
