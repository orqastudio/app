---
id: DOC-RST-27becb92
type: doc
name: Rust Development Guide
status: active
category: how-to
plugin: "@orqastudio/plugin-rust"
relationships:
  - target: KNOW-RST-5efa83a5
    type: synchronised-with
  - target: KNOW-RST-214b7cdc
    type: synchronised-with
  - target: KNOW-12495e28
    type: synchronised-with
  - target: KNOW-RST-2a72f929
    type: synchronised-with
---

# Rust Development Guide

## Overview

The Rust plugin provides base development infrastructure for any Rust project managed by OrqaStudio. It handles clippy linting, rustfmt formatting, and cargo test execution — all driven by OrqaStudio coding standards rules.

## Tools Provided

| Tool | Command | Config File | Purpose |
|------|---------|-------------|---------|
| clippy | `cargo clippy` | `clippy.toml` | Lint enforcement |
| rustfmt | `cargo fmt --check` | `.rustfmt.toml` | Format enforcement |
| cargo-test | `cargo test` | CLI args | Test execution |

## How It Works

1. **Define rules** — coding standards rules with enforcement entries targeting `@orqastudio/plugin-rust`
2. **Configure** — `orqa check configure` reads rules and generates `clippy.toml` / `.rustfmt.toml`
3. **Check** — `orqa check` runs clippy, rustfmt, and cargo test against the config
4. **Assess** — the Rust Standards Agent (AGENT-RST-4241392c) provides structured violation reports

## Extension

The Tauri plugin (`@orqastudio/plugin-tauri`) extends this plugin with:
- Tauri v2 patterns and API usage guidance
- Tauri-specific cargo features and build configuration
- `tauri.conf.json` management

Install the Tauri plugin for Tauri v2 desktop applications.

## Prerequisites

- Rust stable toolchain via rustup
- Cargo on PATH
