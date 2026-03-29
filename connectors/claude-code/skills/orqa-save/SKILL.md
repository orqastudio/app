---
name: orqa-save
description: "Save current session context to governance artifacts via the OrqaStudio daemon."
user-invocable: true
---

# Save Context

Persist the current session context (active epics, tasks, decisions, session state) to governance artifacts.

## Usage

```bash
orqa save
```

This command:

1. Queries the daemon for active work items (epics, tasks)
2. Captures current session state (workflow stage, active context)
3. Writes `.state/session-state.md` with recovery information
4. Updates artifact statuses if work has been completed

Run this before ending a session or when you want to checkpoint progress.
