---
scope: project
enforcement:
  - event: file
    action: block
    conditions:
      - field: file_path
        pattern: ui/lib/components/.*\.svelte$
      - field: new_text
        pattern: invoke\s*\(|from\s+['"]@tauri-apps/api
---

# Architecture Decisions

**Source of Truth:** `@docs/architecture/decisions.md`

**READ the full document to understand all architecture decisions.**

## Critical Decisions (violations = immediate rejection)

| Decision | Rule |
|----------|------|
| Error propagation | All Rust functions return `Result`. No `unwrap()` / `expect()` / `panic!()` in production. `thiserror` for typed errors. |
| IPC boundary | Tauri `invoke()` is the ONLY frontend-backend interface. No side channels, no direct FFI. |
| Component purity | Display components receive props only. Pages/containers fetch data. No `invoke()` in `$lib/components/`. |
| Type safety | Strict TypeScript (no `any`). Rust IPC types derive `Serialize`/`Deserialize`. Types match across the boundary. |
| Immutability | Rust domain types immutable by default. Svelte stores use runes (`$state`, `$derived`). |
| UX-first design | User journeys drive backend requirements, not the reverse. |
| Svelte 5 only | Runes only. No Svelte 4 patterns (`$:`, `export let`, `let:`). |
| SQLite for structured data | All structured persistence goes through SQLite. No localStorage for application state. |

## Before Writing Code

1. Read `@docs/architecture/decisions.md`
2. Check if your change affects any existing decision
3. If proposing a new decision, follow the decision template format

## Before Writing Plans

1. Read `.claude/rules/plan-mode-compliance.md`
2. Start with user journeys and UI design (UX-first)
3. Include architectural compliance section verifying all relevant decisions
