# Session State — 2026-04-08

## What was done this session

### Plugin loading pipeline (FIXED — 3 root causes)

- Daemon `GET /plugins/:name` round-tripped through lossy Rust struct, dropping
  `defaultNavigation`, `requires`, `workflows`, `knowledge`, `semantics`.
  Now reads raw JSON file directly.
- Scoped plugin names with `/` broke axum `/{name}` route matching.
  DaemonClient percent-encodes `/` as `%2F`.
- Daemon wrapped response in `{name, path, manifest}` but frontend expected raw.
  `plugin_get_manifest` extracts `response["manifest"]`.

### Daemon restart reliability (FIXED)

- `SO_REUSEADDR` via socket2 on TCP listener prevents EADDRINUSE (OS error 10048)
  when old socket is in TIME_WAIT after restart.

### Frontend timing (FIXED)

- `loadNavTree()` and `artifactGraphSDK.initialize()` now await `pluginsReady`.
- `settingsStore.initialize()` clears stale polling intervals on HMR re-entry.

### Build hygiene (FIXED)

- Root `tsconfig.json` had `declaration:true` without `noEmit` — was the cause of
  1100+ orphaned `.js`/`.d.ts` files in `src/` directories. Fixed with `noEmit:true`.
- `rustEnv()` auto-detects Windows SDK RC.EXE path for tauri-winres.
- Gitignore rules added for `src/**/*.js` patterns across libs, cli, plugins.

### Enforce pipeline (FIXED)

- Removed obsolete `artifact-validation` engine from core-framework plugin manifest.
- Deleted migration-period scripts (`validate-artifacts.mjs`, `validate-connector-output.mjs`).
- File batching in `runAction` for Windows 8191-char command line limit.

### Lint compliance (FIXED)

- All `clippy::too_many_lines` fixed via helper extraction (sidecar, logging, events, discovery, test helpers).
- All `clippy::map_unwrap_or` fixed in devtools lib.rs.
- `clippy::ref_option` fixed in events.rs.
- Missing stories added for ColorDot, GlowDot, IndentedBlock.
- JSDoc params added to enforce.ts new functions.
- `no-explicit-any` fixed in vitest.svelte.d.ts.
- Generated `.js`/`.d.ts` files removed from git tracking.

### Observability

- Tracing added to `daemon_health` and `sidecar_status` IPC commands.

## Verified working

- Navigation menu populates with plugin-contributed groups (Discovery, Planning, Delivery, etc.)
- Daemon health shows connected with 1622 artifacts, 60 rules
- Sidecar shows connected in status bar
- Health polling recovers after HMR hot-reload
- `orqa dev` builds succeed with RC.EXE auto-detection
- Pre-commit hook passes clean (zero errors, zero warnings)

## Known issues

### Sidecar not in devtools node graph

- Sidecar is spawned on-demand per session (`streaming.rs`), not tracked in `process_snapshots`
- Devtools only reads `process_snapshots` from `/health`, so sidecar never appears
- Architecture: sidecars are plugins with `provides.sidecar`, daemon should spawn/track them
- LSP + MCP are global infrastructure services that sidecars USE, not sidecars themselves

### LSP and MCP crash on startup

- Both exit with code 1 within 260ms of spawning
- Needs investigation — likely port conflict or missing dependency

### Devtools graph layout

- Currently tiered dependency layout, should be hub-spoke with daemon as hub

### Windows taskbar icon

- Binary has correct icon embedded, Windows displays cached version
- Need to clear Windows icon cache

## Next priorities

1. Investigate LSP/MCP crash — they're infrastructure for all sidecars
2. Sidecar lifecycle: daemon spawns from plugin registrations, tracks in snapshots
3. Devtools hub-spoke graph layout with daemon as central hub
4. Content loading verification — nav items show, need to verify artifact lists populate
