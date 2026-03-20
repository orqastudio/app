---
id: DOC-36d28810
type: doc
title: Centralized Logging Guide
status: active
created: 2026-03-19
category: how-to
relationships:
  - target: KNOW-01a64d58
    type: synchronised-with
---

# Centralized Logging Guide

## Overview

OrqaStudio uses a centralized logging system that routes all output — from the frontend, SDK, and libraries — through a single logger. In dev mode, all log entries are forwarded to the dev controller dashboard for unified debugging across the full stack.

## Why

- **No bare `console.log`** — scattered console calls are invisible to the dev controller and can't be filtered
- **Structured output** — every entry has a source module, level, and timestamp
- **Filterable** — the dashboard lets you isolate specific modules or severity levels
- **Performance visibility** — built-in timing for async operations appears alongside regular logs
- **Enforced** — ESLint's `no-console` rule and Clippy's `print_stdout`/`print_stderr` deny prevent bare output

## Architecture

```
Frontend Code → logger("module") → Log Entry
                                      ↓
                              ┌───────┴───────┐
                              │               │
                        Browser Console   Dev Dashboard
                        (always)          (dev mode only)
                                              ↓
                                    HTTP POST → Dev Controller
                                              ↓
                                    SSE → Dashboard UI
                                              ↓
                                    Source/Level/Text Filters
```

## The Logger API

```typescript
import { logger } from "@orqastudio/sdk";
const log = logger("my-module");

log.debug("trace info");                    // dev only
log.info("operational info");               // dev only
log.warn("unexpected state");               // always
log.error("failure", err);                  // always
log.perf("label", () => syncWork());        // timing
await log.perfAsync("label", () => async());// async timing
```

## Log Levels

| Level | Console | Dashboard | When to Use |
|-------|---------|-----------|-------------|
| `debug` | Dev only | Yes | Detailed trace info during development |
| `info` | Dev only | Yes | Normal operational events (navigation, loads) |
| `perf` | Dev only | Yes | Performance timing measurements |
| `warn` | Always | Yes | Unexpected but recoverable situations |
| `error` | Always | Yes | Failures that need attention |

## Dev Controller Dashboard

Access at `http://localhost:3001` when the dev environment is running.

### Filters
- **Source buttons** — toggle individual modules on/off (navigation, artifact, graph, plugins, etc.)
- **Level buttons** — toggle debug/info/perf/warn/error visibility
- **Text search** — filter by content substring

### Source Colours
Each log source has a consistent colour in the dashboard for visual scanning:
- Process sources: controller (amber), vite (green), rust (purple)
- Library watchers: types, sdk, cli, components, software
- SDK modules: navigation (blue), artifact (green), graph (lime), plugins (tan)

## Requirements for Contributors

1. **Every `catch` block must log the error** before handling it
2. **Every IPC/API call should use `perfAsync`** for timing visibility
3. **User-initiated actions should log at `info` level** (opens, navigations, selections)
4. **No bare `console.*` calls** — the linter will catch these
5. **Choose a descriptive source name** that matches the module's responsibility
