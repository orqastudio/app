---
id: SKILL-RST-2a72f929
type: skill
name: Rust Plugin Installation
status: active
plugin: "@orqastudio/plugin-rust"
relationships:
  - target: DOC-RST-27becb92
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
- **Clippy Config Management** skill (SKILL-RST-5efa83a5)
- **Rust Testing Patterns** skill (SKILL-RST-214b7cdc)
- **Rust Standards Agent** (AGENT-RST-4241392c) — assess and configure modes

## Installation

```bash
orqa plugin install @orqastudio/plugin-rust
```

## Post-Install

The rust standards agent runs in `assess` mode to evaluate the project's current Rust configuration and suggest rules. Then in `configure` mode to generate `clippy.toml` and `.rustfmt.toml` from those rules.

## Extension

The Tauri plugin (`@orqastudio/plugin-tauri`) extends this plugin with Tauri-specific configuration — `tauri.conf.json`, Tauri v2 patterns, and Tauri-aware build settings.
