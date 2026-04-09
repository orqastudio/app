---
id: IDEA-4d48c4a7
title: Containerise dev environment services
status: proposed
created: 2026-04-09
---

# Containerise Dev Environment Services

## Problem

The current dev environment manages 7+ processes via a Node.js ProcessManager
that spawns, monitors, and kills them directly. This causes:

1. **Orphaned processes** -- if ProcessManager dies unexpectedly (crash, OOM,
   forced reboot), all children are orphaned. The daemon is intentionally
   detached (`unref()`), so it survives parent death by design. LSP, MCP, and
   search survive as daemon children or independent processes.

2. **Port conflicts on restart** -- killed processes leave sockets in TIME_WAIT.
   `killAll()` must scan ports, kill stragglers, and wait up to 15s per port.
   Windows PID reuse adds another race condition.

3. **No unified lifecycle** -- `orqa dev` manages app services; `orqa hosting`
   manages Forgejo separately. Two commands, two lifecycles, no shared state.

4. **Fragile cleanup** -- `taskkill /T /F` on Windows, platform-specific PID
   detection, manual stale PID file cleanup. Works most of the time, fails
   silently when it doesn't.

5. **Missing health checks** -- search server has no health endpoint (JSON-RPC
   over stdio). Daemon doesn't auto-restart crashed LSP/MCP subprocesses.

## Current Process Hierarchy

```text
orqa dev (Node.js ProcessManager)
+-- cargo tauri dev (OrqaDev -- native Tauri window)
+-- cargo tauri dev (Main app -- native Tauri window)
+-- orqa-daemon (Rust, detached, port 10100)
|   +-- orqa-lsp-server (TCP :10101)
|   +-- orqa-mcp-server (TCP :10102)  ← embeds SearchEngine via orqa-search crate
+-- orqa-search-server (Rust, JSON-RPC over stdio)  ← LEGACY, redundant
+-- storybook (Node.js, port 10150)
+-- File watchers (fs.watch on all src/ directories)

orqa hosting up (separate command, separate lifecycle)
+-- Docker: Forgejo (ports 10030 HTTP, 10222 SSH)
```text

## Search Integration Status (as of 2026-04-09)

Search is already fully integrated into the daemon-managed stack:

- **MCP server** (`engine/mcp-server/src/tools/search.rs`) — 4 MCP tools
  (`search_regex`, `search_semantic`, `search_research`, `search_status`) with
  `scope` parameter (artifacts/codebase/all). Used by LLM clients.
- **Daemon HTTP routes** (`daemon/src/routes/search.rs`) — REST endpoints at
  `/search/index`, `/search/regex`, `/search/semantic`, `/search/status`. Used
  by app frontend and CLI.
- **Standalone `orqa-search-server` binary** — LEGACY. JSON-RPC over stdio,
  spawned by ProcessManager and `orqa index`. Redundant with daemon routes.

Both MCP server and daemon embed the `orqa-search` library crate directly. The
standalone binary should be retired:
1. Remove "search" service node from ProcessManager graph
2. Rewrite `orqa index` to call daemon HTTP endpoints instead of spawning binary
3. Remove `engine/search/src/bin/server.rs` (keep the library crate)
4. Remove `orqa-search-server` from `killAll()` process name list

## Proposal: Hybrid Containerisation

Containerise **runtime services**. Keep **build tooling and native UI** on the
host. Docker Compose manages service lifecycle; ProcessManager manages only
native-host concerns (Tauri apps, Vite, file watchers, builds).

### What gets containerised

| Service | Container | Ports | Why |
|---------|-----------|-------|-----|
| orqa-daemon | `orqa-daemon` | 10100-10102 | Long-running, owns LSP+MCP+search, has HTTP health endpoint |
| Forgejo | `orqastudio-git` (existing) | 10030, 10222 | Already containerised |

Note: search is NOT a separate container. The daemon embeds the search engine
via its HTTP routes and the MCP server embeds it via MCP tools. The standalone
`orqa-search-server` binary is retired.

### What stays native

