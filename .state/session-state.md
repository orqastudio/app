# Session State — 2026-04-09

## What was done this session

### Dev environment investigation

- Full process hierarchy mapped: ProcessManager → daemon → LSP/MCP, search, Forgejo, Tauri apps, file watchers
- Identified orphan process risks, missing health checks, port mismatches
- Forgejo container recreated on correct 10xxx ports, SSH config fixed

### Containerisation (IDEA-4d48c4a7)

- Wrote full proposal with hybrid approach (containerised services + native UI)
- Two modes: `orqa dev` (containerised daemon, stable backend) and `orqa dev --include-backend` (native, full hot-reload)
- Phase 1 COMPLETE: search consolidation, retire standalone search server, daemon auto-restart
- Phases 2-5 COMPLETE: Dockerfile.daemon, docker-compose.dev.yml, ProcessManager integration, unified startup

### Search consolidation (daemon is single source of truth)

- MCP server search tools now proxy to daemon HTTP endpoints via DaemonClient
- Removed duplicate SearchEngine/DuckDB from MCP server
- Retired standalone orqa-search-server binary
- `orqa index` rewired to call daemon HTTP endpoints

### Daemon improvements

- Auto-restart for crashed LSP/MCP subprocesses (exponential backoff, max 5 retries)
- ORQA_HEADLESS=1 support — skips system tray in container mode
- System tray: left-click focuses/launches app, "Open DevTools" menu item, cross-platform (Win32/macOS/Linux)
- Fixed latent libc dependency bug exposed by Linux container build

### Port consolidation

- `infrastructure/ports.json` is single source of truth for all port allocation
- Consumed by Rust (compile-time include_str!) and TypeScript (runtime import)
- Vite port corrected to 10420 everywhere
- `orqa check ports` validates static configs against ports.json
- Zero hardcoded port numbers in source code

### Dev environment quality of life

- `make link` target for fast CLI relinking after reboot
- Forgejo unified into `orqa dev` — `orqa hosting up/down` redirects

## Commits this session

- `d1f32738b` — ports.json SOT, search consolidation, daemon auto-restart, tray UX (56 files)
- `f80a66f5f` — Containerise dev environment: Dockerfile, compose, ProcessManager integration (17 files)

## Verified working

- Forgejo git server on 10xxx ports (10030 HTTP, 10222 SSH), SSH authenticated
- Docker image `orqa-daemon:latest` builds and runs (587MB, 3 binaries, ORQA_HEADLESS=1)
- `docker compose -f infrastructure/docker-compose.dev.yml config` validates
- All Rust workspace compiles clean (cargo check)
- All TypeScript compiles clean (npx tsc)
- Pre-commit hooks pass (lint, format, markdownlint, stories)

## Known issues

### LSP single-client TCP mode

LSP server exits after one editor disconnects (single-client design). Auto-restart
handles this now, but making it loop like MCP would be cleaner. Low priority.

### Docker image size

587MB due to GTK3 + libxdo pulled in by tray-icon dependency (even in headless mode).
A `headless` Cargo feature flag excluding tray-icon would reduce this significantly.
Future optimisation.

## Next priorities

1. Sidecar lifecycle — daemon spawns sidecars from plugin registrations, tracks in snapshots
2. Devtools hub-spoke graph layout with daemon as central hub
3. Content loading verification — nav items populate, artifact lists work
4. Test `orqa dev` end-to-end with containerised daemon (first real usage)
5. Consider `orqa-tray` native binary for containerised mode (currently no tray in container mode)
