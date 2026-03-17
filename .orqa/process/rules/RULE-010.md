---
id: RULE-010
title: End-to-End Completeness
description: "Every feature must be implemented across all layers (Rust command, IPC type, Svelte component, store) in the same commit."
status: active
created: 2026-03-07
updated: 2026-03-13
layer: project
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: End-to-end completeness ensures every feature is fully structured across all layers
  - target: RULE-020
    type: informs
    rationale: All four layers must return real data — commands must not be stubs
  - target: RULE-012
    type: informs
    rationale: The full chain must be verified before marking a feature complete
  - target: AD-017
    type: enforces
    rationale: Auto-generated inverse of enforces relationship from AD-017
  - target: AD-009
    type: enforces
    rationale: Auto-generated inverse of enforces relationship from AD-009
  - target: IMPL-017
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from IMPL-017
  - target: AD-001
    type: enforces
  - target: AD-002
    type: enforces
  - target: RULE-002
    type: informed-by
  - target: RULE-003
    type: informed-by
  - target: RULE-005
    type: informed-by
  - target: RULE-012
    type: informed-by
  - target: RULE-018
    type: informed-by
  - target: RULE-020
    type: informed-by
  - target: RULE-022
    type: informed-by
  - target: RULE-028
    type: informed-by
  - target: RULE-041
    type: informed-by
  - target: PILLAR-001
    type: informs
---
Every feature MUST be implemented across ALL layers in the same commit. Partial implementations that work in isolation but fail at runtime due to missing layers are forbidden.

## Documentation Before Implementation

**When an audit reveals gaps between docs and code:** Update the docs FIRST to define the correct target state. Code is then changed to match the docs. Never fix code without first verifying the docs describe the intended behavior.

## The Full Stack Rule

OrqaStudio is a Tauri desktop app. There is no API gateway — the Tauri IPC bridge IS the boundary between backend and frontend. When adding or modifying ANY feature, ALL of the following layers MUST be updated together:

1. **Rust command** — the `#[tauri::command]` function in `backend/src-tauri/src/` that implements the backend logic
2. **IPC type** — Rust structs with `Serialize`/`Deserialize` for the command's input/output, AND matching TypeScript interfaces in the frontend
3. **Svelte component** — the Svelte 5 component that calls the Tauri command via `invoke()`
4. **Store binding** — the Svelte store (runes-based) that manages the reactive state for this feature

All four layers must be committed together. A feature is not done until the full chain works end-to-end.

## The IPC Boundary Rule

**The Tauri `invoke()` bridge is the ONLY interface between frontend and backend.** The frontend NEVER accesses Rust functions directly or via any other mechanism.

Every Rust backend capability that the frontend needs MUST have a corresponding `#[tauri::command]` function registered in the Tauri app builder. If you add backend logic and a Svelte component to call it, you MUST also add the Tauri command and register it. No exceptions.

## Verification Checklist (MANDATORY before every commit touching a feature)

Run through this checklist before committing any change that adds or modifies a feature:

- [ ] Does the Rust command exist and is it registered in the Tauri app builder?
- [ ] Are the input/output types defined as Rust structs with Serialize/Deserialize?
- [ ] Do matching TypeScript interfaces exist for all IPC types?
- [ ] Does the Svelte component call `invoke()` with the correct command name and arguments?
- [ ] Does the store correctly manage the state lifecycle (loading, loaded, error)?
- [ ] Are the types consistent across Rust structs and TypeScript interfaces?
- [ ] Have you verified with an actual invocation (not just type-checking) that the command is reachable?

## FORBIDDEN Patterns

**Adding a Rust function without the Tauri command registration:**

```rust
// Function exists in a module but is NOT registered as a #[tauri::command]
// Result: invoke() fails at runtime with "command not found"
```

**Adding a frontend invoke() call without verifying the command exists:**

```typescript
// Svelte component calls invoke('get_hardware_info')
// But no #[tauri::command] fn get_hardware_info exists
// Result: runtime error
```

**Legacy fallback code paths:**

```typescript
// FORBIDDEN: keeping old command as fallback when new one fails
async function getHardwareInfo() {
  try {
    return await invoke('get_hardware_info_v2');
  } catch {
    return await invoke('get_hardware_info_legacy'); // FORBIDDEN
  }
}
```

When changes are made, they are made fully and completely. Old code paths MUST be removed, not kept as fallbacks.

See the `orqa-ipc-patterns` skill for full code examples and patterns for all four layers.

## Declaring a Task Complete

A task that adds or modifies features is NOT complete until:

1. The full request chain has been verified (not just type-checked)
2. All four layers exist and are consistent
3. No legacy fallback paths remain
4. Tests cover the new feature at the integration level

## ChunkHound Integration

Use `code_research` to map the full request chain (component -> store -> invoke -> Rust command) in one query. Use `search_regex` with the command name (e.g. `"get_hardware_info"`) to verify each layer exists before committing.

## Related Rules

- [RULE-020](RULE-020) (no-stubs) — commands must return real data, not fake responses
- [RULE-012](RULE-012) (error-ownership) — verify the full chain works, don't assume
- [RULE-005](RULE-005) (code-search-usage) — tools for verifying the chain exists
