---
id: RULE-22783309
title: IDs Are Not Priority
description: "Artifact IDs are sequential identifiers for uniqueness and reference. They carry no information about priority, importance, or execution order."
status: active
created: 2026-03-07
updated: 2026-03-07
relationships:
  - target: AGENT-ff44f841
    type: observed-by
  - target: AD-0c56aa90
    type: enforces
---
Artifact IDs ([EPIC-be023ed2](EPIC-be023ed2), [TASK-7d550875](TASK-7d550875), [AD-774cc3d0](AD-774cc3d0), etc.) are sequential identifiers for uniqueness and reference. They carry NO information about priority, importance, or execution order.

## Rule

- **IDs are identifiers, not rankings.** [EPIC-e045ab6d](EPIC-e045ab6d) is not more important than [EPIC-be023ed2](EPIC-be023ed2).
- **Priority is explicit.** Use the `priority` field (P1/P2/P3) and scoring dimensions to determine importance.
- **Creation order is irrelevant.** When an artifact was created has no bearing on when it should be worked on.
- **Never sort by ID to imply priority.** Sort by priority field, then by dependency order.

## Why

Sequential IDs tempt agents into treating lower numbers as higher priority. This leads to working on old artifacts before newer, more urgent ones. Priority is a product decision expressed through the scoring framework, not an accident of creation order.

## Related Rules

- [RULE-7b770593](RULE-7b770593) (artifact-lifecycle) — priority scoring and status transitions
- [RULE-1e8a1914](RULE-1e8a1914) (vision-alignment) — pillar alignment drives priority, not ID sequence
