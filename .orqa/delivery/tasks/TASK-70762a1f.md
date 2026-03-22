---
id: TASK-70762a1f
title: Integration test — verify all declared enforcement entries are consumed
description: Audit all enforcement entries across all rules and verify each one is actually evaluated by the plugin (CLI) or app (Rust). Produce a coverage report showing declared vs. implemented enforcement.
status: completed
priority: P2
scoring:
  impact: 4
  urgency: 3
  complexity: 2
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - "Every enforcement entry with event: file is evaluated by rule-engine.mjs PreToolUse hook"
  - "Every enforcement entry with event: bash is evaluated by rule-engine.mjs PreToolUse hook"
  - "Every enforcement entry with event: stop is evaluated by rule-engine.mjs Stop hook"
  - "Every enforcement entry with action: inject loads actual skill content"
  - "Coverage report: number of declared entries vs. number consumed, per event type"
  - Any entry that is declared but not consumed is flagged as a gap
relationships:
  - target: EPIC-915291e7
    type: delivers
    rationale: Verification task — confirms the bootstrapping gap is closed
  - target: TASK-ab9fa5f9
    type: depends-on
  - target: TASK-528bba63
    type: depends-on
  - target: TASK-419e65f4
    type: depends-on
  - target: TASK-84e27636
    type: depended-on-by
---

## Scope

### Audit Process

1. Scan all `.orqa/process/rules/RULE-*.md` for `enforcement:` arrays
2. Extract all entries grouped by event type
3. For each event type, verify the plugin hook that handles it:
   - `file` → PreToolUse hook → rule-engine.mjs
   - `bash` → PreToolUse hook → rule-engine.mjs
   - `stop` → Stop hook → rule-engine.mjs (after TASK-ab9fa5f9)
   - `lint` → Declarative only (verify documented as such)
4. For each `action: inject` entry, verify skill content is loaded (after TASK-528bba63)
5. Produce coverage report

### Expected Output

```
ENFORCEMENT COVERAGE REPORT
===========================
Event: file   — 24 declared, 24 consumed (100%)
Event: bash   —  9 declared,  9 consumed (100%)
Event: stop   —  3 declared,  3 consumed (100%)
Event: lint   —  5 declared,  5 declarative-only (correct)
Action: inject — 10 declared, 10 with content (100%)

GAPS: None
```
