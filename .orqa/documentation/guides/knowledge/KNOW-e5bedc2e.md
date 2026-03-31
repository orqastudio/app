---
id: KNOW-e5bedc2e
type: knowledge
title: "Agent-optimized: Rust Development Guide"
description: "Condensed Rust plugin infrastructure — clippy, rustfmt, cargo test, config generation from rules."
status: active
tier: on-demand
relationships:
  - type: synchronised-with
    target: DOC-2372ed36
---

# Rust Development — Agent Reference

## Plugin: @orqastudio/plugin-rust

Base Rust development infrastructure for any Rust project managed by OrqaStudio.

## Tools

| Tool | Command | Config File | Purpose |
| --- | --- | --- | --- |
| clippy | `cargo clippy` | `clippy.toml` | Lint enforcement |
| rustfmt | `cargo fmt --check` | `.rustfmt.toml` | Format enforcement |
| cargo-test | `cargo test` | CLI args | Test execution |

## Workflow

1. **Define rules** — coding standards rules with enforcement entries targeting `@orqastudio/plugin-rust`
2. **Configure** — `orqa check configure` reads rules, generates `clippy.toml` / `.rustfmt.toml`
3. **Check** — `orqa check` runs clippy, rustfmt, and cargo test
4. **Assess** — Rust Standards Agent (AGENT-26e5029d) provides violation reports

## Extension

The Tauri plugin (`@orqastudio/plugin-tauri`) extends this with:
- Tauri v2 patterns and API guidance
- Tauri-specific cargo features and build config
- `tauri.conf.json` management

## Prerequisites

- Rust stable toolchain via rustup
- Cargo on PATH
