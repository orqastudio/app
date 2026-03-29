---
id: RULE-b03009da
type: rule
title: End-to-End Completeness
description: "Every feature must include all required layers in the same commit. No partial implementations across boundaries."
status: active
enforcement_type: advisory
created: 2026-03-07
updated: 2026-03-24
enforcement:

  - engine: behavioral

    message: "Every feature must include all required layers in the same commit — no partial implementations across boundaries"

  - engine: behavioral

    message: "Code reviewer verifies all layers exist for each new feature endpoint"
summary: "Every feature crossing the IPC boundary must include all four layers in the same commit: backend command, IPC type (Rust + TypeScript), frontend store, UI component. Types must match across boundaries. Error handling end-to-end. General principle: no partial implementations across any service boundary."
tier: stage-triggered
roles: [implementer, reviewer]
priority: P1
tags: [end-to-end, four-layer, ipc, completeness]
relationships:

  - target: PD-4e7faf0e

    type: enforces
    rationale: "IPC boundary design requires matching types on both sides"

  - target: RULE-05ae2ce7

    type: complements
    rationale: "Architecture decisions define the layer requirements this rule enforces"

  - target: RULE-af5771e3

    type: complements
    rationale: "No-stubs ensures each layer has a real implementation, not scaffolding"

  - target: RULE-c382e053

    type: complements
    rationale: "No-aliases ensures layers agree on types without shims"
---

Every feature that crosses a service boundary MUST include all required layers in the same commit. A feature that exists in only some layers is incomplete — it creates dead code, broken contracts, and false confidence.

## The Four-Layer Rule

For any feature that touches the IPC boundary between frontend and backend, ALL four layers must be present in the same commit:

| Layer | What | Where |
| --- | --- | --- |
| **Backend command** | Tauri command or service function | `backend/src-tauri/src/commands/` |
| **IPC type** | Shared type definitions (Rust + TypeScript) | Rust: domain types with `Serialize`/`Deserialize`. TS: matching interface in `$lib/types/` |
| **Frontend store** | Reactive state management | `ui/src/lib/stores/` |
| **UI component** | User-facing display | `ui/src/lib/components/` or `ui/src/routes/` |

If a feature adds a backend command, the matching TypeScript types, store bindings, and UI component must ship in the same commit. If a feature adds a UI component that calls the backend, the backend command must already exist or be added in the same commit.

## When This Applies

- Adding a new Tauri command that will be called from the frontend
- Adding a new UI feature that requires backend data
- Modifying an IPC type (both sides must be updated together)
- Adding a new store that fetches from the backend

## When This Does NOT Apply

- Pure backend changes with no frontend impact (e.g., internal refactoring)
- Pure frontend changes with no backend calls (e.g., layout adjustments)
- Documentation-only changes
- Governance artifact changes

## Verification Checklist

Before committing a feature that crosses the IPC boundary:

1. **Backend command exists** — the Tauri command is defined and registered
2. **Types match** — Rust types derive `Serialize`/`Deserialize`, TypeScript interfaces match field names and types
3. **Store calls the command** — the store uses `invoke()` to call the backend
4. **Component renders the data** — the UI component displays the data from the store
5. **Error handling is end-to-end** — errors propagate from backend through IPC to the UI with user-visible messages

## Generalised Principle

The four-layer rule is the OrqaStudio-specific instance of a general principle: **all layers that participate in a feature must be updated together**. In other project types, the layers differ:

- **REST API project**: endpoint + handler + client + UI
- **CLI tool**: command + logic + output formatter
- **Library**: public API + implementation + documentation + tests

The principle is the same: no partial implementations across boundaries.

## FORBIDDEN

- Adding a Tauri command without a matching TypeScript interface
- Adding a TypeScript interface without a matching Tauri command
- Adding a store that calls `invoke()` for a command that doesn't exist
- Adding a UI component that reads from a store that has no backend wiring
- Modifying a Rust IPC type without updating the TypeScript counterpart (or vice versa)
- Claiming a feature is "done" when only some layers exist

## Related Rules

- [RULE-05ae2ce7](RULE-05ae2ce7) (architecture-decisions) — decisions define the layer requirements (IPC boundary, component purity, type safety) that this rule enforces
- [RULE-af5771e3](RULE-af5771e3) (no-stubs) — each layer must have a real implementation, not scaffolding
- [RULE-c382e053](RULE-c382e053) (no-aliases) — type consistency must hold across all layers in the same commit
- [RULE-0be7765e](RULE-0be7765e) (error-ownership) — the full chain must be verified
- [RULE-0d29fc91](RULE-0d29fc91) (code-search-usage) — use `search_research` to map the full request chain
- [RULE-dccf4226](RULE-dccf4226) (plan-mode-compliance) — the full-stack requirement per feature
- [RULE-43f1bebc](RULE-43f1bebc) (systems-thinking) — full-stack thinking is systems thinking applied to the four-layer feature structure
- [RULE-09a238ab](RULE-09a238ab) (data-persistence) — all layers must agree on persistence strategy
- [RULE-d543d759](RULE-d543d759) (honest-status-reporting) — completion requires all layers working, not just one layer
- [RULE-63cc16ad](RULE-63cc16ad) (artifact-config-integrity) — config changes must be reflected across all layers
- [RULE-f3dca71e](RULE-f3dca71e) (version-numbering) — version consistency across all layers is a layer-consistency requirement
- [RULE-9814ec3c](RULE-9814ec3c) (coding-standards) — standards that apply within each layer
