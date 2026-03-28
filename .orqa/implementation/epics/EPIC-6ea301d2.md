---
id: "EPIC-6ea301d2"
type: "epic"
title: "Automated status transitions — the system enforces its own lifecycle"
description: "Implement app-level automation that detects conditions requiring status changes and applies them. Validates all artifacts have valid statuses. The status process documented in DOC-61ecc85e and KNOW-51de8fb7 is enforced mechanically, not just by convention."
status: active
priority: "P1"
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 2
created: 2026-03-15T00:00:00.000Z
updated: 2026-03-15T00:00:00.000Z
deadline: null
horizon: "active"
relationships:
  - target: "MS-21d5096a"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
  - target: "MS-b1ac0a20"
    type: "fulfils"
---

## Context

The unified status system (AD-487e045a) defines 11 statuses and DOC-61ecc85e documents which transitions are automatic vs manual. Currently nothing enforces this — agents and users must remember to update statuses. This epic adds mechanical enforcement.

## Implementation Design

### Status Validation Rule

A Rust-side validation that runs on every artifact scan:

- Check every artifact's `status` field against the valid enum from project.json
- Invalid statuses flagged as integrity errors
- Surfaced in the IntegrityWidget alongside other graph health checks

### Automatic Transition Engine

A Rust service that detects conditions and updates artifact statuses:

| Condition | Transition | Rationale |
| --- | --- | --- |
| All tasks in an epic are `completed` | Epic → `review` | All work done, needs verification |
| All P1 epics in a milestone are `completed` | Milestone → `review` | Gate question needs answering |
| A task's `depends-on` items are all `completed` | Task stays `ready` (no change) | Dependencies met but don't auto-start |
| A task's `depends-on` has an incomplete item | Task → `blocked` | Can't proceed |
| A lesson's recurrence reaches threshold | Lesson → `review` | Needs promotion decision |
| An idea is promoted to an epic | Idea → `completed` | Promotion is completion |

### Plugin Hook Integration

The CLI plugin's graph-guardian should also validate statuses on PostToolUse when `.orqa/` artifacts are written.

## Tasks

- [ ] [TASK-46c06db8](TASK-46c06db8): Add status validation to artifact graph integrity checks
- [ ] [TASK-6810ef0c](TASK-6810ef0c): Build automatic status transition engine in Rust
- [ ] [TASK-cdd8c228](TASK-cdd8c228): Wire transition engine to artifact graph refresh cycle
- [ ] [TASK-62fbe1ec](TASK-62fbe1ec): Add status validation to plugin graph-guardian PostToolUse hook
- [ ] [TASK-936c8fb1](TASK-936c8fb1): Update PipelineStepper to show valid transitions for current artifact