| Component | Why |
|-----------|-----|
| orqa-studio (Tauri app) | Needs WebView2, system tray, native window management |
| orqa-devtools (Tauri app) | Same -- native desktop UI |
| Vite dev server | Hot reload needs native FS events, sub-100ms latency |
| Storybook | Same as Vite |
| File watchers (build) | Native ReadDirectoryChangesW/inotify for fast rebuilds |
| Rust/TS builds | Host-native filesystem speed, 93 GB target/ cache |

### Architecture after containerisation

```text
orqa dev (Node.js ProcessManager -- simplified)
+-- docker compose up -d (services)
|   +-- orqa-daemon (Rust, ports 10100-10102)
|   |   +-- orqa-lsp-server (internal, TCP :10101)
|   |   +-- orqa-mcp-server (internal, TCP :10102)
|   |   +-- search engine (embedded in daemon + MCP, no separate process)
|   +-- forgejo (ports 10030, 10222)
+-- cargo tauri dev (OrqaDev -- native)
+-- cargo tauri dev (Main app -- native)
+-- Vite dev servers (native, ports 10120/10140)
+-- storybook (native, port 10150)
+-- File watchers (native, rebuild triggers only)
```text

The daemon container is the single backend service. It owns:
- Artifact graph + validation (HTTP)
- Search engine (HTTP `/search/*` + MCP tools)
- LSP server (TCP subprocess)
- MCP server (TCP subprocess)
- File watching on `.orqa/` and `plugins/`
- System tray (disabled in container mode — tray runs native on host)

## Implementation Plan

### Phase 1: Single source of truth for search (daemon only)

Search is currently duplicated: the daemon embeds `SearchEngine` via HTTP routes
(`daemon/src/routes/search.rs`, DuckDB at `.state/search.duckdb`), and the MCP
server embeds its own `SearchEngine` instance (`server.rs:30`, DuckDB at
`.orqa/search.duckdb`). Two separate indexes, two ONNX model loads.

The daemon is already the single source of truth for graph operations — the MCP
server calls daemon via `DaemonClient` HTTP for all graph tools. Search should
follow the same pattern.

**Step 1a: MCP search tools → daemon HTTP proxy**

Rewrite MCP search tools (`engine/mcp-server/src/tools/search.rs`) to call the
daemon's `/search/*` HTTP endpoints via `DaemonClient`, same as graph tools:
- `search_regex` → `POST /search/regex` on daemon
- `search_semantic` → `POST /search/semantic` on daemon
- `search_research` → compound: `POST /search/semantic` + `POST /search/regex`
- `search_status` → `GET /search/status` on daemon

Remove from MCP server:
- `search: Option<SearchEngine>` field from `McpServer` struct
- `get_search()` lazy init method
- `orqa-search` dependency from `engine/mcp-server/Cargo.toml`
- Duplicate DuckDB file at `.orqa/search.duckdb`

Add to `DaemonClient`:
- `search_regex()`, `search_semantic()`, `search_status()` methods

**Step 1b: Retire standalone search server binary**

- Remove "search" service node from ProcessManager graph (`process-manager.ts:434-452`)
- Remove `startSearch()` method (`process-manager.ts:1238-1248`)
- Remove search restart logic (`process-manager.ts:1801-1812`)
- Remove `orqa-search-server` from `killAll()` name list (`dev.ts:234`)
- Rewrite `orqa index` to call daemon HTTP endpoints (`POST /search/index`,
  `POST /search/embed`) instead of spawning the binary
- Delete `engine/search/src/bin/server.rs` (keep the library crate)
- Remove `[[bin]]` entry from `engine/search/Cargo.toml`

**Step 1c: Daemon subprocess auto-restart**

Add exponential backoff restart for crashed LSP/MCP in `daemon/src/subprocess.rs`
(~30 lines). Max 5 retries, backoff: 2s, 4s, 8s, 16s, 30s.

### Phase 2: Docker Compose for services

Create `infrastructure/docker-compose.dev.yml`:

