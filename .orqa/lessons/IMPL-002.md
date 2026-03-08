---
id: IMPL-002
title: "Kill existing dev server processes before starting new ones"
category: dev-environment
recurrence: 1
status: active
promoted_to: null
created: 2026-03-03
updated: 2026-03-03
description: >
  Check for and kill existing dev server processes on port 1420 before
  starting a new instance to avoid port-in-use errors.
---

## Pattern
Starting `cargo tauri dev` while a previous instance is still running (or its port is held by a lingering process) causes a `Port 1420 is already in use` error. This happens when the previous window was closed but the process wasn't fully terminated, or when restarting after code changes.

## Fix
Before starting the dev server, check for and kill any existing `orqa-studio.exe` or `node.exe` processes bound to port 1420. The orchestrator should do this automatically.

```bash
netstat -ano | grep ":1420 "
taskkill //F //PID <pid>
```
