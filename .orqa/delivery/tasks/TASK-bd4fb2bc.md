---
id: "TASK-bd4fb2bc"
title: "Establish learning loop and completion discipline (IMPL-97e2788f, 022, 023, 024)"
description: "Create enforcement for: tracking open items during implementation, human-gated epic completion, automated observation logging by agents, and recording lessons on task completion artifacts."
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
acceptance:
  - "IMPL-97e2788f through IMPL-257c8303 maturity updated to understanding"
  - "Epic completion gate updated in RULE-7b770593 to require human approval"
  - "Open-item tracking discipline documented (rule update or new rule)"
  - "Epic readiness surfacing approach documented (UI feature or tool output)"
  - "Learning checkpoint defined for task completion"
  - "Task body template updated with Lessons section"
relationships:
  - target: "EPIC-942c7678"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-cdfd039f"
    type: "depended-on-by"
  - target: "TASK-cea1bc37"
    type: "depended-on-by"
  - target: "IMPL-ccb23768"
    type: "yields"
---
## What

Address four related process gaps:

1. Open items discovered during implementation must be immediately captured as tasks, not held in conversation (IMPL-97e2788f)
2. Epics with all tasks done but not marked complete must be surfaced to the user for review (IMPL-b149653d)
3. Epic completion (`review → done`) must be a human gate — the orchestrator presents status and asks for approval
4. Agents must auto-log observations and increment recurrence when they encounter "why did that happen?" moments (IMPL-f2b140da)
5. Task completion artifacts must record what lessons were created or updated, making learning visible to the user (IMPL-ccb23768)

## How

1. Update [RULE-7b770593](RULE-7b770593) epic completion gate to require explicit user approval
2. Add learning checkpoint to task completion — orchestrator asks "what observations were logged?" before accepting done
3. Update task schema bodyTemplate to include a Lessons section
4. Document the epic readiness surfacing approach
5. Update all four IMPL entries to understanding

## Verification

- [RULE-7b770593](RULE-7b770593) updated with human gate for epic completion
- Task schema bodyTemplate includes Lessons section
- Process documented and enforceable
- All four IMPL entries have maturity: understanding

## Lessons

- Updated [RULE-7b770593](RULE-7b770593): added human gate, epic readiness surfacing, observation triage sections, and FORBIDDEN patterns
- Updated task schema: added required Lessons body section
- [IMPL-97e2788f](IMPL-97e2788f) through [IMPL-257c8303](IMPL-257c8303) already at understanding — no maturity changes needed
