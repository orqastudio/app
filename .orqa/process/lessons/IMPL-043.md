---
id: IMPL-043
title: Null relationship targets should link to the tracking artifact instead
description: "When a relationship target doesn't exist yet, the target should point to the IDEA or TASK that tracks its creation — not be set to null. Every planned artifact has a tracking artifact somewhere in the delivery pipeline."
status: active
created: 2026-03-13
updated: 2026-03-13
maturity: understanding
recurrence: 1
relationships: []
---

## Pattern

When creating relationship edges where the target artifact doesn't exist yet, agents default to `target: null` with a rationale explaining the gap. But if the work is planned, there's always an artifact tracking it — an IDEA, TASK, or EPIC. The null target hides a valid forward reference and creates unnecessary integrity warnings.

## Fix

When a relationship target doesn't exist yet:
1. Check if an IDEA, TASK, or EPIC tracks the planned creation
2. If yes: point the target to the tracking artifact (e.g., `target: [IDEA-071](IDEA-071)`)
3. If no: create the tracking artifact first, then link to it
4. `null` targets should only exist when there is genuinely no plan to create the target — and in that case, set `intended: true` to suppress the warning
