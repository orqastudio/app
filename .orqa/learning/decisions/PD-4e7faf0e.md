---
id: PD-4e7faf0e
type: principle-decision
title: IPC Boundary Design
description: Tauri invoke() commands are the only frontend-backend communication channel. Streaming clarified by PD-39e2fb81.
status: archived
created: 2026-03-02
updated: 2026-04-13
relationships:
  - type: drives
    target: EPIC-347a8c3d
    rationale: "IPC boundary design decision shaped the Rust artifact platform engine's Tauri invoke interface"
---

## Decision

Tauri `invoke()` commands are the only mechanism for frontend-backend communication. All commands are defined as `#[tauri::command]` functions in Rust.

## Rationale

A single, typed communication channel makes the boundary auditable, testable, and secure. No HTTP servers, no WebSocket servers, no shared memory.

## Consequences

Every feature that crosses the boundary needs a Rust command + TypeScript type. Streaming data uses Tauri events (`emit`/`listen`).