```yaml
services:
  daemon:
    build:
      context: ../..
      dockerfile: infrastructure/docker/Dockerfile.daemon
    ports:
      - "10100:10100"   # Health + HTTP API
      - "10101:10101"   # LSP (TCP)
      - "10102:10102"   # MCP (TCP)
    volumes:
      - ../../.orqa:/project/.orqa
      - ../../plugins:/project/plugins
      - ../../models:/project/models:ro
      - daemon-state:/project/.state
    environment:
      - ORQA_PORT_BASE=10100
      - ORQA_EMBED_MODEL=/project/models/all-MiniLM-L6-v2
      - RUST_LOG=info
      - ORQA_HEADLESS=1
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:10100/health"]
      interval: 5s
      timeout: 3s
      retries: 3
    restart: unless-stopped

  forgejo:
    image: codeberg.org/forgejo/forgejo:10
    container_name: orqastudio-git
    restart: unless-stopped
    environment:
      FORGEJO__database__DB_TYPE: sqlite3
      FORGEJO__server__DOMAIN: localhost
      FORGEJO__server__SSH_DOMAIN: localhost
      FORGEJO__server__ROOT_URL: http://localhost:10030/
      FORGEJO__server__HTTP_PORT: "3000"
      FORGEJO__server__SSH_PORT: "10222"
      FORGEJO__server__DISABLE_SSH: "false"
      FORGEJO__server__LFS_START_SERVER: "true"
      FORGEJO__security__INSTALL_LOCK: "true"
      FORGEJO__repository__DEFAULT_BRANCH: main
      FORGEJO__repository__ENABLE_PUSH_CREATE_USER: "true"
      FORGEJO__mirror__ENABLED: "true"
      FORGEJO__mirror__DEFAULT_INTERVAL: 1h
      FORGEJO__actions__ENABLED: "true"
      FORGEJO__service__DISABLE_REGISTRATION: "false"
      FORGEJO__service__REQUIRE_SIGNIN_VIEW: "false"
    volumes:
      - forgejo-data:/data
      - forgejo-config:/etc/forgejo
    ports:
      - "10030:3000"
      - "10222:22"

volumes:
  daemon-state:
  forgejo-data:
  forgejo-config:
```text

### Phase 3: Daemon Dockerfile (multi-stage, cached)

```dockerfile
# Dockerfile.daemon — daemon + LSP + MCP + embedded search
FROM rust:1.82-slim AS builder
WORKDIR /build

# Cache: copy only manifests first, build deps
COPY Cargo.toml Cargo.lock ./
COPY daemon/Cargo.toml daemon/Cargo.toml
COPY engine/ engine/
RUN mkdir -p daemon/src && echo "fn main() {}" > daemon/src/main.rs \
    && cargo build --release -p orqa-daemon 2>/dev/null || true

# Real build: copy actual source
COPY daemon/ daemon/
RUN cargo build --release -p orqa-daemon -p orqa-lsp-server -p orqa-mcp-server

# Runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/orqa-daemon /usr/local/bin/
COPY --from=builder /build/target/release/orqa-lsp-server /usr/local/bin/
COPY --from=builder /build/target/release/orqa-mcp-server /usr/local/bin/
WORKDIR /project
ENV ORQA_HEADLESS=1
ENTRYPOINT ["orqa-daemon"]
```text

The `ORQA_HEADLESS=1` env var tells the daemon to skip the system tray (no GUI
in a container).

### Phase 3b: Native tray binary (`orqa-tray`)

The system tray runs natively on the host, polling the containerised daemon's
health endpoint for status. This preserves the native system tray UX while the
daemon runs headless in a container.

**Current (in-process):** tray reads `Arc<Mutex<SubprocessStatuses>>` from the
daemon's event loop thread. Both run in the same process.

**Containerised:** tray is a standalone native binary that polls
`GET /health` on the daemon container.

```text
Docker: orqa-daemon (ORQA_HEADLESS=1, no tray)
  └── GET /health returns { processes: [{ name, status, pid, uptime }...] }

Host: orqa-tray (native binary, polls /health every 2s)
  └── renders tray icon + context menu from HTTP response
  └── "Quit" sends POST /shutdown to daemon (or docker compose down)
  └── "Open App" launches browser to Vite dev server
```text

Implementation: extract `build_icon()`, `build_menu()`, `pump_messages()`,
`process_events()` from `daemon/src/tray.rs` into a shared crate. The tray
binary replaces the `Arc<Mutex>` read with a `reqwest::blocking::get()` call.
~100 lines of new code; most logic is reused from existing `tray.rs`.

