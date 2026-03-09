---
id: RULE-010
slug: end-to-end-completeness
layer: canon
status: active
title: "End-to-End Completeness"
description: "Every feature must be implemented across all layers (Rust command, IPC type, Svelte component, store) in the same commit."
scope: system
---


Every feature MUST be implemented across ALL layers in the same commit. Partial implementations that work in isolation but fail at runtime due to missing layers are forbidden.

## Documentation Before Implementation

**When an audit reveals gaps between docs and code:** Update the docs FIRST to define the correct target state. Code is then changed to match the docs. Never fix code without first verifying the docs describe the intended behavior.

## The Full Stack Rule

OrqaStudio is a Tauri desktop app. There is no API gateway — the Tauri IPC bridge IS the boundary between backend and frontend. When adding or modifying ANY feature, ALL of the following layers MUST be updated together:

1. **Rust command** — the `#[tauri::command]` function in `src-tauri/src/` that implements the backend logic
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

## Required Pattern

When adding a new feature (e.g., hardware info display):

**1. Rust types (`src-tauri/src/types/`):**

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu: String,
    pub memory_gb: u64,
    pub gpu: Option<String>,
}
```

**2. Rust command (`src-tauri/src/commands/`):**

```rust
#[tauri::command]
pub async fn get_hardware_info() -> Result<HardwareInfo, String> {
    // Real implementation — no stubs
    Ok(HardwareInfo { /* ... */ })
}
```

**3. TypeScript types (`ui/lib/types/`):**

```typescript
export interface HardwareInfo {
  cpu: string;
  memory_gb: number;
  gpu: string | null;
}
```

**4. Svelte component:**

```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { HardwareInfo } from '$lib/types';

  let info = $state<HardwareInfo | null>(null);
  let error = $state<string | null>(null);

  $effect(() => {
    invoke<HardwareInfo>('get_hardware_info')
      .then(r => info = r)
      .catch(e => error = e.toString());
  });
</script>
```

**5. Store binding (if shared state):**

```typescript
// $lib/stores/hardware.svelte.ts
import { invoke } from '@tauri-apps/api/core';
import type { HardwareInfo } from '$lib/types';

let info = $state<HardwareInfo | null>(null);
let loading = $state(false);
let error = $state<string | null>(null);

export function useHardware() {
  // ...fetch, expose reactive state
}
```

## Declaring a Task Complete

A task that adds or modifies features is NOT complete until:

1. The full request chain has been verified (not just type-checked)
2. All four layers exist and are consistent
3. No legacy fallback paths remain
4. Tests cover the new feature at the integration level

## ChunkHound Integration

Use `code_research` to map the full request chain (component -> store -> invoke -> Rust command) in one query. Use `search_regex` with the command name (e.g. `"get_hardware_info"`) to verify each layer exists before committing.

## Related Rules

- RULE-020 (no-stubs) — commands must return real data, not fake responses
- RULE-012 (error-ownership) — verify the full chain works, don't assume
- RULE-005 (chunkhound-usage) — tools for verifying the chain exists
