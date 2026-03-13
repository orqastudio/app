---
id: TASK-308
title: "Establish learning loop and completion discipline (IMPL-021, 022, 023, 024)"
description: "Create enforcement for: tracking open items during implementation, human-gated epic completion, automated observation logging by agents, and recording lessons on task completion artifacts."
status: done
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-059
depends-on: []
acceptance:
  - "IMPL-021 through IMPL-025 maturity updated to understanding"
  - "Epic completion gate updated in RULE-004 to require human approval"
  - "Open-item tracking discipline documented (rule update or new rule)"
  - "Epic readiness surfacing approach documented (UI feature or tool output)"
  - "Learning checkpoint defined for task completion"
  - "Task body template updated with Lessons section"
---

## What

Address four related process gaps:

1. Open items discovered during implementation must be immediately captured as tasks, not held in conversation (IMPL-021)
2. Epics with all tasks done but not marked complete must be surfaced to the user for review (IMPL-022)
3. Epic completion (`review → done`) must be a human gate — the orchestrator presents status and asks for approval
4. Agents must auto-log observations and increment recurrence when they encounter "why did that happen?" moments (IMPL-023)
5. Task completion artifacts must record what lessons were created or updated, making learning visible to the user (IMPL-024)

## How

1. Update RULE-004 epic completion gate to require explicit user approval
2. Add learning checkpoint to task completion — orchestrator asks "what observations were logged?" before accepting done
3. Update task schema bodyTemplate to include a Lessons section
4. Document the epic readiness surfacing approach
5. Update all four IMPL entries to understanding

## Verification

- RULE-004 updated with human gate for epic completion
- Task schema bodyTemplate includes Lessons section
- Process documented and enforceable
- All four IMPL entries have maturity: understanding

## Lessons

- Updated [RULE-004](RULE-004): added human gate, epic readiness surfacing, observation triage sections, and FORBIDDEN patterns
- Updated task schema: added required Lessons body section
- [IMPL-021](IMPL-021) through [IMPL-025](IMPL-025) already at understanding — no maturity changes needed
