---
id: IDEA-30d6a2f9
type: discovery-idea
title: App-wide centralized logging
status: captured
created: 2026-03-19
relationships:
  - target: PERSONA-c4afd86b
    type: benefits
  - target: PILLAR-c9e0a695
    type: grounded
---

# App-Wide Centralized Logging

## The Idea

Introduce a centralized logging system across the OrqaStudio app (both Rust backend and Svelte frontend) that provides structured, levelled logging with consistent formatting. Currently, logging is ad-hoc (`console.log` in TypeScript, `println!` in Rust) with no filtering, no persistence, and no way to diagnose issues after the fact.

## What This Would Enable

- Structured log output with timestamps, levels, and source context
- Runtime log level filtering (debug/info/warn/error) without rebuilding
- Log persistence to file for post-mortem debugging
- Frontend logs forwarded to the Rust backend via IPC for unified output
- Plugin logs namespaced by plugin ID so noisy plugins can be filtered independently
- A developer console view in the app for real-time log inspection