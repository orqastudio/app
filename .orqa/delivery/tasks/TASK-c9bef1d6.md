---
id: TASK-c9bef1d6
type: task
title: Investigate and suppress tao event loop warnings
description: Investigate tao framework event loop warnings flooding the dev controller output. Determine if they indicate a real issue or can be safely filtered.
status: archived
priority: P3
scoring:
  impact: 1
  urgency: 1
  complexity: 1
  dependencies: 1
created: 2026-03-13
updated: 2026-03-13
assignee: null
acceptance:
  - Root cause of tao NewEvents/RedrawEventsCleared warnings understood and documented
  - Warnings either fixed at source or filtered from dev output with documented justification
relationships:
  - target: EPIC-f7055984
    type: delivers
    rationale: Theme H — framework warning noise from UAT
---

## Scope

### tao Event Loop Warnings (Finding #34)

- **Symptoms**: Dev controller floods with:
  - `WARN tao::platform_impl::platform::event_loop::runner: NewEvents emitted without explicit RedrawEventsCleared`
  - `WARN tao::platform_impl::platform::event_loop::runner: RedrawEventsCleared emitted without explicit MainEventsCleared`
- **Investigation**: Is this a known tao/Tauri issue? Does it affect Windows specifically? Is there a version fix?
- **Resolution options**:
  1. Upgrade tao/wry if a fix exists
  2. Filter WARN-level tao logs in dev controller
  3. Document as known framework noise if harmless
