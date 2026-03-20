---
id: DOC-TAU-d0237b23
title: "Tauri Plugin Setup"
description: "How to install and configure the Tauri development plugin — toolchain requirements, config generation, and organisation mode."
category: onboarding
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: SKILL-2539a6e4
    type: synchronised-with
---

# Tauri Plugin Setup

## Installation

```bash
orqa plugin install @orqastudio/plugin-tauri
```

The installer:
1. Detects which sub-projects have Rust/Tauri code
2. Recommends which sub-projects should receive the plugin
3. Verifies the Rust stable toolchain is installed
4. Generates `clippy.toml` and `.rustfmt.toml` from coding standards rules

## What Gets Configured

- `clippy.toml` — lint levels from enforcement entries
- `.rustfmt.toml` — formatting rules from enforcement entries
- Cargo test integration via `orqa check` and `orqa test`

No npm dependencies — Rust tools ship with the toolchain.

## Organisation Mode

When installed at the org level, the plugin detects Rust sub-projects by scanning for `Cargo.toml` files. AI recommends which sub-projects apply. Each gets config generated from org-level rules.
