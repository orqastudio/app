---
id: KNOW-2a72f929
type: knowledge
name: Rust Plugin Installation
status: active
plugin: "@orqastudio/plugin-rust"
relationships:
  - target: DOC-27becb92
    type: synchronised-with
---

# Rust Plugin Installation

## Prerequisites

- Rust toolchain (stable) installed via rustup
- Cargo available on PATH

## What This Plugin Provides

- **clippy** tool — lint enforcement via `cargo clippy`
- **rustfmt** tool — formatting enforcement via `cargo fmt --check`
- **cargo-test** tool — test execution via `cargo test`
- **Clippy Config Management** skill (KNOW-5efa83a5)
- **Rust Testing Patterns** skill (KNOW-214b7cdc)
- **Rust Standards Agent** (AGENT-4241392c) — assess and configure modes

## Installation

```bash
orqa plugin install @orqastudio/plugin-rust
```

## Post-Install

The rust standards agent runs in `assess` mode to evaluate the project's current Rust configuration and suggest rules. Then in `configure` mode to generate `clippy.toml` and `.rustfmt.toml` from those rules.

## Extension

The Tauri plugin (`@orqastudio/plugin-tauri`) extends this plugin with Tauri-specific configuration — `tauri.conf.json`, Tauri v2 patterns, and Tauri-aware build settings.
