---
id: SKILL-012
title: Orqa IPC Patterns
description: |
  OrqaStudio IPC patterns: Tauri invoke() calls, #[tauri::command] handlers,
  Channel<T> streaming, command registration, and IPC type contracts.
  Use when: Adding or modifying Tauri commands, wiring frontend to backend,
  implementing streaming features, or debugging IPC boundary issues.
status: active
created: 2026-03-01
updated: 2026-03-10
layer: project
category: domain
file-patterns:
  - "backend/src-tauri/src/commands/**"
version: 1.0.0
user-invocable: true
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Tauri invoke() contracts with matching Rust and TypeScript types make the frontend-backend boundary explicit and type-safe
  - target: AD-010
    type: grounded-by
    rationale: Auto-generated inverse of practices relationship from AD-010
  - target: TASK-007
    type: grounded
  - target: TASK-008
    type: grounded
  - target: TASK-020
    type: grounded
  - target: TASK-021
    type: grounded
  - target: TASK-029
    type: grounded
  - target: TASK-074
    type: grounded
  - target: TASK-176
    type: grounded
  - target: TASK-189
    type: grounded
  - target: TASK-190
    type: grounded
  - target: TASK-415
    type: grounded
---
OrqaStudio uses Tauri v2's `invoke()` as the ONLY interface between the Svelte frontend and the Rust backend. There are no HTTP servers, no direct FFI calls, and no side channels.

## The Full Request Chain

Every feature follows this path. All layers must exist in the same commit.

```
Svelte Component (event handler)
    → Svelte Store (invoke() call)
    → Tauri IPC Boundary (serialization)
    → #[tauri::command] handler (thin wrapper)
    → Domain Service (business logic)
    → Repository (SQLite) or Sidecar (Claude API)
    → Response flows back up
```

## Patterns

### 1. Tauri Command Handler

Commands live in `backend/src-tauri/src/commands/` and are thin wrappers delegating to domain services.

```rust
// backend/src-tauri/src/commands/project_commands.rs
#[tauri::command]
pub fn project_open(path: String, state: State<'_, AppState>) -> Result<Project, OrqaError> {
    let canonical = validate_directory_path(&path)?;
    let conn = state.db.lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let project = match project_repo::get_by_path(&conn, &canonical) {
        Ok(project) => project,
        Err(OrqaError::NotFound(_)) => {
            let name = derive_project_name(&canonical);
            project_repo::create(&conn, &name, &canonical, None)?
        }
        Err(e) => return Err(e),
    };

    Ok(project)
}
```

**Key rules:**
- Always return `Result<T, OrqaError>` — never `unwrap()` or `panic!()`
- Access database via `State<'_, AppState>` and lock the mutex
- Delegate to domain modules in `backend/src-tauri/src/domain/` and repos in `backend/src-tauri/src/repo/`
- Keep commands thin — business logic belongs in domain services

### 2. Command Registration

Every command must be registered in `backend/src-tauri/src/lib.rs`:

```rust
// backend/src-tauri/src/lib.rs
.invoke_handler(tauri::generate_handler![
    // Project
    commands::project_commands::project_open,
    commands::project_commands::project_list,
    // Sessions
    commands::session_commands::session_create,
    commands::session_commands::session_list,
    // Streaming
    commands::stream_commands::stream_send_message,
    commands::stream_commands::stream_stop,
    // ... all commands must be listed here
])
```

**If a command exists but isn't registered here, `invoke()` will fail at runtime with "command not found".**

### 3. Frontend invoke() Call

The frontend uses a typed wrapper around `@tauri-apps/api/core`:

```typescript
// ui/src/lib/ipc/invoke.ts — typed wrapper
import { invoke } from '@tauri-apps/api/core';

// ui/src/lib/stores/conversation.svelte.ts — usage in store
async loadMessages(sessionId: number): Promise<void> {
    this.isLoading = true;
    this.error = null;
    try {
        this.messages = await invoke<Message[]>("message_list", { sessionId });
    } catch (err) {
        this.error = err instanceof Error ? err.message : String(err);
    } finally {
        this.isLoading = false;
    }
}
```

**Key rules:**
- Always specify the return type generic: `invoke<ReturnType>()`
- Always handle errors — show them to the user
- Only call `invoke()` in stores or page-level code, NEVER in display components

### 4. Channel<T> Streaming

For long-running operations (Claude conversations), use `Channel<T>`:

```typescript
// Frontend: create channel, pass to invoke
const channel = createStreamChannel((event: StreamEvent) => {
    this.handleStreamEvent(event);
});

await invoke("stream_send_message", {
    sessionId,
    content,
    model: this.selectedModel,
    onEvent: channel,
});
```

