---
id: IMPL-057
title: "Fix the problem, not the number"
description: "When facing integrity errors, the orchestrator's instinct was to suppress the signal rather than fix the root cause. 'Quickest fix' thinking optimises for making error counts drop, not for working software."
status: active
created: 2026-03-14
updated: 2026-03-14
maturity: understanding
recurrence: 1
relationships: []
---

## Pattern

When the integrity scanner reports errors, the first impulse is to find the fastest way to make the number go to zero. This leads to:

- Changing data to satisfy the checker rather than fixing the checker's bug
- Treating error suppression as equivalent to error resolution
- Framing workarounds as "pragmatic" when they're actually hiding problems

The goal is working software, not green dashboards. A passing check that hides a real problem is worse than a failing check that surfaces one.

## Fix

Every error gets a root cause analysis. If the checker is wrong, fix the checker. If the data is wrong, fix the data. If both are wrong, fix both. Never suppress a signal to make a number go down.
