---
id: RULE-016
title: IDs Are Not Priority
description: Artifact IDs are sequential identifiers for uniqueness and reference. They carry no information about priority, importance, or execution order.
status: active
created: 2026-03-07
updated: 2026-03-07
layer: core
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Artifact ID semantics prevents confusion between identity and priority
  - target: RULE-004
    type: informs
    rationale: Priority scoring and status transitions depend on ID semantics being identifiers not rankings
  - target: RULE-031
    type: informs
    rationale: Pillar alignment drives priority; ID sequence must not be mistaken for priority order
  - type: informed-by
    target: RULE-004
    rationale: Artifact lifecycle defines how IDs are assigned and how priority is separately scored
  - type: enforces
    target: AD-029
    rationale: This rule enforces the ID semantics principle from the universal roles decision — IDs are identifiers not rankings
  - type: scoped-to
    target: AGENT-001
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-002
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-003
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-004
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-005
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-006
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-007
    rationale: Migrated from scope field
  - target: AGENT-008
    type: scoped-to
    rationale: "Auto-generated inverse of scoped-to relationship from AGENT-008"
---
Artifact IDs ([EPIC-045](EPIC-045), [TASK-051](TASK-051), [AD-029](AD-029), etc.) are sequential identifiers for uniqueness and reference. They carry NO information about priority, importance, or execution order.

## Rule

- **IDs are identifiers, not rankings.** [EPIC-001](EPIC-001) is not more important than [EPIC-045](EPIC-045).
- **Priority is explicit.** Use the `priority` field (P1/P2/P3) and scoring dimensions to determine importance.
- **Creation order is irrelevant.** When an artifact was created has no bearing on when it should be worked on.
- **Never sort by ID to imply priority.** Sort by priority field, then by dependency order.

## Why

Sequential IDs tempt agents into treating lower numbers as higher priority. This leads to working on old artifacts before newer, more urgent ones. Priority is a product decision expressed through the scoring framework, not an accident of creation order.

## Related Rules

- [RULE-004](RULE-004) (artifact-lifecycle) — priority scoring and status transitions
- [RULE-031](RULE-031) (vision-alignment) — pillar alignment drives priority, not ID sequence