```rust
// Backend: accept Channel<T> parameter, send events
#[tauri::command]
pub async fn stream_send_message(
    session_id: i64,
    content: String,
    model: Option<String>,
    on_event: Channel<StreamEvent>,
    state: State<'_, AppState>,
) -> Result<(), OrqaError> {
    on_event.send(StreamEvent::StreamStart {
        message_id,
        resolved_model: Some(model_name),
    })?;
    // ... process events from sidecar, forward via on_event.send()
}
```

### 5. IPC Type Contracts

Types must match exactly across the Rust/TypeScript boundary.

```rust
// Rust: backend/src-tauri/src/domain/provider_event.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum StreamEvent {
    StreamStart { message_id: i64, resolved_model: Option<String> },
    TextDelta { content: String },
    ToolUseStart { tool_call_id: String, tool_name: String },
    TurnComplete { input_tokens: i64, output_tokens: i64 },
    StreamError { code: String, message: String, recoverable: bool },
    // ...
}
```

```typescript
// TypeScript: ui/src/lib/types/streaming.ts
export type StreamEvent =
    | { type: "stream_start"; data: { message_id: number; resolved_model: string | null } }
    | { type: "text_delta"; data: { content: string } }
    | { type: "tool_use_start"; data: { tool_call_id: string; tool_name: string } }
    | { type: "turn_complete"; data: { input_tokens: number; output_tokens: number } }
    | { type: "stream_error"; data: { code: string; message: string; recoverable: boolean } }
    // ...
```

**`serde(rename_all = "snake_case")` converts Rust PascalCase variants to snake_case in JSON.** The TypeScript union must use the snake_case names.

## Full Four-Layer Feature Example

When adding a new feature, all four layers must be created together. Below is a complete example (hardware info display) showing the canonical shape for each layer.

**1. Rust types (`backend/src-tauri/src/domain/`):**

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu: String,
    pub memory_gb: u64,
    pub gpu: Option<String>,
}
```

**2. Rust command (`backend/src-tauri/src/commands/`):**

```rust
#[tauri::command]
pub async fn get_hardware_info() -> Result<HardwareInfo, String> {
    // Real implementation — no stubs
    Ok(HardwareInfo { /* ... */ })
}
```

**3. TypeScript types (`ui/src/lib/types/`):**

```typescript
export interface HardwareInfo {
  cpu: string;
  memory_gb: number;
  gpu: string | null;
}
```

**4. Svelte component (minimal, no shared state):**

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

**5. Store binding (when state is shared across components):**

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

## Anti-Patterns

### Missing command registration
```rust
// Command exists but NOT in invoke_handler → runtime "command not found" error
#[tauri::command]
pub fn my_new_command() -> Result<(), OrqaError> { /* ... */ }
// FORGOT to add to backend/src-tauri/src/lib.rs!
```

### invoke() in display components
```svelte
<!-- FORBIDDEN: display components must not call invoke() -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  let data = $state(null);
  $effect(() => { invoke('get_data').then(d => data = d); }); // WRONG
</script>
```
Move the `invoke()` call to a store or page-level container.

### Type mismatch across boundary
```rust
// Rust returns i64
pub struct Info { pub count: i64 }
```
```typescript
// TypeScript expects string — SILENT RUNTIME ERROR
interface Info { count: string }
```

### Forgetting error handling
```typescript
// WRONG: no error handling
const data = await invoke('get_data');

// CORRECT: always handle errors
try {
    const data = await invoke<Data>('get_data');
} catch (err) {
    this.error = String(err);
}
```

## Verification

Before committing any IPC change, use `search_regex` (or `mcp__chunkhound__search_regex` in CLI) to verify:

1. The command name appears in `backend/src-tauri/src/lib.rs` (registered)
2. The command name appears in a `#[tauri::command]` function (implemented)
3. The command name appears in an `invoke()` call (wired to frontend)
4. The return type has matching TypeScript interface

## Key Files

| File | Purpose |
|------|---------|
| `backend/src-tauri/src/lib.rs` | Command registration (invoke_handler) |
| `backend/src-tauri/src/commands/` | All `#[tauri::command]` handlers |
| `backend/src-tauri/src/domain/` | Business logic (commands delegate here) |
| `backend/src-tauri/src/domain/provider_event.rs` | StreamEvent enum |
| `ui/src/lib/ipc/invoke.ts` | Typed invoke wrapper |
| `ui/src/lib/stores/*.svelte.ts` | Stores that call invoke() |
| `ui/src/lib/types/` | TypeScript interfaces matching Rust types |
| `ui/src/lib/types/streaming.ts` | StreamEvent TypeScript union |
