---
id: KNOW-6f7f4da5
name: Tauri Plugin Installation
status: active
plugin: "@orqastudio/plugin-tauri"
relationships:
  - target: DOC-d0237b23
    type: synchronised-with
  - target: DOC-d9c0d1c7
    type: synchronised-with

---

# Tauri Plugin Installation

This skill is consumed by the core installer agent when setting up the Tauri plugin.

## Detection

Identify sub-projects that should receive this plugin by checking for:
- `Cargo.toml` → Rust project
- `src-tauri/` directory → Tauri app
- `.rs` files → Rust source code

## Dependencies

Ensure the Rust stable toolchain is installed via rustup. No npm dependencies — Rust tools ship with the toolchain.

## Initial Config Generation

Generate `clippy.toml` and `.rustfmt.toml` from the project's coding standards rules. If no rules exist, create a default coding standards rule with sensible defaults (deny unwrap, deny expect, pedantic warnings, 100 char width).

## Organisation Mode

When installing to an org project:
1. Scan all sub-projects for Cargo.toml / .rs files
2. Recommend sub-projects based on detection
3. Present selection UI
4. Generate config in each selected sub-project from org-level rules