ProcessManager starts `orqa-tray` as a native managed process alongside the
Docker Compose services.

### Phase 4: ProcessManager integration

Simplify ProcessManager — services are Docker containers, not spawned processes:

```typescript
async startServices(): Promise<void> {
  // One command starts daemon + forgejo
  execSync(
    `docker compose -f infrastructure/docker-compose.dev.yml up -d --wait`,
    { cwd: this.root, stdio: "inherit" },
  );
  // --wait blocks until healthchecks pass

  // Storybook stays native
  await this.startStorybook();
}

async shutdown(): Promise<void> {
  // Close file watchers, kill build processes (unchanged)
  // ...

  // Kill native processes (app, storybook)
  for (const [id, child] of this.managedProcesses) {
    if (child.pid) killProcessTree(child.pid);
  }

  // One command stops all services — no orphans possible
  execSync(
    `docker compose -f infrastructure/docker-compose.dev.yml down`,
    { cwd: this.root, stdio: "inherit" },
  );
}
```text

Remove from ProcessManager:
- `startDaemon()` / `startSearch()` methods
- Daemon PID file management
- Service crash-restart logic (Docker handles this)
- Platform-specific process killing for services

### Phase 5: Unified `orqa dev` startup

Merge `orqa hosting` into `orqa dev` — Forgejo is just another service in the
compose stack. Remove the separate `orqa hosting up/down` commands (keep `setup`,
`push`, `mirror` as `orqa git` subcommands).

## What this solves

| Problem | Before | After |
|---------|--------|-------|
| Orphaned processes after crash | Manual `orqa dev kill` | `docker compose down` or Docker auto-cleanup |
| Stale port conflicts | Port scanning + 15s wait per port | Container restarts get fresh ports |
| Windows PID reuse | Stale PID file race condition | Docker manages PIDs internally |
| LSP/MCP crash recovery | No auto-restart | `restart: unless-stopped` + healthcheck |
| Standalone search server | Redundant process, no health check | Retired — daemon owns search |
| Two startup commands | `orqa dev` + `orqa hosting up` | `orqa dev` (unified) |
| Platform-specific kill logic | `taskkill /T /F` vs `SIGKILL` | `docker compose down` (cross-platform) |

## Dev Mode: Auto-Rebuild Strategy

On Windows, host Rust builds produce PE binaries that cannot run inside Linux
containers. This is the central tension for containerised dev mode. Three
approaches, with a recommended default:

### Option A: Two-profile dev mode (RECOMMENDED)

Two distinct modes for the daemon, selectable at startup:

**`orqa dev` (default — containerised daemon, stable backend)**

The daemon runs in Docker from a pre-built image. No Rust source watching,
no daemon rebuilds. Frontend hot-reload works normally. This is the day-to-day
workflow — most dev time is frontend work against a stable backend.

```text
orqa dev (ProcessManager)
+-- Docker Compose
|   +-- orqa-daemon (container, ports 10100-10102)
|   +-- forgejo (container, ports 10030/10222)
+-- orqa-tray (native, polls /health)
+-- cargo tauri dev (OrqaDev + App — native)
+-- Vite dev servers (native, hot-reload)
+-- File watchers for TS/Svelte libs (hot-reload)
+-- No Rust file watchers (container image is static)
```text

Rebuild the backend manually when needed: `orqa dev rebuild daemon` →
`docker compose build daemon && docker compose up -d daemon`

**`orqa dev --include-backend` (native daemon, full hot-reload)**

The daemon runs natively on the host. File watchers trigger `cargo build`
(incremental, fast, uses the 93 GB target/ cache) and restart the daemon.
Use this when actively changing Rust engine/daemon code.

```text
orqa dev --include-backend (ProcessManager — full)
+-- Docker Compose
|   +-- forgejo (container, ports 10030/10222)
+-- orqa-daemon (native, port 10100)
|   +-- orqa-lsp-server (native subprocess)
|   +-- orqa-mcp-server (native subprocess)
+-- orqa-tray (native, polls /health)
+-- cargo tauri dev (OrqaDev + App — native)
+-- Vite dev servers (native, hot-reload)
+-- File watchers for everything (TS, Svelte, Rust → rebuild + restart)
```text

