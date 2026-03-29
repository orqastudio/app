---
id: KNOW-a3f1c927
type: knowledge
status: active
title: "Engine Library Architecture and Crate Domains"
description: "Rust engine crates by functional domain, their responsibilities, and how they are consumed by access layers — use when working in or calling engine code"
tier: always
created: 2026-03-29
roles: [implementer, reviewer, planner]
paths: [engine/]
tags: [architecture, engine, rust-crates, libraries]
relationships:
  - type: synchronised-with
    target: DOC-62969bc3
---

# Engine Library Architecture and Crate Domains

The engine is a collection of Rust library crates — not a monolith, not a single process. Each crate covers one functional domain and is independently consumable by any access layer.

## Engine Crates

| Crate | Domain | Key Responsibilities |
| ------- | -------- | -------------------- |
| `engine/types` | Shared foundation | Shared types, errors, traits, config, path resolution — consumed by all other crates |
| `engine/artifact` | Artifact I/O | Artifact reading, parsing, frontmatter extraction |
| `engine/graph` | Relationships | Artifact relationships, traceability, inverse computation from forward-only declarations |
| `engine/workflow` | State machines | State machine evaluation, transition resolution, guard/action execution |
| `engine/prompt` | Prompt generation | Five-stage prompt generation from plugin registries and workflow state |
| `engine/search` | Semantic search | Semantic search over governance artifacts via ONNX embeddings and DuckDB |
| `engine/enforcement` | Rule enforcement | Rule evaluation, artifact validation, coding standards, linting config generation |
| `engine/plugin` | Plugin system | Plugin installation, composition, schema generation, content management |
| `engine/agent` | Agent generation | Base role definitions, task-specific agent generation from role + workflow + knowledge |
| `engine/streaming` | Stream loop | LLM interaction, tool execution, event delivery |
| `engine/lesson` | Learning loop | Lesson store, promotion pipeline from lesson to mechanical rule |
| `engine/project` | Project scanning | Project settings, governance counts, artifact discovery |
| `engine/validation` | Integrity checks | Cross-artifact validation, graph construction, referential integrity |
| `engine/mcp-server` | MCP protocol | Model Context Protocol server (exposes engine to LLM tools) |
| `engine/lsp-server` | LSP protocol | Language Server Protocol server (validates artifacts in editors) |
| `engine/core` | Facade | Thin re-export of all engine crates for consumers that need the full surface |

## Critical Design Rule: Protocols Are Not Boundaries

MCP and LSP are **access protocols** — they expose engine capabilities to external consumers. They are NOT application boundaries. Business logic belongs in engine crates, not in the protocol handlers.

**Correct:** MCP tool call → MCP server receives call → delegates to `engine/graph` or `engine/search` → returns result
**Wrong:** MCP server implementing graph traversal or search logic directly

The same applies to LSP: validation logic lives in `engine/enforcement` and `engine/validation`, not in the LSP server itself.

## How Access Layers Consume Engine Crates

All four access layers are **peer consumers** of the same engine crate library:

| Access Layer | How It Consumes Engine |
| ------------ | ----------------------- |
| **Daemon** | Links directly to crates; serves MCP/LSP as protocol adapters |
| **App (Tauri backend)** | Links directly to crates via Tauri Rust backend |
| **CLI** | Links directly to crates; thin wrapper exposing commands |
| **Connector** | Links directly to crates; generates tool-native plugin output |

No access layer IS the engine. No access layer contains engine business logic. They all consume it.

## What Belongs in Engine vs Access Layer

| Concern | Where It Lives |
| --------- | --------------- |
| Rule evaluation logic | `engine/enforcement` |
| Knowledge injection algorithms | `engine/prompt` |
| Artifact validation | `engine/enforcement` + `engine/validation` |
| Prompt generation/assembly | `engine/prompt` |
| Impact analysis logic | `engine/graph` |
| Graph queries | `engine/graph` |
| Search queries | `engine/search` |
| Plugin installation | `engine/plugin` |
| State machine evaluation | `engine/workflow` |
| Agent system prompt generation | `engine/agent` |

If connector source contains `if/else` trees for rule evaluation or knowledge injection — that logic belongs in the engine.
