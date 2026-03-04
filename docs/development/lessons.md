# Implementation Lessons

**Date:** 2026-03-02

Implementation lessons captured from code review failures. Each entry documents a pattern that caused a review FAIL, with recurrence tracking and promotion status.

When a lesson reaches recurrence >= 2, the `agent-maintainer` promotes it to a rule, coding standard addition, or skill update.

## Format

```
### IMPL-NNN: [Title]

**Date:** YYYY-MM-DD | **Recurrence:** N | **Promoted to:** [rule/standard/skill or "pending"]

**Pattern:** What went wrong
**Fix:** What to do instead
**Found by:** [code-reviewer | qa-tester | ux-reviewer]
```

---

### IMPL-001: Run `npx vite optimize` after new dependencies

**Date:** 2026-03-03 | **Recurrence:** 1 | **Promoted to:** pending

**Pattern:** After `npm install` or when Vite encounters new dependencies for the first time, `cargo tauri dev` launches the app but Vite triggers a dependency optimization reload (`✨ optimized dependencies changed. reloading`). On Windows, the Tauri WebView2 webview fails to reconnect after this reload, leaving the app stuck on a white screen. The app only works after a full restart of `cargo tauri dev`.

**Fix:** Run `npx vite optimize` before starting or restarting the dev server whenever new dependencies have been added (`npm install`, `bun install`) or on first launch after cloning. This pre-bundles dependencies so Vite skips the runtime optimization reload.

```bash
# After installing new deps or on first launch
npx vite optimize
cargo tauri dev
```

**Found by:** qa-tester (manual testing)

---

### IMPL-002: Kill existing dev server processes before starting new ones

**Date:** 2026-03-03 | **Recurrence:** 1 | **Promoted to:** pending

**Pattern:** Starting `cargo tauri dev` while a previous instance is still running (or its port is held by a lingering process) causes a `Port 1420 is already in use` error and the app fails to launch. This happens when the previous window was closed but the process wasn't fully terminated, or when restarting after code changes.

**Fix:** Before starting the dev server, check for and kill any existing `orqa-studio.exe` or `node.exe` processes bound to port 1420. The orchestrator should do this automatically so the user never has to run manual commands.

```bash
# Before starting dev server
tasklist | grep -i "orqa-studio.exe" | grep -v CurseForge
# Kill any found PIDs
taskkill //PID <pid> //F
# Then start fresh
cargo tauri dev
```

**Found by:** qa-tester (manual testing)

---

### IMPL-003: Orchestrator must prepare and launch the dev environment for the user

**Date:** 2026-03-03 | **Recurrence:** 1 | **Promoted to:** pending

**Pattern:** After making code changes, the orchestrator reported what needed to happen ("you'll need to restart `cargo tauri dev`") but left the user to do it manually. The user should not need to run terminal commands — the orchestrator should handle the full lifecycle: kill old processes, pre-bundle dependencies, and start the dev server.

**Fix:** When the orchestrator makes changes that require a dev server restart (Rust changes, new dependencies, config changes), it must automatically:
1. Kill any existing `orqa-studio.exe` / dev server processes
2. Run `npx vite optimize` if dependencies changed (IMPL-001)
3. Start `cargo tauri dev` in the background
4. Verify the server started successfully before reporting to the user

Never tell the user "you'll need to restart" — just do it.

**Found by:** user feedback
