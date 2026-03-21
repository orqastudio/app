---
id: "IDEA-0d3f67de"
title: "Dev controller as standalone repository — attachable to dev and production processes"
description: "Extract the dev controller into its own Tauri app in a standalone repository. Auto-detects dev or production OrqaStudio instances via process discovery. In dev mode it manages process lifecycle + log aggregation. In production mode it attaches to running instances for debugging. Lightweight standalone download — not a webview, a proper Tauri desktop app. Works with the unified logger from IDEA-29bdf6cf."
status: "captured"
created: "2026-03-13"
updated: "2026-03-13"
horizon: "someday"
research-needed:
  - "What is the current dev controller architecture? What does it manage (process lifecycle, SSE, HMR signals)?"
  - "What interface would production attachment use — socket, named pipe, HTTP endpoint?"
  - "How does the controller discover running processes in dev vs production mode? What process signatures distinguish them?"
  - "What is the right Tauri app architecture for a lightweight tool app — minimal frontend, system tray?"
  - "What security considerations exist for attaching to production processes?"
  - "How does this interact with IDEA-29bdf6cf (unified logger)? The controller would be the log aggregation point."
relationships:
  - target: "PILLAR-569581e0"
    type: "grounded"
  - target: "PERSONA-015e8c2c"
    type: "benefits"
---
## Motivation

The dev controller currently lives inside the OrqaStudio repository and is tightly coupled to the development workflow. Extracting it brings two benefits:

1. **Clean separation**: The controller becomes a general-purpose process manager and log aggregator. Other projects could use it. It gets its own release cycle and test suite.

2. **Production debugging**: Users experiencing issues with OrqaStudio could attach the controller to their running instance. The controller captures structured logs from all runtimes (Rust backend, Vite frontend, sidecar) and presents them in a single stream. This turns "something went wrong" into diagnosable evidence.

The combination with [IDEA-29bdf6cf](IDEA-29bdf6cf) (unified logger) is key: the logger produces structured events, the controller aggregates them. In dev mode it's automatic. In production mode the user opts in.

## Sketch

**Standalone Tauri app:**
- Own repository, own release cycle
- Small Tauri desktop app (not a webview) — lightweight download, native feel
- OrqaStudio pulls it in as a dev dependency for development mode
- Users can download it independently for production debugging

**Automatic process discovery:**
- On launch, scans running processes to detect OrqaStudio instances
- Identifies mode based on process signatures:
  - Dev mode: Vite dev server + cargo/tauri processes + sidecar = development instance
  - Production mode: OrqaStudio binary running without dev tooling = production instance
- User selects which instance to attach to if multiple are found

**Two modes:**

| Mode | How attached | What it manages |
|------|-------------|----------------|
| **Development** | Spawns and manages child processes (Vite, Tauri, sidecar) | Process lifecycle + log aggregation + HMR signals |
| **Production** | Connects to running instance via log endpoints | Log aggregation only — no process lifecycle management |

**Production attachment:**
- OrqaStudio exposes a log endpoint (localhost only, opt-in)
- Controller connects and starts receiving structured log events
- Could also capture system resource usage, crash reports, performance metrics
- User shares the log output for support/debugging

**UI:**
- Unified log stream with level filtering (error/warn/info/debug/trace)
- Source filtering (Rust backend, Vite frontend, sidecar)
- Process health indicators (CPU, memory, restart count)
- Exportable log bundles for support tickets
