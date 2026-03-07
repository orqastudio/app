---
title: "Architecture Overview"
category: development
tags: []
created: 2026-03-07
updated: 2026-03-07
---

# Architecture Overview

**Date:** 2026-03-07

A contributor-friendly introduction to OrqaStudio's architecture. For detailed decisions, see [Architecture Decisions](../architecture/decisions.md).

---

## System Layers

OrqaStudio is a Tauri v2 desktop application with three main layers:

```
┌─────────────────────────────────────────┐
│  Frontend (Svelte 5 + TypeScript)       │
│  ui/                                    │
│    Components, stores, pages            │
├─────────────────────────────────────────┤
│  IPC Boundary (Tauri invoke / Channel)  │
├─────────────────────────────────────────┤
│  Backend (Rust)                         │
│  src-tauri/src/                         │
│    Commands, domain services, repos     │
├─────────────────────────────────────────┤
│  Persistence (SQLite)                   │
│  src-tauri/migrations/                  │
├─────────────────────────────────────────┤
│  AI Sidecar (TypeScript / Bun)          │
│  sidecar/                               │
│    Provider interface, NDJSON protocol  │
└─────────────────────────────────────────┘
```

### Frontend (`ui/`)

- **Framework:** Svelte 5 with runes (`$state`, `$derived`, `$effect`, `$props`)
- **Components:** shadcn-svelte primitives, Lucide icons, Tailwind CSS
- **State:** Class-based rune stores in `.svelte.ts` files
- **IPC:** All backend communication via `invoke()` from `@tauri-apps/api`
- **Rule:** Display components receive props only. Stores call `invoke()`, components read stores.

### IPC Boundary

- `invoke()` for request/response (CRUD, settings, queries)
- `Channel<T>` for streaming (AI conversation events)
- Frontend and backend types must match — Rust structs derive `Serialize`/`Deserialize`, TypeScript interfaces mirror them

### Backend (`src-tauri/src/`)

- **Commands** (`commands/`): Thin `#[tauri::command]` handlers that delegate to domain services
- **Domain** (`domain/`): Business logic, pure functions where possible
- **Repositories** (`repo/`): SQLite persistence via `rusqlite`
- **Sidecar management** (`sidecar/`): Protocol types and process control for the AI sidecar
- **Error handling:** All functions return `Result<T, OrqaError>`. No `unwrap()` in production.

### AI Sidecar (`sidecar/`)

A separate TypeScript process that bridges AI provider SDKs (currently Claude Agent SDK) with the Rust backend:

```
Agent SDK → Sidecar (Bun) → NDJSON stdout → Rust → Channel<T> → Svelte
```

- Communicates with Rust via NDJSON over stdin/stdout
- Implements the `Provider` interface — new AI providers implement this interface
- Compiled to a standalone binary with `bun build --compile`

### Persistence (`src-tauri/migrations/`)

- SQLite database for sessions, messages, projects, settings
- Migrations are `.sql` files applied idempotently on startup
- `.claude/` files on disk are the source of truth for governance artifacts
- SQLite is a derived cache that can be rebuilt from disk

---

## Key Boundaries

| Boundary | Rule |
|----------|------|
| Frontend → Backend | Only via `invoke()` or `Channel<T>`. No direct FFI. |
| Backend → Database | Only via repository modules. Commands never write SQL directly. |
| Backend → AI | Only via sidecar process. NDJSON protocol is provider-agnostic. |
| Components → Stores | Components read stores. Only stores call `invoke()`. |

---

## Directory Map

| Path | Contents |
|------|----------|
| `ui/lib/components/` | Svelte components (shared, layout, feature-specific) |
| `ui/lib/stores/` | Rune stores (`.svelte.ts` files) |
| `ui/lib/types/` | TypeScript type definitions |
| `ui/routes/` | SvelteKit pages |
| `src-tauri/src/commands/` | Tauri command handlers |
| `src-tauri/src/domain/` | Domain logic and services |
| `src-tauri/src/repo/` | Database repositories |
| `src-tauri/src/sidecar/` | Sidecar protocol and types |
| `sidecar/src/` | AI provider implementations |
| `docs/` | Project documentation |
| `.claude/` | Governance framework (agents, rules, skills, hooks) |

---

## Related Documents

- [Architecture Decisions](../architecture/decisions.md) — Detailed AD records
- [Coding Standards](coding-standards.md) — Code quality rules
- [Streaming Pipeline](../architecture/streaming-pipeline.md) — AI streaming architecture
- [IPC Commands](../architecture/ipc-commands.md) — Command inventory
- [SQLite Schema](../architecture/sqlite-schema.md) — Database design
