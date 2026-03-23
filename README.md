![License](https://img.shields.io/badge/license-BSL%201.1-blue)
![Status](https://img.shields.io/badge/status-pre--release-orange)
![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte-FF3E00?logo=svelte&logoColor=white)
![Shell](https://img.shields.io/badge/Shell-4EAA25?logo=gnubash&logoColor=white)

![OrqaStudio](https://github.com/orqastudio/.github/blob/main/assets/banners/banner-1680x240.png?raw=1)

# OrqaStudio

> **Pre-release** — APIs and interfaces may change without notice until v1.0.0.

AI-assisted clarity engine for structured thinking and adaptive action.

## Prerequisites

- **Git** — [git-scm.com](https://git-scm.com/)
- **Docker** — for the local git server

Node.js 22+ and Rust are checked and installed automatically during setup.

## Setup

```bash
git clone git@github.com:orqastudio/app.git
cd app
make install
```

This runs the full bootstrap:

1. **Prereqs** — checks git, node, npm, rust, cargo. Offers to install Node (via fnm) and Rust (via rustup) if missing.
2. **Dependencies** — `npm install` (workspaces resolve all packages), `cargo fetch` (workspace).
3. **Build** — builds all TypeScript packages in dependency order, then the Svelte app.
4. **Plugins** — syncs plugin content to `.orqa/`.
5. **Smoke test** — verifies the CLI, artifact graph, compilation, and type checks.

After install, the `orqa` CLI is on your PATH.

## Daily Use

```bash
orqa install              # Re-run full setup
orqa verify               # Governance checks (integrity, version, license)
orqa check                # Code quality (lint, typecheck, format)
orqa test                 # Run test suites
orqa plugin status        # Plugin content sync status
orqa plugin refresh       # Re-sync plugin content
orqa version check        # Check for version drift
orqa graph --stats        # Artifact graph statistics
orqa hosting up           # Start local git server
orqa git status           # Component-level change overview
orqa git sync             # Push to all remotes
```

## Structure

```
app/                      Tauri desktop app (Rust backend + Svelte frontend)
libs/                     Shared libraries (TypeScript + Rust)
plugins/                  First-party plugins (governance, coding standards, etc.)
connectors/               AI tool connectors (Claude Code)
integrations/             SDK integrations (Claude Agent SDK sidecar)
templates/                Plugin scaffold templates
infrastructure/           Docker configs for local git server + sync bridge
tools/                    Development utilities
.orqa/                    Governance artifacts (rules, knowledge, docs, planning)
```

## License

BSL-1.1 with Ethical Use Addendum — see [LICENSE](LICENSE).

All documentation lives within the app at `.orqa/`.
