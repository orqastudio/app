---
id: EPIC-077
title: Automated status transitions — the system enforces its own lifecycle
description: "Implement app-level automation that detects conditions requiring status changes and applies them. Validates all artifacts have valid statuses. The status process documented in DOC-075 and SKILL-051 is enforced mechanically, not just by convention."
status: active
priority: P1
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 2
created: 2026-03-15
updated: 2026-03-15
deadline: null
horizon: active
relationships:
  - target: AD-049
    type: driven-by
    rationale: Mechanically enforces the icon-based status system
  - target: MS-002
    type: fulfils
    rationale: Epic belongs to this milestone
  - target: MS-001
    type: fulfils
  - target: TASK-504
    type: delivered-by
  - target: TASK-505
    type: delivered-by
  - target: TASK-506
    type: delivered-by
  - target: TASK-507
    type: delivered-by
  - target: TASK-508
    type: delivered-by
---

## Context

The unified status system (AD-049) defines 11 statuses and DOC-075 documents which transitions are automatic vs manual. Currently nothing enforces this — agents and users must remember to update statuses. This epic adds mechanical enforcement.

## Implementation Design

### Status Validation Rule

A Rust-side validation that runs on every artifact scan:
- Check every artifact's `status` field against the valid enum from project.json
- Invalid statuses flagged as integrity errors
- Surfaced in the IntegrityWidget alongside other graph health checks

### Automatic Transition Engine

A Rust service that detects conditions and updates artifact statuses:

| Condition | Transition | Rationale |
|---|---|---|
| All tasks in an epic are `completed` | Epic → `review` | All work done, needs verification |
| All P1 epics in a milestone are `completed` | Milestone → `review` | Gate question needs answering |
| A task's `depends-on` items are all `completed` | Task stays `ready` (no change) | Dependencies met but don't auto-start |
| A task's `depends-on` has an incomplete item | Task → `blocked` | Can't proceed |
| A lesson's recurrence reaches threshold | Lesson → `review` | Needs promotion decision |
| An idea is promoted to an epic | Idea → `completed` | Promotion is completion |

### Plugin Hook Integration

The CLI plugin's graph-guardian should also validate statuses on PostToolUse when `.orqa/` artifacts are written.

## Tasks

- [ ] [TASK-504](TASK-504): Add status validation to artifact graph integrity checks
- [ ] [TASK-505](TASK-505): Build automatic status transition engine in Rust
- [ ] [TASK-506](TASK-506): Wire transition engine to artifact graph refresh cycle
- [ ] [TASK-507](TASK-507): Add status validation to plugin graph-guardian PostToolUse hook
- [ ] [TASK-508](TASK-508): Update PipelineStepper to show valid transitions for current artifact
