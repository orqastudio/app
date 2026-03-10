---
id: EPIC-029
title: Technical Design
status: done
priority: P1
milestone: MS-000
pillars: [clarity-through-structure]
description: "The complete technical blueprint: database schema, IPC commands, Rust modules, streaming pipeline, tool definitions, and error taxonomy."
created: 2026-03-02
updated: 2026-03-07
docs-required: [docs/architecture/decisions.md, docs/ui/]
docs-produced: [docs/architecture/]
depends-on: [EPIC-025, EPIC-026, EPIC-028]
blocks: [EPIC-030]
scoring:
  pillar: 3
  impact: 5
  dependency: 3
  effort: 1
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

All technical design documentation in `docs/architecture/`.

## Notes

Retroactively captured. Work preceded the artifact framework. These specifications are the source of truth for all implementation.
