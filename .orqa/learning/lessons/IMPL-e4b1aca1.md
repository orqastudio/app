---
id: IMPL-e4b1aca1
type: lesson
title: Frontend must never call local services directly — always route through Tauri invoke
category: architecture
status: active
recurrence: 1
created: 2026-03-23
tags: [dogfooding, tauri, architecture, ipc]
relationships:
  - type: teaches
    target: PD-4e7faf0e
    rationale: "Frontend must route through Tauri invoke — reinforces IPC boundary design decision"
---

## Observation

The daemon health check was implemented as a direct `fetch("http://127.0.0.1:3002/health")` from the Svelte frontend. This failed inside Tauri's WebView due to CSP restrictions. The daemon was running but the app showed "offline".

## Root Cause

The frontend runs inside a WebView with restricted network access. Direct HTTP calls to localhost services are blocked by Tauri's security model. All external communication must go through Tauri's invoke bridge — the Rust backend makes the HTTP call, the frontend calls invoke.

## Rule

The frontend (app/ui/) MUST NEVER make direct HTTP calls to local services (daemon, search server, MCP, etc.). All communication goes through Tauri commands. This is an architectural constraint from the IPC boundary decision (PD-7121ec20).

## Applies To

- Daemon health checks
- Search queries
- Any future local service integration
- WebSocket connections to local services
