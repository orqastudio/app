---
id: TASK-7a3000fd
type: task
title: "First governed session test — real task under full governance"
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-b07d074c
    type: delivers
  - target: TASK-c035b984
    type: depends-on
---

# TASK-7a3000fd: First Governed Session Test

## Acceptance Criteria

1. Start a new Claude Code session with the connector plugin active
2. Complete a real task (e.g., create a new artifact, modify an existing one, run a delivery workflow)
3. Verify governance context is injected in every prompt (check prompt injector output)
4. Verify artifact creation uses hex IDs and correct frontmatter
5. Verify relationships are bidirectional (forward and inverse both created)
6. Verify session state is saved on session end and restored on next session start
7. Verify the orchestrator delegates correctly to sub-agents
8. No manual workarounds needed — everything works through the plugin, not through legacy .claude/ configuration
9. Document any issues found for immediate follow-up