Process lifecycle in this mode:

- Forgejo still gets Docker lifecycle (no orphans, auto-restart)
- Daemon gets subprocess auto-restart for LSP/MCP (Phase 1c)
- `killAll()` handles native daemon cleanup on crash

### Option B: cargo watch inside container (LINUX/MAC ONLY)

Mount the source tree into the container and run `cargo watch` inside it.
On Linux and macOS, Docker bind mounts have near-native filesystem performance,
so this works well. On Windows, the WSL2 filesystem bridge adds 2-5x overhead
to Rust builds — not recommended.

```yaml
daemon-dev:
  build:
    context: ../..
    dockerfile: infrastructure/docker/Dockerfile.daemon-dev
  volumes:
    - ../../daemon:/workspace/daemon
    - ../../engine:/workspace/engine
    - cargo-registry:/usr/local/cargo/registry
    - cargo-target:/workspace/target
  command: cargo watch -w daemon/src -w engine -x 'run -p orqa-daemon'
```text

Performance (incremental rebuild after single-file change):
- Linux/macOS: ~10-30s (near-native)
- Windows (WSL2 bridge): ~40-90s (filesystem overhead dominates)

### Option C: Cross-compile on host + mount binary (ADVANCED)

Cross-compile Linux binaries on the Windows host using `cross` or
`cargo build --target x86_64-unknown-linux-gnu`, then bind-mount the
binary into a minimal runtime container. Fast rebuilds + container lifecycle.

Downsides: requires Linux cross-compilation toolchain on Windows (cross-rs,
or WSL2 with Rust installed). Adds toolchain complexity.

### Rebuild Matrix

What auto-rebuilds on file change vs what needs `orqa dev restart <target>`:

**`orqa dev` (native daemon, default mode)**

| Component | Change in | Auto-rebuild? | Mechanism | Downtime |
|-----------|-----------|---------------|-----------|----------|
| TypeScript libs | `libs/*/src/` | Yes | File watcher → `tsc` | None (HMR) |
| Svelte components | `libs/svelte-components/src/` | Yes | File watcher → `svelte-package` | None (HMR) |
| App frontend | `app/src/` | Yes | Vite HMR | None |
| Devtools frontend | `devtools/src/` | Yes | Vite HMR | None |
| App Rust backend | `app/src-tauri/src/` | Yes | `cargo tauri dev` watches + rebuilds | ~10-30s (app restart) |
| Devtools Rust backend | `devtools/src-tauri/src/` | Yes | `cargo tauri dev` watches + rebuilds | ~10-30s (devtools restart) |
| Engine crates | `engine/*/src/` | Yes | File watcher → `cargo build --workspace` → daemon restart | ~15-40s |
| Daemon source | `daemon/src/` | Yes | File watcher → `cargo build --workspace` → daemon restart | ~15-40s |
| Plugins | `plugins/*/` | Yes | File watcher → plugin build → `orqa plugin refresh` | None |
| Storybook stories | `libs/svelte-components/**/*.stories.ts` | Yes | Storybook HMR | None |
| CLI source | `cli/src/` | **No** | Needs `orqa dev restart` | Full restart |
| ProcessManager changes | `cli/src/lib/process-manager.ts` | **No** | Needs `orqa dev restart` | Full restart |
| Docker Compose config | `infrastructure/docker-compose.dev.yml` | **No** | Needs `orqa dev restart` | Full restart |
| Cargo.toml deps | `*/Cargo.toml` | **No** | Needs `orqa dev restart` (full rebuild) | Full restart |
| package.json deps | `*/package.json` | **No** | Needs `npm install` + `orqa dev restart` | Full restart |
| ONNX models | `models/` | **No** | Needs daemon restart to reload | Daemon restart |

**`orqa dev --include-backend` (containerised daemon)**

