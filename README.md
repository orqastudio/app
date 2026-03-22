![License](https://img.shields.io/badge/license-BSL%201.1-blue)
![Status](https://img.shields.io/badge/status-pre--release-orange)
![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte-FF3E00?logo=svelte&logoColor=white)
![Shell](https://img.shields.io/badge/Shell-4EAA25?logo=gnubash&logoColor=white)

![OrqaStudio](https://github.com/orqastudio/orqastudio-brand/blob/main/assets/banners/banner-1680x240.png?raw=1)

# OrqaStudio Dev Environment

> **Pre-release** — APIs and interfaces may change without notice until v1.0.0.

## Prerequisites

- **Git** — [git-scm.com](https://git-scm.com/)

Node.js 22+ and Rust are checked and installed automatically during setup.

## Setup

```bash
git clone git@github.com:orqastudio/orqastudio-dev.git
cd orqastudio-dev
make install
```

This runs the full bootstrap:

1. **Prereqs** — checks git, node, npm, rust, cargo. Offers to install Node (via fnm) and Rust (via rustup) if missing.
2. **Submodules** — inits all git submodules recursively.
3. **Dependencies** — `npm install` for all packages, `cargo fetch` for Rust.
4. **Build & link** — builds all libraries, `npm link` into the app, svelte-kit sync, UI build.
5. **Smoke test** — verifies the CLI responds, artifact graph builds, `cargo check` compiles, and `svelte-check` passes.

After install, the `orqa` CLI is on your PATH.

## Daily Use

```bash
orqa install              # Re-run full setup (or individual: prereqs, submodules, deps, link)
orqa verify               # Governance checks (integrity, version, license, readme)
orqa check                # Code quality (lint, typecheck, format) — or: orqa check rust, orqa check app
orqa test                 # Run test suites — or: orqa test rust, orqa test app
orqa enforce             # Integrity validation only
orqa version check        # Check for version drift
orqa version bump 0.2.0   # Bump version across all repos
orqa plugin install <src> # Install a plugin
orqa graph --stats        # Artifact graph statistics
```

## License

BSL-1.1 with Ethical Use Addendum — see [LICENSE](app/LICENSE) and [CHANGE-LICENSE](app/CHANGE-LICENSE).

All documentation lives within the app at `.orqa/`.
