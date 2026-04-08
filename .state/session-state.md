# Session State — 2026-04-08

## What was done

### Navigation menu fix (CRITICAL)

- Root cause: daemon's GET /plugins/:name round-tripped through lossy Rust struct
- Second cause: scoped plugin names with "/" broke axum route matching
- Third cause: daemon wrapped manifest in PluginManifestResponse, frontend expected raw
- Fix: raw JSON read, percent-encoding, manifest unwrap

### Daemon restart fix

- SO_REUSEADDR via socket2 prevents EADDRINUSE on restart
- Old daemon socket in TIME_WAIT no longer blocks new instance

### Frontend timing fix

- loadNavTree() and artifactGraphSDK.initialize() now await pluginsReady
- settingsStore.initialize() clears stale polling intervals on HMR re-entry

### Build hygiene

- Root tsconfig.json had declaration:true without noEmit — caused 1100+ orphaned .js/.d.ts in src/
- Fixed: noEmit:true, removed declaration. Added gitignore defense-in-depth.
- rustEnv() auto-detects Windows SDK RC.EXE path for tauri-winres

### Lint compliance

- All clippy::too_many_lines fixed via helper extraction
- All clippy::map_unwrap_or fixed
- Missing stories added for ColorDot, GlowDot, IndentedBlock

## Known issues

### Sidecar not in devtools node graph

- Sidecar is spawned on-demand per session (streaming.rs), not tracked in process_snapshots
- Devtools only reads process_snapshots from /health, so sidecar never appears
- Architecture: sidecars are plugins with provides.sidecar, daemon should spawn/track them
- LSP + MCP are global infrastructure services, not sidecars

### LSP and MCP crash on startup

- Both exit with code 1 within 260ms of spawning
- Needs investigation — likely port conflict or missing dependency

### Devtools graph layout

- Currently tiered dependency layout, should be hub-spoke with daemon as hub

## Next priorities

1. Fix LSP/MCP crash (they're infrastructure for all sidecars)
2. Sidecar lifecycle: daemon spawns from plugin registrations, tracks in snapshots
3. Devtools hub-spoke graph layout
4. Windows icon cache (binary has correct icon, Windows displays cached)
