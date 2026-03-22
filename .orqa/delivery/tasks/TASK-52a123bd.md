---
id: TASK-52a123bd
type: task
title: "Final connector audit and cleanup before switch"
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-cdb03816
    type: delivers
  - target: TASK-15a03760
    type: depends-on
---

# TASK-52a123bd: Final Connector Audit

## Acceptance Criteria

1. All hooks fire correctly (test each event type)
2. All 9 agents discoverable via `/agents`
3. Skill sync produces correct output (proactive skills only)
4. MCP server responds to graph + search queries
5. Session management works (start → work → stop → start picks up state)
6. No stale references to old IDs, old paths, or old skill format
7. Prompt injector intent mappings all resolve to existing skills
8. Run connector switch checklist (memory: project_connector_switch_checklist.md)
