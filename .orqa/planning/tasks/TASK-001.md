---
id: TASK-001
title: Emit SystemPromptSent event from stream_commands.rs
status: done
epic: EPIC-001
description: In stream_send_message(), emit a SystemPromptSent event via on_event channel after resolve_system_prompt() and before sidecar.send().
created: 2026-03-07
updated: 2026-03-07
assignee: backend-engineer
scope: [src-tauri/src/commands/stream_commands.rs]
acceptance: [SystemPromptSent event is emitted via on_event channel before sidecar request is sent, Event carries governance_prompt (full text) and total_chars (correct length), custom_prompt is None (EPIC-002 adds this), No event emitted when system prompt is None (no project loaded), Emission failure does not prevent message from being sent, No unwrap() or expect() in the new code]
---

## What

In `stream_send_message()` in `src-tauri/src/commands/stream_commands.rs`, after `resolve_system_prompt()` returns `Some(prompt)` and before `state.sidecar.send(&request)`, emit a `SystemPromptSent` event.

## Where

Line ~827-838 in `stream_commands.rs`, between:
```rust
let system_prompt = resolve_system_prompt(&state);
// <-- EMIT HERE
let request = SidecarRequest::SendMessage { ... };
state.sidecar.send(&request)?;
```

## How

```rust
if let Some(ref prompt) = system_prompt {
    let _ = on_event.send(StreamEvent::SystemPromptSent {
        custom_prompt: None,
        governance_prompt: prompt.clone(),
        total_chars: prompt.len() as i64,
    });
}
```

The `let _ =` ensures a closed channel doesn't cause an error — the event is informational, not critical.

## Verification

1. `cargo clippy -- -D warnings` passes
2. `cargo test` passes
3. Send a message with a project loaded → `SystemPromptSent` event arrives in frontend
4. Send a message without a project → no event, no error
