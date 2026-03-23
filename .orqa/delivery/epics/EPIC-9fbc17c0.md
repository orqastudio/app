---
id: EPIC-9fbc17c0
type: epic
title: Automated status transitions — the system enforces its own lifecycle
description: "Implement app-level automation that detects conditions requiring status changes and applies them. Validates all artifacts have valid statuses. The status process documented in DOC-94224b27 and KNOW-449b1e02 is enforced mechanically, not just by convention."
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
  - target: AD-6cea0c64
    type: driven-by
    rationale: Mechanically enforces the icon-based status system
  - target: MS-eea45fa8
    type: fulfils
    rationale: Epic belongs to this milestone
  - target: MS-654badde
    type: fulfils
  - target: TASK-8024efcb
    type: delivered-by
  - target: TASK-a2384d29
    type: delivered-by
  - target: TASK-332477b0
    type: delivered-by
  - target: TASK-6aba71e3
    type: delivered-by
  - target: TASK-47fc9eb2
    type: delivered-by
---

## Context

The unified status system (AD-6cea0c64) defines 11 statuses and DOC-94224b27 documents which transitions are automatic vs manual. Currently nothing enforces this — agents and users must remember to update statuses. This epic adds mechanical enforcement.

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

- [ ] [TASK-8024efcb](TASK-8024efcb): Add status validation to artifact graph integrity checks
- [ ] [TASK-a2384d29](TASK-a2384d29): Build automatic status transition engine in Rust
- [ ] [TASK-332477b0](TASK-332477b0): Wire transition engine to artifact graph refresh cycle
- [ ] [TASK-6aba71e3](TASK-6aba71e3): Add status validation to plugin graph-guardian PostToolUse hook
- [ ] [TASK-47fc9eb2](TASK-47fc9eb2): Update PipelineStepper to show valid transitions for current artifact