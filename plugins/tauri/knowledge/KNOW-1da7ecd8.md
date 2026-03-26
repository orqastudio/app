---
id: KNOW-1da7ecd8
type: knowledge
title: Tauri v2 IPC Patterns Reference
summary: "Tauri v2 IPC Patterns Reference. Tauri v2 provides three IPC primitives: 1. **Commands**: Request-response (most common) 2."
status: active
created: 2026-03-20
updated: 2026-03-20
relationships:
  - target: DOC-13c73ecf
    type: synchronised-with
  - target: DOC-9505a5b5
    type: synchronised-with
---

# Tauri v2 IPC Patterns Reference

Three IPC primitives: Commands (request-response), Events (fire-and-forget), Channels (streaming).

## Commands (invoke)

```rust
#[tauri::command]
fn greet(name: String) -> String { format!("Hello, {}!", name) }

#[tauri::command]
async fn fetch_data(url: String) -> Result<String, String> {
    reqwest::get(&url).await.map_err(|e| e.to_string())?.text().await.map_err(|e| e.to_string())
}

// State access
#[tauri::command]
fn increment(state: State<'_, Mutex<AppState>>) -> u32 {
    let mut s = state.lock().unwrap(); s.counter += 1; s.counter
}

// Register ALL commands:
tauri::Builder::default()
    .manage(Mutex::new(AppState { counter: 0 }))
    .invoke_handler(tauri::generate_handler![greet, fetch_data, increment])
```

```typescript
import { invoke } from '@tauri-apps/api/core';
const result = await invoke<string>('greet', { name: 'World' });
```

## Events

```rust
use tauri::Emitter;
app.emit("progress", 50).unwrap();           // To all windows
app.emit_to("main", "notification", msg).unwrap(); // Specific window
```

```typescript
import { listen, once } from '@tauri-apps/api/event';
const unlisten = await listen<number>('progress', (e) => console.log(e.payload));
unlisten(); // cleanup
```

## Channels (Streaming)

```rust
use tauri::ipc::Channel;

#[derive(Clone, serde::Serialize)]
#[serde(tag = "event", content = "data")]
enum DownloadEvent {
    Progress { downloaded: u64, total: u64 },
    Complete { path: String },
}

#[tauri::command]
async fn download(url: String, on_event: Channel<DownloadEvent>) -> Result<String, String> {
    on_event.send(DownloadEvent::Progress { downloaded: 50, total: 100 }).unwrap();
    on_event.send(DownloadEvent::Complete { path: "/done".into() }).unwrap();
    Ok("/done".into())
}
```

```typescript
import { invoke, Channel } from '@tauri-apps/api/core';
const ch = new Channel<DownloadEvent>();
ch.onmessage = (msg) => { /* switch on msg.event */ };
await invoke('download', { url: '...', onEvent: ch });
```

## Selection Guide

| Pattern | Use Case | Direction | Frequency |
|---------|----------|-----------|-----------|
| Commands | Request-response, CRUD | Frontend → Rust | One-time |
| Events | Notifications, broadcasts | Bidirectional | Low-medium |
| Channels | Progress, streaming | Rust → Frontend | High |
