# Architecture Migration Context (persistent — not overwritten by stop hook)

## Status: Complete (2026-03-22)

All 8 migration epics are done. The ecosystem is aligned with the composability architecture.

## Architecture

See `.claude/plans/glimmering-orbiting-lobster.md` for the full diagram.

- **Rust binary** (`libs/validation`) = canonical artifact engine. Parse, query, hook, content, daemon.
- **Daemon** = standalone HTTP server at localhost:3002. Start with `orqa daemon start`.
- **CLI** = bridge to Rust. All commands via `orqa`. `make install` bootstraps.
- **Connector** = thin adapter. Hooks call daemon. Zero enforcement logic.
- **LSP/MCP** = thin interfaces over daemon.
- **Logger** = `libs/logger`, zero deps.
- **App** = UI consumer. 569 tests pass.

## What the orchestrator needs to know

1. Connector hooks are in `dist/hooks/*.js` (compiled from `src/hooks/*.ts`)
2. hooks.json references `dist/hooks/` not `hooks/scripts/`
3. The daemon should be started for fast hook evaluation: `orqa daemon start`
4. Without daemon, hooks fall back to spawning the binary directly (slower but works)
5. The TS validator fallback was deleted — Rust binary is the only engine
6. All frontmatter parsing goes through Rust — no hand-rolled JS parsers

## Pending work

- EPIC-fba4debd: System tray for daemon lifecycle (captured, not started)
