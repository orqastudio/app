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

### Clone and Install

```bash
# Clone the repository
git clone git@github.com:bobbibg/forge.git
cd forge

# Install frontend dependencies
npm install

# Build and run in development mode
cargo tauri dev
```

### Project Initialization (New Project)

If scaffolding from scratch rather than cloning, use the Tauri CLI to initialize:

```bash
# Create a new Tauri v2 + SvelteKit project
cargo tauri init
```

The `cargo tauri init` command prompts for:
- **App name:** `forge`
- **Window title:** `Forge`
- **Frontend dev server URL:** `http://localhost:5173` (Vite default)
- **Frontend build command:** `npm run build`
- **Frontend dev command:** `npm run dev`

After initialization, the expected directory structure is:

```
forge/
├── src-tauri/
│   ├── src/
│   │   └── main.rs              # Tauri entry point
│   ├── capabilities/
│   │   └── default.json         # Security permissions
│   ├── icons/                   # App icons
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri configuration
├── src/                         # SvelteKit frontend (already exists)
├── package.json
├── svelte.config.js
├── vite.config.ts
└── tsconfig.json
```

### Tauri Plugins (AD-012)

Forge requires the following Tauri v2 plugins. All are official and maintained in [tauri-apps/plugins-workspace](https://github.com/tauri-apps/plugins-workspace) unless noted.

| Plugin | Purpose | Notes |
|--------|---------|-------|
| `tauri-plugin-sql` | SQLite persistence | Enable `--features sqlite` |
| `tauri-plugin-fs` | File system access + file watching | Enable `--features watch` |
| `tauri-plugin-shell` | Git/shell commands + sidecar management | Pre-declare commands with arg validators |
| `tauri-plugin-store` | App preferences and UI state | Not for secrets or relational data |
| `tauri-plugin-dialog` | File/folder selection dialogs | Selected paths auto-added to fs scope |
| `tauri-plugin-notification` | System notifications | Requires permission on some platforms |
| `tauri-plugin-updater` | Auto-update via GitHub Releases | |
| `tauri-plugin-window-state` | Persist window size/position | Automatic after registration |
| `tauri-plugin-autostart` | Optional launch at system startup | |
| `tauri-plugin-persisted-scope` | Remember file system permissions across restarts | Pairs with fs plugin |
| `tauri-plugin-keyring` | API key storage in OS keychain | Community plugin |

**Install Rust-side plugins** (run from `src-tauri/`):

```bash
cd src-tauri

cargo add tauri-plugin-sql --features sqlite
cargo add tauri-plugin-fs --features watch
cargo add tauri-plugin-shell
cargo add tauri-plugin-store
cargo add tauri-plugin-dialog
cargo add tauri-plugin-notification
cargo add tauri-plugin-updater
cargo add tauri-plugin-window-state
cargo add tauri-plugin-autostart
cargo add tauri-plugin-persisted-scope
cargo add tauri-plugin-keyring

cd ..
```

**Install frontend plugin bindings:**

```bash
npm install @tauri-apps/plugin-sql
npm install @tauri-apps/plugin-fs
npm install @tauri-apps/plugin-shell
npm install @tauri-apps/plugin-store
npm install @tauri-apps/plugin-dialog
npm install @tauri-apps/plugin-notification
npm install @tauri-apps/plugin-updater
npm install @tauri-apps/plugin-window-state
npm install @tauri-apps/plugin-autostart
npm install @tauri-apps/plugin-persisted-scope
npm install tauri-plugin-keyring-api
```

Each plugin must also be registered in the Tauri app builder (`src-tauri/src/main.rs`) and have its permissions declared in `src-tauri/capabilities/default.json`. See [AD-012](/architecture/decisions#ad-012-tauri-plugin-selections) and [Tauri v2 Research](/research/tauri-v2) for configuration details.

### Frontend Dependencies (AD-013)

Forge's frontend depends on these libraries, selected in [AD-013](/architecture/decisions#ad-013-frontend-library-selections):

| Library | Purpose |
|---------|---------|
| `shadcn-svelte` | UI component library (Svelte 5 native, accessible primitives) |
| PaneForge | Resizable panel layout (shadcn-svelte's `Resizable` component) |
| `@humanspeak/svelte-markdown` | Markdown rendering (Svelte 5 runes, caching) |
| `svelte-highlight` | Syntax highlighting in code blocks (highlight.js wrapper) |
| `lucide-svelte` | Icon library (consistent with shadcn-svelte ecosystem) |
| `svelte-codemirror-editor` | CodeMirror 6 wrapper for markdown editing (Svelte 5 runes) |
| LayerChart | Charts and visualizations (shadcn-svelte's `Chart` component) |
| `@tauri-apps/api` | Tauri IPC (`invoke()`, `Channel`, events) |

**Install frontend dependencies:**

```bash
# Tauri IPC
npm install @tauri-apps/api

# UI components (shadcn-svelte is added via its CLI, not npm install)
npx shadcn-svelte@latest init

# Markdown and code
npm install @humanspeak/svelte-markdown svelte-highlight

# Icons
npm install lucide-svelte

# Editor
npm install svelte-codemirror-editor codemirror @codemirror/lang-markdown

# Charts (installed via shadcn-svelte CLI as needed)
# npx shadcn-svelte@latest add chart
```

PaneForge and LayerChart are installed as shadcn-svelte components (`Resizable` and `Chart` respectively) using `npx shadcn-svelte@latest add resizable` and `npx shadcn-svelte@latest add chart`.

### .gitignore

The project `.gitignore` must include these entries (most are already configured):

```gitignore
# Rust
/target/

# Node
node_modules/
dist/

# SvelteKit
.svelte-kit/

# Build output
/build/

# SQLite database (local data, not committed)
forge.db
forge.db-wal
forge.db-shm
```

See the root `.gitignore` file for the complete list, which also covers IDE files, OS artifacts, environment files, temporary output, and Claude Code workspace directories.

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

---

## Project Structure

```
forge/
├── src-tauri/              # Rust backend (Tauri v2)
│   ├── src/                # Rust source code
│   ├── capabilities/       # Tauri security permissions (JSON)
│   ├── migrations/         # SQLite migrations (.sql files)
│   └── Cargo.toml          # Rust dependencies
├── src/                    # Svelte 5 frontend
│   ├── lib/                # Shared components, stores, types
│   └── routes/             # SvelteKit pages
├── docs/                   # Project documentation
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

Currently only `docs/` and `.claude/` exist. The `src-tauri/` and `src/` directories will be created during Phase 1 scaffold using `cargo tauri init`.

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
