---
id: EPIC-5edafb59
type: epic
title: Claude Code connector switch — clear state, register plugin, verify governance
description: The actual switch to the Claude Code connector plugin. Clears stale symlinks and session state, registers the plugin in Claude Code settings, resets auto-memory, verifies hooks/agents/skills/MCP all work end-to-end, and runs a first governed session to confirm the system is production-ready.
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: MS-654badde
    type: fulfils
  - target: TASK-c6dd7927
    type: delivered-by
  - target: TASK-60598b3d
    type: delivered-by
  - target: TASK-afeac1b7
    type: delivered-by
  - target: TASK-2647d288
    type: delivered-by
  - target: TASK-05a35e0e
    type: delivered-by
---

# EPIC-5edafb59: Claude Code Connector Switch

## Goal

Execute the actual switch from the legacy .claude/ configuration to the new Claude Code connector plugin. This is the moment everything built in EPIC-6967c7dc (rewrite), EPIC-4cedf7bc (ID migration), and EPIC-cdb03816 (pre-switch cleanup) comes together.

## Prerequisites

- EPIC-cdb03816 (pre-switch cleanup) must be complete — search consolidation, skill sync refactor, connector audit all done

## Sequence

1. **Clear stale state** — remove old .claude/ symlinks, stale session files, leftover config
2. **Register plugin** — add connector plugin to Claude Code settings, verify detection
3. **Reset auto-memory** — clear Claude Code auto-memory so it rebuilds from governed sources
4. **Verify everything works** — hooks fire, agents load, skills sync, MCP responds, LSP validates
5. **First governed session** — real work session under full governance, confirm no regressions

## Success Criteria

- No stale .claude/ symlinks or session state remain
- Plugin registered and recognised by Claude Code
- All 9 agents discoverable
- Hooks fire on correct events (pre-prompt, post-response, validate-artifact, etc.)
- MCP server responds to graph and search queries
- Skill sync produces correct proactive skill set
- First governed session completes a real task with full traceability