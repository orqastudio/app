# Getting Started

**Date:** 2026-03-02

Prerequisites, installation, and development commands for working on Forge.

---

## Prerequisites

| Tool | Minimum Version | Purpose | Install |
|------|----------------|---------|---------|
| **Rust** (rustc + cargo) | 1.75+ | Backend compilation | [rustup.rs](https://rustup.rs) |
| **Node.js** | 20+ | Frontend build tooling | [fnm](https://github.com/Schniz/fnm) or [nodejs.org](https://nodejs.org) |
| **npm** | 10+ | Package management | Bundled with Node.js |
| **Bun** | 1.0+ | Agent SDK sidecar compilation | `npm install -g bun` or [bun.sh](https://bun.sh) |
| **Tauri CLI** | 2.0+ | Tauri project management | `cargo install tauri-cli --version "^2"` |
| **Claude Code CLI** | Latest | AI provider (Claude Max subscription) | [claude.ai/download](https://claude.ai/download) |

### Platform-Specific Dependencies

Tauri v2 requires platform-specific build tools. See the [Tauri prerequisites guide](https://v2.tauri.app/start/prerequisites/) for your OS:

- **Windows:** Microsoft Visual Studio C++ Build Tools, WebView2 (pre-installed on Windows 10+)
- **macOS:** Xcode Command Line Tools (`xcode-select --install`)
- **Linux:** `build-essential`, `libwebkit2gtk-4.1-dev`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`

---

## Verify Installation

```bash
rustc --version       # 1.75+
cargo --version       # 1.75+
cargo tauri --version # 2.0+
node --version        # 20+
npm --version         # 10+
bun --version         # 1.0+
claude --version      # any
```

---

## Project Setup

Once the Tauri v2 + Svelte 5 project is initialized (Phase 1), setup will be:

```bash
# Clone the repository
git clone git@github.com:bobbibg/forge.git
cd forge

# Install frontend dependencies
npm install

# Build and run in development mode
cargo tauri dev
```

This section will be updated with exact commands once the scaffold exists.

---

## Development Commands

These commands will be available after Phase 1 scaffold is complete:

### Backend (Rust)

```bash
cargo build                       # Compile backend
cargo test                        # Run backend tests
cargo clippy -- -D warnings       # Lint (zero warnings policy)
cargo fmt --check                 # Format check
```

### Frontend (Svelte/TypeScript)

```bash
npm run dev                       # Vite dev server (frontend only)
npm run build                     # Production build
npm run check                     # svelte-check + TypeScript
npm run lint                      # ESLint
npm run test                      # Vitest
```

### Full Application

```bash
cargo tauri dev                   # Run Tauri app in development mode
cargo tauri build                 # Build distributable application
```

### Documentation

```bash
npx docsify-cli serve docs        # Browse docs locally at http://localhost:3000
```

---

## Project Structure

```
forge/
├── src-tauri/              # Rust backend (Tauri v2)
│   ├── src/                # Rust source code
│   ├── migrations/         # SQLite migrations
│   └── Cargo.toml          # Rust dependencies
├── src/                    # Svelte 5 frontend
│   ├── lib/                # Shared components, stores, types
│   └── routes/             # SvelteKit pages
├── docs/                   # Project documentation (Docsify)
├── tests/                  # E2E tests (Playwright)
├── .claude/                # Governance framework
│   ├── agents/             # Agent definitions
│   ├── rules/              # Enforcement rules
│   ├── skills/             # Loaded skills
│   └── hooks/              # Lifecycle hooks
├── TODO.md                 # Active task tracking
├── BLOCKERS.md             # Known blockers
└── AGENTS.md               # Cross-agent instructions
```

This structure will be created during Phase 1 scaffold. Currently only `docs/` and `.claude/` exist.

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Self-Learning Loop | N/A |
| Process Governance | Defines the development environment and commands that all agents and contributors use — standardizing the build/test/lint workflow that governance enforcement depends on. |

---

## Related Documents

- [Coding Standards](/development/coding-standards) — Code quality rules and patterns
- [Agentic Workflow](/process/workflow) — Task lifecycle and agent coordination
- [Tauri v2 Research](/research/tauri-v2) — Platform capabilities and plugin selections
- [Frontend Research](/research/frontend) — Library selections and patterns
