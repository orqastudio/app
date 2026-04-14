---
name: Daemon as business logic boundary
description: The daemon owns all database access — MCP/LSP are access protocols, not application boundaries
type: project
---

The daemon is the business logic boundary. It owns all database access (SQLite/SurrealDB). MCP and LSP are access protocols — thin clients that call the daemon, not standalone applications with their own logic.

**Why:** SQLite contention was a blocker when multiple processes accessed the DB directly. The SeaORM migration (phases C-E) established the daemon as sole DB writer. All app and devtools operations go through the daemon's HTTP API via the libs/db typed client.

**How to apply:** Never access the database directly from the app or CLI. All storage operations go through the daemon HTTP API. The connector source calls engine crates directly for generation, but runtime operations go through the daemon.
