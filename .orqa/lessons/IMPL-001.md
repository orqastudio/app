---
id: IMPL-001
title: "Run vite optimize after new dependencies"
category: dev-environment
recurrence: 1
status: active
promoted_to: null
created: 2026-03-03
updated: 2026-03-03
---

# Run `npx vite optimize` after new dependencies

## Pattern
After `npm install` or when Vite encounters new dependencies for the first time, `cargo tauri dev` launches the app but Vite triggers a dependency optimization reload. On Windows, the Tauri WebView2 webview fails to reconnect after this reload, leaving the app stuck on a white screen. The app only works after a full restart.

## Fix
Run `npx vite optimize` before starting or restarting the dev server whenever new dependencies have been added or on first launch after cloning. This pre-bundles dependencies so Vite skips the runtime optimization reload.

```bash
npx vite optimize
cargo tauri dev
```
