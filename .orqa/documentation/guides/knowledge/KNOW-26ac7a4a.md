---
id: KNOW-26ac7a4a
type: knowledge
title: "Agent-optimized: Tauri Development Guide"
description: "Condensed Tauri v2 development patterns for OrqaStudio — IPC, error handling, testing, and coding standards."
status: active
tier: on-demand
relationships:
  - type: synchronised-with
    target: DOC-13c73ecf
---

# Tauri Development — Agent Reference

## Tools Provided

- **Clippy** — Rust linting, config generated from coding standards rules
- **rustfmt** — formatting, config generated from rules
- **cargo test** — test runner integration
- Config files (`clippy.toml`, `.rustfmt.toml`) are generated, not hand-edited

## Tauri v2 Patterns

- IPC via `#[tauri::command]` — the ONLY frontend-backend interface
- `thiserror` for error types — no `unwrap()` or `expect()` in production code
- Domain-driven structure: `domain/` for logic, `commands/` for IPC, `repo/` for persistence
- Functions under 50 lines, pure domain logic, side effects at boundaries

## Testing

- Unit tests in `#[cfg(test)]` modules
- `make_` prefix for test fixture helpers
- `assert!`/`assert_eq!` — no custom test frameworks

## Standards Management

- Coding standards are OrqaStudio rules with enforcement entries
- Plugin generates tool config files from rules
- To change a standard: edit the rule, not the config

## Sub-Project Overrides

- In org mode, Rust standards propagate from org level
- Override specific clippy lints or rustfmt settings with tracked rationale
