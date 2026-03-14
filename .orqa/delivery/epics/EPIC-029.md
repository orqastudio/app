---
id: EPIC-029
title: Technical Design
description: "The complete technical blueprint: database schema, IPC commands, Rust modules, streaming pipeline, tool definitions, and error taxonomy."
status: done
priority: P1
created: "2026-03-02"
updated: "2026-03-07"
milestone: MS-000
horizon: null
pillars:
  - PILLAR-001
depends-on:
  - EPIC-025
  - EPIC-026
  - EPIC-028
blocks:
  - EPIC-030
docs-required:
  - AD-001
  - AD-002
  - AD-003
  - AD-007
docs-produced: []
scoring:
  pillar: 3
  impact: 5
  dependency: 3
  effort: 1
  score: 28
---
## Why P1

Implementation cannot begin without knowing the data model, the IPC surface, and the module boundaries. This phase converts the architecture decisions and UX design into implementable specifications.

## What Was Done

- SQLite schema — all tables, columns, indexes, and foreign key constraints
- IPC command catalogue — every Tauri command with its input/output types
- Rust module architecture — domain boundaries, service interfaces, repository pattern
- Svelte component tree — component hierarchy mapped to the UX wireframes
- Streaming pipeline — Agent SDK to Svelte event flow, Channel<T> protocol
- Tool definitions — file tools, search tools, governance tools with permission model
- MCP host interface — design for future external MCP server support
- Error taxonomy — typed errors across the Rust/IPC/TypeScript boundary

## Output

All technical design documentation in `.orqa/documentation/architecture/`.

## Notes

Retroactively captured. Work preceded the artifact framework. These specifications are the source of truth for all implementation.

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.

## Tasks

- [TASK-115](TASK-115): Design SQLite schema
- [TASK-116](TASK-116): Design IPC command catalogue
- [TASK-117](TASK-117): Design Rust module architecture
- [TASK-118](TASK-118): Design Svelte component tree
- [TASK-119](TASK-119): Design streaming pipeline
- [TASK-120](TASK-120): Define tool system and permission model
- [TASK-121](TASK-121): Design MCP host interface
- [TASK-122](TASK-122): Define error taxonomy
