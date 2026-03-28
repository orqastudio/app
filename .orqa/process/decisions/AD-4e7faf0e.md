---
id: AD-4e7faf0e
type: discovery-decision
title: IPC Boundary Design
description: Tauri invoke() commands are the only frontend-backend communication channel. Streaming clarified by AD-39e2fb81.
status: completed
created: 2026-03-02
updated: 2026-03-13
relationships: []
---

## Decision

Tauri `invoke()` commands are the only mechanism for frontend-backend communication. All commands are defined as `#[tauri::command]` functions in Rust.

## Rationale

A single, typed communication channel makes the boundary auditable, testable, and secure. No HTTP servers, no WebSocket servers, no shared memory.

## Consequences

Every feature that crosses the boundary needs a Rust command + TypeScript type. Streaming data uses Tauri events (`emit`/`listen`).
