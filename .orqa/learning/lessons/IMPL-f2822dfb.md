---

id: IMPL-f2822dfb
type: lesson
title: "Design discussion mode should be a skill, not a mode or built-in behaviour"
description: "User confirmed the design discussion protocol should be implemented as a skill the orchestrator loads, not a plan mode variant or hardcoded behaviour. This keeps it composable — loadable when needed, not always active."
status: active
created: 2026-03-13
updated: 2026-03-13
maturity: understanding
recurrence: 1
relationships: []
---

## Pattern

Skills are the right vehicle for protocols that shape orchestrator behaviour. A skill is loadable, versionable, and composable. A mode or built-in behaviour is always present and rigid.

## Fix

Implement as a skill (e.g., `design-discussion`) loaded by the orchestrator when it detects a design-level conversation. The skill defines entry triggers, checkpoint intervals, observation logging discipline, and wrap-up protocol.
