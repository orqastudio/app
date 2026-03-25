---
id: "EPIC-b07d074c"
type: "epic"
title: "Claude Code connector switch — clear state, register plugin, verify governance"
description: "The actual switch to the Claude Code connector plugin. Clears stale symlinks and session state, registers the plugin in Claude Code settings, resets auto-memory, verifies hooks/agents/skills/MCP all work end-to-end, and runs a first governed session to confirm the system is production-ready."
status: "captured"
created: 2026-03-19T00:00:00.000Z
updated: 2026-03-19T00:00:00.000Z
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

# EPIC-b07d074c: Claude Code Connector Switch

## Goal

Execute the actual switch from the legacy .claude/ configuration to the new Claude Code connector plugin. This is the moment everything built in EPIC-9b58fdcb (rewrite), EPIC-d1d42012 (ID migration), and EPIC-1358323e (pre-switch cleanup) comes together.

## Prerequisites

- EPIC-1358323e (pre-switch cleanup) must be complete — search consolidation, skill sync refactor, connector audit all done

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