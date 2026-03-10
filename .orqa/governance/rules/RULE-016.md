---
id: RULE-016
slug: id-not-priority
layer: canon
status: active
scope: system
title: IDs Are Not Priority
description: Artifact IDs are sequential identifiers for uniqueness and reference. They carry no information about priority, importance, or execution order.
created: 2026-03-07
updated: 2026-03-07
---

Artifact IDs (EPIC-045, TASK-051, AD-029, etc.) are sequential identifiers for uniqueness and reference. They carry NO information about priority, importance, or execution order.

## Rule

- **IDs are identifiers, not rankings.** EPIC-001 is not more important than EPIC-045.
- **Priority is explicit.** Use the `priority` field (P1/P2/P3) and scoring dimensions to determine importance.
- **Creation order is irrelevant.** When an artifact was created has no bearing on when it should be worked on.
- **Never sort by ID to imply priority.** Sort by priority field, then by dependency order.

## Why

Sequential IDs tempt agents into treating lower numbers as higher priority. This leads to working on old artifacts before newer, more urgent ones. Priority is a product decision expressed through the scoring framework, not an accident of creation order.

## Related Rules

- RULE-004 (artifact-lifecycle) — priority scoring and status transitions
- RULE-031 (vision-alignment) — pillar alignment drives priority, not ID sequence
