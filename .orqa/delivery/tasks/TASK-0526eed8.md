---
id: "TASK-0526eed8"
type: "task"
title: "Establish learning loop and completion discipline (IMPL-0809b549, 022, 023, 024)"
description: "Create enforcement for: tracking open items during implementation, human-gated epic completion, automated observation logging by agents, and recording lessons on task completion artifacts."
status: archived
created: "2026-03-13"
updated: "2026-03-13"
acceptance:
  - "IMPL-0809b549 through IMPL-db8027b6 maturity updated to understanding"
  - "Epic completion gate updated in RULE-b10fe6d1 to require human approval"
  - "Open-item tracking discipline documented (rule update or new rule)"
  - "Epic readiness surfacing approach documented (UI feature or tool output)"
  - "Learning checkpoint defined for task completion"
  - "Task body template updated with Lessons section"
relationships:
  - target: "EPIC-88f359b0"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "IMPL-b27c458f"
    type: "yields"
---

## What

Address four related process gaps:

1. Open items discovered during implementation must be immediately captured as tasks, not held in conversation (IMPL-0809b549)
2. Epics with all tasks done but not marked complete must be surfaced to the user for review (IMPL-e13eb86c)
3. Epic completion (`review → done`) must be a human gate — the orchestrator presents status and asks for approval
4. Agents must auto-log observations and increment recurrence when they encounter "why did that happen?" moments (IMPL-a1373533)
5. Task completion artifacts must record what lessons were created or updated, making learning visible to the user (IMPL-b27c458f)

## How

1. Update [RULE-b10fe6d1](RULE-b10fe6d1) epic completion gate to require explicit user approval
2. Add learning checkpoint to task completion — orchestrator asks "what observations were logged?" before accepting done
3. Update task schema bodyTemplate to include a Lessons section
4. Document the epic readiness surfacing approach
5. Update all four IMPL entries to understanding

## Verification

- [RULE-b10fe6d1](RULE-b10fe6d1) updated with human gate for epic completion
- Task schema bodyTemplate includes Lessons section
- Process documented and enforceable
- All four IMPL entries have maturity: understanding

## Lessons

- Updated [RULE-b10fe6d1](RULE-b10fe6d1): added human gate, epic readiness surfacing, observation triage sections, and FORBIDDEN patterns
- Updated task schema: added required Lessons body section
- [IMPL-0809b549](IMPL-0809b549) through [IMPL-db8027b6](IMPL-db8027b6) already at understanding — no maturity changes needed
