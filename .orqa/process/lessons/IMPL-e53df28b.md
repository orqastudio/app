---
id: "IMPL-e53df28b"
type: "lesson"
title: "Orchestrator must manage the dev environment lifecycle automatically"
description: "The orchestrator should automatically manage dev server restart instead of telling the user to do it manually.\\n"
status: "active"
created: "2026-03-07"
updated: "2026-03-07"
maturity: "understanding"
recurrence: 1
relationships: []
---
## Pattern
After making code changes, the orchestrator reported what needed to happen ("you'll need to restart") but left the user to do it manually. The user should not need to run terminal commands.

## Fix
When changes require a dev server restart (Rust changes, new dependencies, config changes), the orchestrator must automatically:
1. Kill any existing dev server processes
2. Run `npx vite optimize` if dependencies changed [IMPL-eb748de2](IMPL-eb748de2)
3. Start `make dev` in the background
4. Verify the server started successfully before reporting to the user

Never tell the user "you'll need to restart" — just do it.
