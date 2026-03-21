---
id: "DOC-2a7f1063"
title: "Getting Started"
category: "onboarding"
description: "Step-by-step guide to setting up the OrqaStudio development environment from scratch."
created: "2026-03-02"
updated: "2026-03-09"
sort: 1
relationships:
  - target: "AD-834fc71a"
    type: "documents"
    rationale: "Documentation page references AD-834fc71a"
  - target: "AD-dffc3d30"
    type: "documents"
    rationale: "Documentation page references AD-dffc3d30"
  - target: "AD-8b91f5a4"
    type: "documents"
    rationale: "Documentation page references AD-8b91f5a4"
  - target: "AD-afc78f6e"
    type: "documents"
    rationale: "Documentation page references AD-afc78f6e"
---
**Date:** 2026-03-02

Prerequisites, installation, and development commands for working on OrqaStudio™.

---


## Prerequisites

| Tool | Minimum Version | Purpose | Install |
|------|----------------|---------|---------|
| **Rust** (rustc + cargo) | 1.75+ | Backend compilation | [rustup.rs](https://rustup.rs) |
| **Node.js** | 20+ | Frontend build tooling | [fnm](https://github.com/Schniz/fnm) or [nodejs.org](https://nodejs.org) |
| **npm** | 10+ | Package management | Bundled with Node.js |
| **Bun** | 1.0+ | Agent SDK sidecar compilation | `npm install -g bun` or [bun.sh](https://bun.sh) |
| **Tauri CLI** | 2.0+ | Tauri project management | `cargo install tauri-cli --version "^2"` |
| **Claude Code CLI** | Latest | Current AI provider implementation (Claude Max subscription required); alternative providers will implement the same `Provider` interface | [claude.ai/download](https://claude.ai/download) |

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
git clone git@github.com:bobbibg/orqa-studio.git
cd orqa-studio

# Install all dependencies and build the sidecar
make install
make install-sidecar

# Run in development mode
make dev
```

For all development commands, see [Development Commands](commands.md).

### Project Initialization (New Project)

If scaffolding from scratch rather than cloning, use the Tauri CLI to initialize:

```bash
# Create a new Tauri v2 + SvelteKit project
cargo tauri init
```

The `cargo tauri init` command prompts for:
- **App name:** `orqa-studio`
- **Window title:** `OrqaStudio`
- **Frontend dev server URL:** `http://localhost:5173` (Vite default)
- **Frontend build command:** `npm run build`
- **Frontend dev command:** `npm run dev`

After initialization, the expected directory structure is:

```
orqa-studio/
├── backend/src-tauri/
│   ├── src/
│   │   └── main.rs              # Tauri entry point
│   ├── capabilities/
│   │   └── default.json         # Security permissions
│   ├── icons/                   # App icons
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri configuration
├── ui/                          # SvelteKit frontend (already exists)
├── package.json
├── svelte.config.js
├── vite.config.ts
└── tsconfig.json
```

### Tauri Plugins [AD-834fc71a](AD-834fc71a)

OrqaStudio requires the following Tauri v2 plugins. All are official and maintained in [tauri-apps/plugins-workspace](https://github.com/tauri-apps/plugins-workspace) unless noted.

| Plugin | Purpose | Notes |
|--------|---------|-------|
| `tauri-plugin-fs` | File system access + file watching | Enable `--features watch` |
| `tauri-plugin-shell` | Git/shell commands + sidecar management | Pre-declare commands with arg validators |
| `tauri-plugin-store` | App preferences and UI state | Not for secrets or relational data |
| `tauri-plugin-dialog` | File/folder selection dialogs | Selected paths auto-added to fs scope |
| `tauri-plugin-notification` | System notifications | Requires permission on some platforms |
| `tauri-plugin-updater` | Auto-update via GitHub Releases | |
| `tauri-plugin-window-state` | Persist window size/position | Automatic after registration |
| `tauri-plugin-autostart` | Optional launch at system startup | |
| `tauri-plugin-keyring` | API key storage in OS keychain | Community plugin |

> **Note:** SQLite persistence is implemented via `rusqlite` directly in `backend/src-tauri/src/db.rs`, not via `tauri-plugin-sql`. Do not add `tauri-plugin-sql` — it conflicts with the direct `rusqlite` approach ([AD-dffc3d30](AD-dffc3d30), [AD-8b91f5a4](AD-8b91f5a4)).

**Install Rust-side plugins** (run from `backend/src-tauri/`):

```bash
cd src-tauri

cargo add tauri-plugin-fs --features watch
cargo add tauri-plugin-shell
cargo add tauri-plugin-store
cargo add tauri-plugin-dialog
cargo add tauri-plugin-notification
cargo add tauri-plugin-updater
cargo add tauri-plugin-window-state
cargo add tauri-plugin-autostart
cargo add tauri-plugin-keyring

cd ..
```

**Install frontend plugin bindings:**

```bash
npm install @tauri-apps/plugin-fs
npm install @tauri-apps/plugin-shell
npm install @tauri-apps/plugin-store
npm install @tauri-apps/plugin-dialog
npm install @tauri-apps/plugin-notification
npm install @tauri-apps/plugin-updater
npm install @tauri-apps/plugin-window-state
npm install @tauri-apps/plugin-autostart
npm install tauri-plugin-keyring-api
```

Each plugin must also be registered in the Tauri app builder (`backend/src-tauri/src/main.rs`) and have its permissions declared in `backend/src-tauri/capabilities/default.json`. See [AD-834fc71a](AD-834fc71a) and [Tauri v2 Research](RES-00c5dbc3) for configuration details.

### Frontend Dependencies [AD-afc78f6e](AD-afc78f6e)

OrqaStudio's frontend depends on these libraries, selected in [AD-afc78f6e](AD-afc78f6e):

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
orqa.db
orqa.db-wal
orqa.db-shm
```

See the root `.gitignore` file for the complete list, which also covers IDE files, OS artifacts, environment files, temporary output, and Claude Code workspace directories.

---


## Project Structure

```
orqa-studio/
├── backend/src-tauri/              # Rust backend (Tauri v2)
│   ├── src/                # Rust source code
│   ├── capabilities/       # Tauri security permissions (JSON)
│   ├── migrations/         # SQLite migrations (.sql files)
│   └── Cargo.toml          # Rust dependencies
├── ui/                     # Svelte 5 frontend
│   ├── lib/                # Shared components, stores, types
│   └── routes/             # SvelteKit pages
├── tests/                  # E2E tests (Playwright)
├── .orqa/                  # Governance framework (source of truth)
│   ├── team/
│   │   ├── agents/         # Agent definitions
│   │   └── skills/         # Loaded skills
│   ├── governance/
│   │   ├── rules/          # Enforcement rules
│   │   ├── hooks/          # Lifecycle hooks
│   │   ├── hookify/        # Real-time enforcement rules
│   │   └── decisions/      # Architecture decision records
│   ├── planning/
│   │   ├── ideas/          # Captured ideas
│   │   ├── epics/          # Epics
│   │   ├── tasks/          # Graduated tasks
│   │   ├── milestones/     # Milestones
│   │   └── research/       # Research artifacts
│   └── documentation/      # Architecture and process docs
├── .claude/                # Optional CLI symlink compatibility layer
└── AGENTS.md               # Cross-agent instructions
```

`.orqa/` contains the governance framework and documentation. `backend/src-tauri/` and `ui/` hold the application code. All directories shown above are present in the repository.

---


## Related Documents

- [Development Commands](commands.md) — All Makefile targets with descriptions and underlying commands
- Coding Standards — Code quality rules and patterns
- Agentic Workflow — Task lifecycle and agent coordination
- [Tauri v2 Research](RES-00c5dbc3) — Platform capabilities and plugin selections
- [Frontend Research](RES-df5560cb) — Library selections and patterns
