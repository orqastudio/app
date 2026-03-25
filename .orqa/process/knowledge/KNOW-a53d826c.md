---
id: KNOW-a53d826c
type: knowledge
title: Centralized Logging
description: |
  How to use the OrqaStudio centralized logging system. ALL code must use the
  SDK logger instead of bare console calls. Covers log levels, module scoping,
  error capture, perf instrumentation, and dev controller integration.
  Use when: writing any code that produces output, handles errors, or measures performance.
status: active
created: 2026-03-19
updated: 2026-03-19
category: tool
version: 1.0.0
user-invocable: false
relationships:
  - target: DOC-ddba21f4
    type: synchronised-with
  - target: DOC-9814ec3c
    type: synchronised-with
tier: "stage-triggered"
roles:
  - "implementer"
stages:
  - "implement"
paths:
  - "**/*.ts"
  - "**/*.rs"
tags:
  - "logging"
  - "centralized"
  - "sdk"
priority: "P1"
summary: |
  Centralized logging: ALL code must use SDK logger, never bare console calls.
  Covers log levels, module scoping, error capture, perf instrumentation, and
  dev controller integration.

---

# Centralized Logging

## Rule: No Bare Console Calls

**NEVER use `console.log`, `console.warn`, `console.error`, `console.time`, or `console.debug` directly.** All output must go through the centralized logger from `@orqastudio/sdk`.

This is enforced by:
- **TypeScript**: ESLint `no-console: error` (via `@orqastudio/plugin-typescript`)
- **Rust**: `clippy::print_stdout` and `clippy::print_stderr` deny (via `@orqastudio/plugin-rust`)

## How to Use the Logger

### Import and Create a Scoped Logger

```typescript
import { logger } from "@orqastudio/sdk";
const log = logger("my-module");
```

Within the SDK itself (not an external consumer):
```typescript
import { logger } from "../logger.js";
const log = logger("my-module");
```

### Log Levels

```typescript
log.debug("Detailed trace info");       // Only shown in dev mode
log.info("Normal operational info");     // Shown in dev mode
log.warn("Something unexpected");        // Always shown
log.error("Something failed", err);      // Always shown, forwarded to error store
```

### Performance Measurement

```typescript
// Synchronous — auto-times the function
log.perf("processNodes", () => {
  // ... work ...
});

// Asynchronous — auto-times the promise
const result = await log.perfAsync("loadContent", () =>
  fetchContent(path),
);

// Manual label (no timing)
log.perf("render complete");
```

### Error Handling Pattern

Every `try/catch` block that catches an error MUST log it:

```typescript
try {
  const result = await invoke("some_command");
} catch (err) {
  log.error("Failed to invoke some_command", err);
  // Then handle the error (set error state, show toast, etc.)
}
```

### Subscribing to Logs

For in-app error display or telemetry:

```typescript
import { subscribeToLogs } from "@orqastudio/sdk";

const unsubscribe = subscribeToLogs((entry) => {
  if (entry.level === "error") {
    // Show in error panel, send to telemetry, etc.
  }
});
```

## Module Naming Convention

Use short, descriptive names that match the module's responsibility:

| Module | Logger Source |
|--------|-------------|
| Navigation store | `logger("navigation")` |
| Artifact store | `logger("artifact")` |
| Graph SDK | `logger("graph")` |
| Plugin loader | `logger("plugins")` |
| Graph layout service | `logger("graph-layout")` |
| References panel | `logger("references")` |
| Error store | `logger("errors")` |

## What Gets Logged Where

### Dev Mode
All levels (debug through error) are:
1. Output to browser console with `[source]` prefix
2. Forwarded to the dev controller dashboard via HTTP POST
3. Available to log subscribers

### Production Mode
Only `warn` and `error` are:
1. Output to browser console
2. Available to log subscribers (for in-app error display)

## Dev Controller Dashboard

The dashboard at `http://localhost:10401` provides:
- **Source filters**: toggle visibility per module (navigation, artifact, graph, etc.)
- **Level filters**: debug, info, perf, warn, error
- **Text search**: filter log lines by content
- **Color coding**: each source has a distinct colour

## Where Logging Is Required

Every code path that:
1. **Catches an error** → `log.error(message, err)`
2. **Makes an IPC/API call** → `log.perfAsync(label, fn)` for timing
3. **Handles user actions** → `log.info(action)` for navigation, artifact opens
4. **Encounters unexpected state** → `log.warn(description)`
5. **Performs expensive computation** → `log.perf(label, fn)` for timing

## Rust Backend

The Rust backend uses the `tracing` crate (not `println!`). Tracing output appears in the dev controller under the `[rust]` source. The `RUST_LOG` environment variable controls the level (default: `info`).

```rust
use tracing::{info, warn, error, debug};

info!("Project loaded: {}", path);
warn!("Missing config file, using defaults");
error!("Failed to scan artifacts: {}", err);
```