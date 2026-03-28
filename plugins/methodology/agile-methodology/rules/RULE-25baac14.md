---
id: "RULE-25baac14"
type: rule
title: "IDs Are Not Priority"
description: "Artifact IDs are sequential identifiers for uniqueness and reference. They carry no information about priority, importance, or execution order."
status: "active"
created: "2026-03-07"
updated: "2026-03-07"
enforcement:
  - mechanism: behavioral
    message: "Artifact IDs are identifiers not rankings; orchestrator must use the priority field not ID order when sequencing work"
relationships:
  - target: "AD-45cfe1d1"
    type: "enforces"
---
Artifact IDs ([EPIC-7394ba2a](EPIC-7394ba2a), [TASK-0a4a9172](TASK-0a4a9172), [AD-48b310f9](AD-48b310f9), etc.) are sequential identifiers for uniqueness and reference. They carry NO information about priority, importance, or execution order.

## Rule

- **IDs are identifiers, not rankings.** [EPIC-797972a7](EPIC-797972a7) is not more important than [EPIC-7394ba2a](EPIC-7394ba2a).
- **Priority is explicit.** Use the `priority` field (P1/P2/P3) and scoring dimensions to determine importance.
- **Creation order is irrelevant.** When an artifact was created has no bearing on when it should be worked on.
- **Never sort by ID to imply priority.** Sort by priority field, then by dependency order.

## Why

Sequential IDs tempt agents into treating lower numbers as higher priority. This leads to working on old artifacts before newer, more urgent ones. Priority is a product decision expressed through the scoring framework, not an accident of creation order.

## Related Rules

- [RULE-b10fe6d1](RULE-b10fe6d1) (artifact-lifecycle) — priority scoring and status transitions
- [RULE-1b238fc8](RULE-1b238fc8) (vision-alignment) — pillar alignment drives priority, not ID sequence
