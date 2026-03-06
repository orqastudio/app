---
id: IMPL-003
title: "Orchestrator must manage the dev environment lifecycle"
category: dev-environment
recurrence: 1
status: active
promoted_to: null
created: 2026-03-03
updated: 2026-03-03
---

# Orchestrator must manage the dev environment lifecycle

## Pattern
After making code changes, the orchestrator reported what needed to happen ("you'll need to restart") but left the user to do it manually. The user should not need to run terminal commands.

## Fix
When changes require a dev server restart (Rust changes, new dependencies, config changes), the orchestrator must automatically:
1. Kill any existing dev server processes
2. Run `npx vite optimize` if dependencies changed (IMPL-001)
3. Start `make dev` in the background
4. Verify the server started successfully before reporting to the user

Never tell the user "you'll need to restart" — just do it.