| Component | Change in | Auto-rebuild? | Mechanism | Downtime |
|-----------|-----------|---------------|-----------|----------|
| App/devtools frontend | `app/src/`, `devtools/src/` | Yes | Vite HMR | None |
| App/devtools Rust backend | `app/src-tauri/`, `devtools/src-tauri/` | Yes | `cargo tauri dev` | ~10-30s |
| TypeScript libs | `libs/*/src/` | Yes | File watcher → `tsc` | None |
| Svelte components | `libs/svelte-components/src/` | Yes | File watcher → `svelte-package` | None |
| Plugins | `plugins/*/` | Yes | File watcher → plugin build | None |
| Engine/daemon Rust code | `engine/`, `daemon/` | **No** | `orqa dev rebuild daemon` | ~2-5 min (container rebuild) |
| Everything else | Config, deps, CLI | **No** | `orqa dev restart` | Full restart |

### Recommendation

Use **Option A** (two-profile) as the default:

- `orqa dev` — containerised daemon, stable backend, frontend hot-reload (default)
- `orqa dev --include-backend` — native daemon, full Rust hot-reload (active engine work)
- Option B (cargo watch in container) available on Linux/macOS where FS performance permits
- Option C (cross-compile) is a future optimisation

The key insight: most dev time is frontend work. The default should optimise for
that. Containerisation gives clean lifecycle management (no orphans, auto-restart,
unified startup) as a bonus. `--include-backend` opts into the heavier native
mode only when you're actively touching Rust code.

## What this doesn't solve (and shouldn't)

- **Hot reload latency** -- Vite and Storybook stay native for sub-100ms HMR.
- **Native UI** -- Tauri apps stay native (WebView2, system tray).
- **File watcher responsiveness** -- Build watchers stay native for instant
  rebuild triggers.

## Trade-offs

| Cost | Mitigation |
|------|------------|
| Docker Desktop required | Already required for Forgejo |
| Volume mount latency for `.orqa/` | Daemon file watcher tolerates 1-2s latency (debounced at 500ms already) |
| Container build time | Multi-stage Dockerfiles with dependency caching. Pre-built images for CI. |
| Harder to debug Rust services | `docker compose exec daemon bash` + remote debugger attach |
| ONNX runtime in container | CPU-only in container; DirectML for native production builds |
| Image size | ~200 MB per service image (Rust binaries are static-ish) |

## Sequencing

| Step | Effort | Unblocks |
|------|--------|----------|
| 1a. MCP search → daemon HTTP proxy (single source of truth) | Medium | 1b |
| 1b. Retire standalone search server binary | Small | 2 |
| 1c. Daemon subprocess auto-restart (LSP/MCP) | Small | 2 |
| 2. Daemon Dockerfile (multi-stage, ORQA_HEADLESS) | Medium | 3 |
| 3a. Unified docker-compose.dev.yml (daemon + forgejo) | Medium | 4 |
| 3b. Native `orqa-tray` binary (polls daemon /health) | Small | 4 |
| 4. ProcessManager integration (services via compose) | Medium | 5 |
| 5. Unified `orqa dev` startup, retire `orqa hosting` | Small | Done |

## Open questions

1. **Dev vs pre-built images** -- should `orqa dev` build containers from source
   (slow first time, always current) or pull pre-built images (fast, may be
   stale)? Proposal: build from source by default, CI publishes images for
   fresh-clone speed.

2. **Volume mount strategy** -- `.orqa/` needs read-write for daemon (file
   watcher writes .state/). `plugins/` needs read-only. Models need read-only.
   Should we use named volumes or bind mounts?

3. **Port base override** -- currently `ORQA_PORT_BASE` shifts all ports. Docker
   Compose port mappings are static. Do we template the compose file, or fix
   ports inside containers and only remap host-side?

## Additional findings from investigation

### Vite port mismatch

`libs/constants/src/ports.ts` defines vite offset as 20 (= port 10120), but
`app/vite.config.ts` and `app/src-tauri/tauri.conf.json` hardcode 10420. These
should be reconciled — either update the constant or the configs.

### Missing integration: `orqa dev` doesn't start Forgejo

Currently two separate commands with no shared lifecycle. The unified compose
stack (Phase 5) fixes this naturally.

### Daemon subprocess auto-restart

Independent of containerisation, the daemon should auto-restart crashed LSP/MCP
subprocesses with exponential backoff. This is a ~30 line change in
`daemon/src/subprocess.rs` and improves reliability whether containerised or not.
