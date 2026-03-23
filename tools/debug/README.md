![License](https://img.shields.io/badge/license-BSL%201.1-blue)
![Status](https://img.shields.io/badge/status-pre--release-orange)

![OrqaStudio](https://github.com/orqastudio/.github/blob/main/assets/banners/banner-1680x240.png?raw=1)

# OrqaStudio Debug Tool

> **Pre-release** — APIs and interfaces may change without notice until v1.0.0.

A persistent Node.js dev controller and web dashboard for the OrqaStudio development workflow. Replaces `cargo tauri dev` with unified process management, real-time log streaming, and remote control over Vite and Tauri processes.

## Features

- **Unified process management** — Spawns and manages Vite and Tauri as child processes
- **Web dashboard** — Real-time log viewer and process control panel at `http://localhost:3001`
- **SSE log streaming** — All process output streamed via Server-Sent Events
- **Partial restarts** — Restart Tauri without killing Vite (and vice versa)
- **Cross-platform** — Windows (MSYS2/PowerShell), macOS, and Linux process handling
- **Remote control** — HTTP API for programmatic restarts from agents and scripts

## Usage

Run from the app directory (uses `process.cwd()` as the project root):

```bash
node <path>/dev.mjs dev            # Start dev environment (detached, exits when ready)
node <path>/dev.mjs start          # Start controller in foreground
node <path>/dev.mjs stop           # Graceful shutdown
node <path>/dev.mjs kill           # Force-kill all processes
node <path>/dev.mjs restart-tauri  # Restart Tauri only (Vite stays alive)
node <path>/dev.mjs restart-vite   # Restart Vite only
node <path>/dev.mjs restart        # Restart everything
node <path>/dev.mjs status         # Show process status
```

When used as a submodule in `orqastudio-dev`, the app Makefile provides shorthand targets (`make dev`, `make stop`, etc.).

## Part of OrqaStudio

This tool is part of the [OrqaStudio](https://github.com/orqastudio) ecosystem. See [orqastudio-app](https://github.com/orqastudio/orqastudio-app) for the main application.

---

## License

[BSL 1.1](LICENSE) — converts to Apache 2.0 four years after each version release.
