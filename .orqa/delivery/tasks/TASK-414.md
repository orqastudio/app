---
id: TASK-414
title: "Integration test — verify all declared enforcement entries are consumed"
description: "Audit all enforcement entries across all rules and verify each one is actually evaluated by the plugin (CLI) or app (Rust). Produce a coverage report showing declared vs. implemented enforcement."
status: todo
priority: P2
created: "2026-03-14"
updated: "2026-03-14"
epic: EPIC-064
depends-on:
  - TASK-411
  - TASK-412
  - TASK-413
assignee: null
skills:
  - SKILL-033
  - SKILL-011
acceptance:
  - "Every enforcement entry with event: file is evaluated by rule-engine.mjs PreToolUse hook"
  - "Every enforcement entry with event: bash is evaluated by rule-engine.mjs PreToolUse hook"
  - "Every enforcement entry with event: stop is evaluated by rule-engine.mjs Stop hook"
  - "Every enforcement entry with action: inject loads actual skill content"
  - "Coverage report: number of declared entries vs. number consumed, per event type"
  - "Any entry that is declared but not consumed is flagged as a gap"
relationships:
  - target: EPIC-064
    type: delivers
    rationale: "Verification task — confirms the bootstrapping gap is closed"
---

## Scope

### Audit Process

1. Scan all `.orqa/process/rules/RULE-*.md` for `enforcement:` arrays
2. Extract all entries grouped by event type
3. For each event type, verify the plugin hook that handles it:
   - `file` → PreToolUse hook → rule-engine.mjs
   - `bash` → PreToolUse hook → rule-engine.mjs
   - `stop` → Stop hook → rule-engine.mjs (after TASK-411)
   - `lint` → Declarative only (verify documented as such)
4. For each `action: inject` entry, verify skill content is loaded (after TASK-412)
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
