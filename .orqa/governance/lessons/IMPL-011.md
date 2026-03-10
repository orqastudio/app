---
id: IMPL-011
title: Investigate systemically before fixing individual issues
description: |
  Collect all findings first, group by systemic theme, investigate the architecture, then design solutions at the system level. One fix addressing six findings is better than six independent fixes.
status: promoted
created: 2026-03-07
updated: 2026-03-07
recurrence: 1
promoted_to: RULE-030
---
## What Happened

During UAT Round 1 (EPIC-043), multiple issues were identified across artifact display, status UX, null value handling, breadcrumbs, and memory leaks. The instinct was to fix each finding immediately as reported.

## Why It Was Wrong

Fixing issues one by one leads to inconsistent solutions — each fix is designed in isolation without understanding the shared root causes. Multiple findings often stem from the same architectural gap (e.g., 6 findings all traced to "the renderer displays every YAML field regardless of value").

## The Correct Approach

1. **Collect all findings first** — let the user complete their UAT pass without interruption
2. **Group findings by systemic theme** — identify shared root causes across individual symptoms
3. **Investigate the architecture** — understand the current component tree, data flow, and patterns before proposing fixes
4. **Design solutions at the system level** — one fix that addresses 6 findings is better than 6 independent fixes
5. **Then create tasks** — scoped to systemic solutions, not individual symptoms

## Applies To

- UAT rounds and bug triage
- Any batch of related issues reported together
- Refactoring decisions
- Any situation where multiple symptoms share a root cause
