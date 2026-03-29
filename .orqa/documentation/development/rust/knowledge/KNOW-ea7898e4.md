---
id: KNOW-ea7898e4
type: knowledge
title: Rust Async Patterns
domain: platform/rust
description: "Master Rust async programming with Tokio, async traits, error handling, and concurrent patterns. Use when building async Rust applications, implementing concurrent systems, or debugging async code."
tier: on-demand
summary: "Master Rust async programming with Tokio, async traits, error handling, and concurrent patterns. Use when building async Rust applications, implementing concurrent systems, or debugging async code."
status: active
created: 2026-03-01
updated: 2026-03-10
category: domain
user-invocable: false
relationships:
  - target: DOC-2372ed36
    type: synchronised-with
---

Async Rust patterns with Tokio: concurrent tasks, channels, error handling, graceful shutdown, and streams.

## Execution Model

```text
Future (lazy) → poll() → Ready(value) | Pending
                ↑           ↓
              Waker ← Runtime schedules
```

| Concept | Purpose |
| --------- | --------- |
| `Future` | Lazy computation that may complete later |
| `async fn` | Function returning impl Future |
| `Task` | Spawned future running concurrently |
| `select!` | Race multiple futures |

## Concurrent Tasks

```rust
use tokio::task::JoinSet;

async fn fetch_all(urls: Vec<String>) -> Vec<String> {
    let mut set = JoinSet::new();
    for url in urls {
        set.spawn(async move { fetch_data(&url).await });
    }
    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        if let Ok(Ok(data)) = res { results.push(data); }
    }
    results
}

// Concurrency-limited
use futures::stream::StreamExt;
stream::iter(urls)
    .map(|url| async move { fetch_data(&url).await })
    .buffer_unordered(limit)
    .collect().await
```

## Channel Selection Guide

| Channel | Pattern | Use Case |
| --------- | --------- | ---------- |
| `mpsc` | Multi-producer, single-consumer | Work queues |
| `broadcast` | Multi-producer, multi-consumer | Event bus |
| `oneshot` | Single value, single use | Request-response |
| `watch` | Latest value, multi-consumer | Config/state updates |

## Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Network: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Timeout after {0:?}")]
    Timeout(std::time::Duration),
}

// Wrap with timeout
async fn with_timeout<T, F: Future<Output = Result<T, ServiceError>>>(
    dur: Duration, f: F,
) -> Result<T, ServiceError> {
    tokio::time::timeout(dur, f).await.map_err(|_| ServiceError::Timeout(dur))?
}
```

## Graceful Shutdown

```rust
use tokio_util::sync::CancellationToken;

let token = CancellationToken::new();
tokio::spawn({
    let token = token.clone();
    async move {
        loop {
            tokio::select! {
                _ = token.cancelled() => break,
                _ = do_work() => {}
            }
        }
    }
});
signal::ctrl_c().await?;
token.cancel();
```

## Shared State

- **`RwLock`** for read-heavy shared state
- **`Semaphore`** for connection/concurrency limits
- **Channels** preferred over shared state when possible

## Rules

- Never use `std::thread::sleep` in async — use `tokio::time::sleep`
- Never hold locks across `.await` points — causes deadlocks
- Never spawn unboundedly — use semaphores for limits
- Always propagate errors with `?` or log them
- Always ensure spawned futures are `Send`
- Use `#[instrument]` from tracing for async debugging
