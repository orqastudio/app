# Next Session: Fix Artifacts Not Showing in App

## Critical Bug

The app connects to the daemon but shows NO artifacts despite 1718 valid artifacts on disk.

## Root Cause Chain (traced)

1. `project_open` stores the path in SQLite when user opens a project
2. `cargo tauri dev` runs from `app/` — the stored path is `app/` not the project root
3. All Tauri commands use `active_project_path()` → gets wrong path from DB
4. `artifact_entries_from_schema()` looks for `.orqa/schema.composed.json` at wrong path → returns empty
5. Plugin loader calls `plugin_get_manifest` → Rust scans `plugins/` relative to wrong path → "plugin not found"
6. No plugins registered = no schemas = no artifacts

## Fixes Already Applied (in source, need rebuild + DB clear)

- `app/src-tauri/src/commands/helpers.rs` — `active_project_path()` walks up to find `.orqa/`
- `app/src-tauri/src/commands/project_commands.rs` — `project_open()` walks up before storing path
- `engine/artifact/src/lib.rs` — `artifact_entries_from_schema()` logs warnings instead of silent empty return
- `app/src/lib/plugins/loader.ts` — defensive `provides` normalization + better error logging
- `libs/sdk/src/plugins/plugin-registry.svelte.ts` — null-safe `?? []` on all provides arrays

## What You Need To Do

1. **Delete the stale database**: `rm "$APPDATA/com.orqa.studio/orqa.db"`
2. **Stop orqa dev**: `orqa dev stop`
3. **Restart**: `orqa dev` — this rebuilds Rust (picks up the fixes), creates fresh DB, opens project with correct root path
4. **If still broken**: Add temporary logging to trace the actual path being used:
   - In `project_open()`: `tracing::info!("project_open: raw={}, canonical={}", path, canonical);`
   - In `active_project_path()`: `tracing::info!("active_project_path: stored={}", path);`
   - Check `.state/daemon.log` for the path values
5. **Verify schema exists**: `ls .orqa/schema.composed.json` — if missing, run `orqa install` first

## Other Open Runtime Issues

- Dashboard CORS: `Access-Control-Allow-Origin: *` doesn't work with credentials mode. Fix in `tools/debug/dev.mjs` — set specific origin instead of wildcard.
- System tray: right-click menu + left-click open — code is committed but needs daemon binary rebuild.
- Some Vite HMR-loaded code still references old port 10401 (cached in browser — hard refresh should fix).

## Session Context

Migration is COMPLETE. All phases done, targets/ removed, architecture implemented. This is dogfooding/stabilization. The artifact display is the last critical blocker.

Commits this session: `git log --oneline e730f08ec..HEAD`
Session state: `.state/session-state.md`
Design docs: `.state/design/enforcement-plugin-model.md`, `.state/design/frontend-runtime-derivation.md`
