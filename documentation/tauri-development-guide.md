---
id: DOC-TAU-d9c0d1c7
title: "Tauri Development Guide"
description: "How to develop with Rust and Tauri v2 in OrqaStudio projects — IPC patterns, error handling, testing, and coding standards."
category: how-to
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: SKILL-TAU-de97219c
    type: synchronised-with
  - target: SKILL-TAU-6f7f4da5
    type: synchronised-with
  - target: SKILL-a5434b61
    type: synchronised-with
  - target: SKILL-c323ec5f
    type: synchronised-with

---

# Tauri Development Guide

This plugin provides Rust and Tauri development infrastructure for OrqaStudio projects.

## What It Provides

- **Clippy** — Rust linting, configured from coding standards rules
- **rustfmt** — Code formatting, configured from rules
- **cargo test** — Test runner integration
- **Config generation** — `clippy.toml` and `.rustfmt.toml` generated from rules

## Coding Standards

Standards are OrqaStudio rules with enforcement entries. The plugin generates tool config files. To change a standard: edit the rule.

## Tauri v2 Patterns

- IPC via `#[tauri::command]` — the only frontend-backend interface
- `thiserror` for error types — no `unwrap()` or `expect()` in production
- Domain-driven structure: `domain/` for logic, `commands/` for IPC, `repo/` for persistence
- Functions under 50 lines, pure domain logic, side effects at boundaries

## Testing

- Unit tests in `#[cfg(test)]` modules
- `make_` prefix for test fixture helpers
- `assert!`/`assert_eq!` — no custom frameworks

## Sub-Project Overrides

In organisation mode, Rust coding standards propagate from the org level. Override specific clippy lints or rustfmt settings with tracked rationale.
