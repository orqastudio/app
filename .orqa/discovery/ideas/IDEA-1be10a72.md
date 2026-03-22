---
id: IDEA-1be10a72
type: idea
title: Session management framework — prevent context drift across long and multi-session work
description: Structured session management to prevent agents losing track of intent during long sessions or across session boundaries. Sessions have explicit scope (epic/task), state persistence (what was done, what's in progress, what's next), and context recovery after compaction. Implemented first in the Claude Code connector for dogfooding, then brought into the in-app agent framework.
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: PILLAR-94b281db
    type: grounded
  - target: PERSONA-cda6edd6
    type: benefits
---

# IDEA-1be10a72: Session Management Framework

## Problem

Long AI sessions drift. The agent starts focused on a task, accumulates context, gets compacted, and loses the thread. Across session boundaries it's worse — the next session starts cold with no knowledge of what was done.

This directly violates Pillar 3 (Purpose Through Continuity): the user's original intent must survive implementation pressure, and session boundaries are the biggest source of implementation pressure.

## Concept

### Session Lifecycle
1. **Start** — load previous session state, identify scope (which epic/task), load governing docs
2. **Work** — maintain focus on scoped work, delegate to agents within scope
3. **Compact** — save governance context before compaction, recover cleanly after
4. **Stop** — write session state (done, in progress, next steps, blockers), clean handoff

### Session State Artifact
`tmp/session-state.md` — written at session end, read at session start:
```markdown
## Session: 2026-03-19T14:30:00Z

### Scope
- Epic: EPIC-4cedf7bc
- Tasks: TASK-0b703fd2 (completed), TASK-ad41ea9b (in progress)

### What Was Done
- Implemented ID generation utilities in Rust and TypeScript
- Started bulk migration script

### In Progress
- TASK-ad41ea9b: Migration script handles frontmatter but not body text references yet

### Next Steps
- Complete body text reference migration in TASK-ad41ea9b
- Run validation (TASK-db0e3e3c)

### Blockers
- None
```

### Connector Implementation (dogfood first)
- SessionStart hook reads `tmp/session-state.md` and injects as context
- Stop hook prompts the orchestrator to write session state
- PreCompact hook saves governance context to survive compaction
- Orchestrator prompt includes session management protocol

### In-App Framework (future)
- Session model in the Rust backend (SQLite-backed)
- Session scoping in the UI (pick epic/task to focus on)
- Session timeline view (what happened across sessions)
- Session handoff (structured transition between sessions)

## References

- [agent-deck](https://github.com/asheshgoplani/agent-deck) — multi-agent session orchestration framework. Potential reference for both connector-level and in-app session management patterns.

## Why

Without explicit session management, every session starts from scratch and every long session gradually drifts. The artifact graph has the data to prevent this — we just need to read it at start and write it at stop.
