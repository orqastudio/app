---
id: IDEA-080
title: "Dev controller as standalone repository — attachable to dev and production processes"
description: "Extract the dev controller into its own repository. In development mode it hooks into dev processes (Vite, Tauri, sidecar). In production it attaches to a running OrqaStudio instance to capture logging and help users debug platform issues. Works with the unified logger from IDEA-079."
status: captured
created: "2026-03-13"
updated: "2026-03-13"
pillars:
  - PILLAR-001
  - PILLAR-002
research-needed:
  - "What is the current dev controller architecture? What does it manage (process lifecycle, SSE, HMR signals)?"
  - "What interface would production attachment use — socket, named pipe, HTTP endpoint?"
  - "How does the controller discover running processes in dev vs production mode?"
  - "What security considerations exist for attaching to production processes?"
  - "How does this interact with IDEA-079 (unified logger)? The controller would be the log aggregation point."
promoted-to: null
---

## Motivation

The dev controller currently lives inside the OrqaStudio repository and is tightly coupled to the development workflow. Extracting it brings two benefits:

1. **Clean separation**: The controller becomes a general-purpose process manager and log aggregator. Other projects could use it. It gets its own release cycle and test suite.

2. **Production debugging**: Users experiencing issues with OrqaStudio could attach the controller to their running instance. The controller captures structured logs from all runtimes (Rust backend, Vite frontend, sidecar) and presents them in a single stream. This turns "something went wrong" into diagnosable evidence.

The combination with [IDEA-079](IDEA-079) (unified logger) is key: the logger produces structured events, the controller aggregates them. In dev mode it's automatic. In production mode the user opts in.

## Sketch

**Repository structure:**
- Standalone repo with its own package.json
- Published as an npm package or standalone binary
- OrqaStudio pulls it in as a dev dependency

**Two modes:**

| Mode | How attached | What it manages |
|------|-------------|----------------|
| **Development** | Spawns and manages child processes (Vite, Tauri, sidecar) | Process lifecycle + log aggregation + HMR signals |
| **Production** | Connects to running processes via log endpoints | Log aggregation only — no process lifecycle management |

**Production attachment:**
- OrqaStudio exposes a log endpoint (localhost only, opt-in)
- Controller connects and starts receiving structured log events
- Could also capture system resource usage, crash reports, performance metrics
- User shares the log output for support/debugging
