## Session: 2026-03-30

### Fixes Applied (uncommitted)

**Bug 1: `find_plugin_dir` only scanned 1 level deep (ROOT CAUSE)**
- `app/src-tauri/src/commands/plugin_commands.rs` — `find_plugin_dir` now scans `plugins/<taxonomy>/<plugin>/` (2 levels) and `connectors/<plugin>/` (1 level)
- This was THE reason no artifacts showed — plugins couldn't be loaded → no `defaultNavigation` → no sidebar items

**Bug 2: Fresh DB had no setup_version**
- `app/src-tauri/src/lib.rs` — auto-complete first-run setup when `ORQA_PROJECT_ROOT` env var is set (dev mode)

**Bug 3: Fresh DB had no active project**
- `app/src-tauri/src/lib.rs` — auto-open project from `ORQA_PROJECT_ROOT` env var
- `cli/src/commands/dev.ts` — pass `ORQA_PROJECT_ROOT` env var to `cargo tauri dev`

**Bug 4: Dashboard control file path wrong**
- `tools/debug/dev.mjs` — fixed path from `../tmp/dev-controller.json` to `.state/dev-controller.json`

**Bug 5: sendBeacon CORS preflight**
- `app/src/lib/utils/dev-console.ts` — changed Blob type from `application/json` to `text/plain`

**Bug 6: `defaultNavigation` missing from Rust PluginManifest**
- `engine/plugin/src/manifest.rs` — added `default_navigation: Vec<serde_json::Value>` with `#[serde(rename = "defaultNavigation")]`
- `engine/plugin/src/constraints.rs` — added field to test helper

**Bug 7: Only software-kanban had `defaultNavigation`**
- Added `defaultNavigation` to 5 more plugins:
  - `plugins/workflows/agile-discovery/orqa-plugin.json` — Discovery group
  - `plugins/workflows/agile-planning/orqa-plugin.json` — Planning group
  - `plugins/workflows/agile-documentation/orqa-plugin.json` — Documentation group
  - `plugins/workflows/agile-review/orqa-plugin.json` — Learning group
  - `plugins/workflows/core/orqa-plugin.json` — Agents group

**Bug 8: Duplicate nav groups not merged**
- `libs/sdk/src/stores/navigation.svelte.ts` — `_buildDefaultNavTree()` now merges groups by key, unioning children

**Bug 9: Plugin relationship type conflicts blocked registration**
- `libs/sdk/src/plugins/plugin-registry.svelte.ts` — `checkConflicts()` now merges (unions) from/to type arrays instead of rejecting when relationship key+inverse match

**Bug 10: CSP blocked plugin view scripts**
- `app/src-tauri/tauri.conf.json` — added `https://asset.localhost` to `script-src` and `connect-src`

**Diagnostic logging added:**
- `app/src-tauri/src/commands/artifact_commands.rs` — tracing for scan results
- `app/src/lib/plugins/loader.ts` — logging for plugin discovery and defaultNavigation

### Current State

- App starts, auto-completes setup, auto-opens project ✓
- Backend returns 16 groups / 1583 artifact nodes ✓
- Plugins load successfully with from/to type merging ✓
- 6 navigation groups: Discovery, Delivery, Planning, Documentation, Learning, Agents ✓
- Plugin views fail to load via asset protocol (CSP fix applied, needs rebuild) ⚠

### Remaining Issues

1. **Plugin view loading** — CSP fix applied but needs Rust rebuild + Vite restart
2. **Daemon showing disconnected** — Frontend health check sees 0 artifacts/rules
3. **Stale Vite process on port 10420** — `orqa dev kill` doesn't always kill Vite. Need to fix the kill logic.
4. **Diagnostic logging should be cleaned up** before committing

### Investigation Artifacts
- `.state/investigation/01-rust-pipeline.md` through `08-nav-entries-added.md`
