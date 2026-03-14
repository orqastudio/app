---
id: DOC-020
title: Artifact Graph SDK
description: Development guide for the Artifact Graph SDK — the typed frontend interface to the bidirectional artifact node graph.
created: 2026-03-10
updated: 2026-03-10
category: development
tags:
  - artifact-graph
  - sdk
  - typescript
  - svelte5
  - ipc
  - plugins
relationships:
  - target: EPIC-048
    type: documents
    rationale: Documentation page references EPIC-048
---


# Artifact Graph SDK

## Overview

The artifact graph is a bidirectional in-memory graph of every governance artifact under `.orqa/`. Each `.md` file with a YAML `id` frontmatter field becomes a node. Frontmatter fields that reference other artifact IDs (such as `epic`, `milestone`, `depends-on`) become directed edges.

The graph exists to make artifact relationships queryable without reading individual files. It replaces ad-hoc patterns — hardcoded prefix maps, direct `invoke("read_artifact")` calls, manual frontmatter parsing — with a single typed interface that any component, store, or plugin can use.

The Artifact Graph SDK is a Svelte 5 rune-based module that maintains a local copy of this graph. After `initialize()` is called, all resolution and query methods operate synchronously against the cached data — no IPC round-trips are needed for lookups. Only `readContent()` makes a round-trip, because raw file content is never cached locally.

The SDK lives at `ui/src/lib/sdk/artifact-graph.svelte.ts` and is exported as a singleton called `artifactGraphSDK`.

## Architecture

The data pipeline from files on disk to components in the UI:

```text
.orqa/ markdown files
        |
        v
  build_artifact_graph()              — Rust domain function, two-pass construction
  (artifact_graph.rs)
  - Pass 1: walk all .md files, collect nodes + references_out
  - Pass 2: invert references_out into references_in (backlinks)
        |
        v
  ArtifactGraph (in-memory)           — Cached in AppState behind Mutex
  - Lazy-init on first command call
  - Invalidated by file watcher when .orqa/ changes
        |
        v
  8 Tauri commands (graph_commands.rs)
  - resolve_artifact, resolve_artifact_path
  - get_references_from, get_references_to
  - get_artifacts_by_type
  - read_artifact_content
  - get_graph_stats
  - refresh_artifact_graph
        |
        v
  ArtifactGraphSDK (artifact-graph.svelte.ts)
  - graph: SvelteMap<string, ArtifactNode>   — keyed by artifact ID
  - pathIndex: SvelteMap<string, string>     — path → ID reverse lookup
  - Reactive state via $state runes
  - Auto-refresh on "artifact-graph-updated" Tauri event
        |
        v
  Components and stores
  - navigationStore.navigateToArtifact(id)
  - ArtifactLink.svelte — resolve + broken link detection
  - FrontmatterHeader.svelte — isArtifactId(), isBrokenPath()
  - artifactStore.loadContent(path) — delegates to readContent()
```

The file watcher (`artifact_watch_start` command) monitors `.orqa/` for changes and emits two events:

- `"artifact-changed"` — consumed by `AppLayout` to refresh the nav tree and project settings
- `"artifact-graph-updated"` — consumed by the SDK to rebuild the in-memory graph

These are separate events because the nav tree and the graph serve different purposes: the nav tree defines sidebar structure, the graph provides artifact data.

## Getting Started

The SDK is a singleton — import it directly from the module:

```typescript
import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
```

The app shell (`AppLayout.svelte`) calls `artifactGraphSDK.initialize()` once when a project becomes active. Plugins and components can assume the graph is available and react to `loading` / `error` state for the brief initialization window.

## API Reference

### Types

The types are defined in `ui/src/lib/types/artifact-graph.ts` and mirror the Rust structs in `backend/src-tauri/src/domain/artifact_graph.rs`. Field names use `snake_case` to match the Rust serde serialization output.

```typescript
interface ArtifactNode {
    /** Frontmatter `id` field (e.g. "EPIC-048"). */
    id: string;
    /** Relative path from the project root (e.g. ".orqa/delivery/epics/EPIC-048.md"). */
    path: string;
    /** Inferred category string (e.g. "epic", "task", "milestone", "idea", "decision"). */
    artifact_type: string;
    /** Frontmatter `title` field, or a humanized fallback from the filename. */
    title: string;
    /** Frontmatter `description` field. */
    description: string | null;
    /** Frontmatter `status` field. */
    status: string | null;
    /** Full YAML frontmatter parsed into a generic JSON object. */
    frontmatter: Record<string, unknown>;
    /** Forward references declared in this node's frontmatter. */
    references_out: ArtifactRef[];
    /** Backlinks computed from other nodes' `references_out` during graph construction. */
    references_in: ArtifactRef[];
}

interface ArtifactRef {
    /** The artifact ID that is referenced (the link target). */
    target_id: string;
    /** Name of the frontmatter field that contains this reference. */
    field: string;
    /** ID of the artifact that declares this reference (the link source). */
    source_id: string;
}

interface GraphStats {
    /** Total number of nodes (artifacts with an `id` field). */
    node_count: number;
    /** Total number of directed edges (sum of all `references_out` lengths). */
    edge_count: number;
    /** Nodes that have no `references_out` and no `references_in`. */
    orphan_count: number;
    /** References whose `target_id` does not exist in the graph. */
    broken_ref_count: number;
}

type ArtifactGraphType =
    | "epic" | "task" | "milestone" | "idea" | "decision"
    | "research" | "lesson" | "rule" | "agent" | "skill"
    | "hook" | "pillar" | "doc";
```

### Reactive State

All state is exposed as Svelte 5 `$state` runes. Components reading these properties will re-render when the values change.

| Property | Type | Description |
|----------|------|-------------|
| `graph` | `SvelteMap<string, ArtifactNode>` | In-memory node store, keyed by artifact ID (this is the `nodes` map) |
| `pathIndex` | `SvelteMap<string, string>` | Reverse-lookup index: relative path to artifact ID |
| `stats` | `GraphStats \| null` | Summary statistics from the last refresh |
| `loading` | `boolean` | True while a refresh or initialization is in progress |
| `lastRefresh` | `Date \| null` | Timestamp of the last successful refresh |
| `error` | `string \| null` | Error message from the last failed operation, or null when healthy |

### Resolution Methods

These methods are synchronous — they read from the in-memory `graph` and `pathIndex` maps.

```typescript
resolve(id: string): ArtifactNode | undefined
```

Resolve an artifact node by its frontmatter ID (e.g. `"[EPIC-048](EPIC-048)"`). Returns `undefined` when no artifact with the given ID exists in the graph.

```typescript
resolveByPath(path: string): ArtifactNode | undefined
```

Resolve an artifact node by its relative file path (e.g. `".orqa/delivery/epics/[EPIC-048](EPIC-048).md"`). Returns `undefined` when no artifact at that path has been indexed.

### Relationship Methods

```typescript
referencesFrom(id: string): ArtifactRef[]
```

Return all forward references (outgoing edges) declared in the artifact's frontmatter. Returns an empty array if the artifact does not exist or has no outgoing references.

```typescript
referencesTo(id: string): ArtifactRef[]
```

Return all backlinks (incoming edges) to the artifact — references declared by other artifacts pointing at this one. Backlinks are computed during graph construction and are not stored in the source files.

### Bulk Query Methods

```typescript
byType(type: string): ArtifactNode[]
```

Return all nodes whose `artifact_type` matches the given string (e.g. `"epic"`, `"task"`). The type is inferred from the artifact's directory path during graph construction.

```typescript
byStatus(status: string): ArtifactNode[]
```

Return all nodes whose frontmatter `status` field matches the given string (e.g. `"in-progress"`, `"done"`).

Both methods iterate the full in-memory graph and return a new array on each call. For reactive derived state, wrap the call in `$derived`.

### Content Reading

```typescript
async readContent(path: string): Promise<string>
```

Read the raw markdown content of an artifact file from disk. This is always an async IPC call — the SDK does not cache file content. Pass the relative path from the project root (e.g. `".orqa/delivery/epics/[EPIC-048](EPIC-048).md"`).

### Graph Health

```typescript
brokenRefs(): ArtifactRef[]
```

Return all forward references (`references_out`) whose `target_id` does not exist in the graph. A non-empty result indicates frontmatter fields that reference artifact IDs that have not been created yet or have been deleted.

```typescript
orphans(): ArtifactNode[]
```

Return all nodes that have neither outgoing nor incoming references. An orphaned artifact is not linked from any other artifact and does not link to any other artifact.

### Subscriptions (Plugin API)

Subscriptions allow code to react to graph refreshes without polling reactive state. They are the intended extension point for plugins.

```typescript
subscribe(id: string, callback: (node: ArtifactNode) => void): () => void
```

Subscribe to changes for a specific artifact by ID. The callback fires after every graph refresh where the node exists. Returns an unsubscribe function — call it to cancel the subscription.

```typescript
subscribeType(type: string, callback: (nodes: ArtifactNode[]) => void): () => void
```

Subscribe to changes for all artifacts of a given type. The callback receives the full array of nodes of that type after every graph refresh. Returns an unsubscribe function.

### Lifecycle

```typescript
async initialize(): Promise<void>
```

Initialize the SDK: fetch the full graph from the backend and register for auto-refresh on `"artifact-graph-updated"` events. Safe to call multiple times — subsequent calls are no-ops when the graph is already loaded and healthy. Call this once when a project becomes active.

```typescript
async refresh(): Promise<void>
```

Rebuild the backend graph from disk, then re-fetch all nodes into the local cache. Updates `stats`, `graph`, `pathIndex`, and `lastRefresh`. The SDK calls this automatically when it receives an `"artifact-graph-updated"` event — manual calls are only needed when you need to force a refresh outside the normal watcher cycle.

The SDK does not expose a `destroy()` method. The singleton persists for the application session. To tear down subscriptions you registered, call the unsubscribe function returned by `subscribe()` or `subscribeType()`.

## Usage Patterns

### Resolving an Artifact

Use `resolve()` when you have an artifact ID and need its metadata:

```typescript
import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";

const node = artifactGraphSDK.resolve("EPIC-048");
if (!node) {
    console.warn("EPIC-048 not found in artifact graph");
    return;
}
console.log(node.title, node.status);
```

In a Svelte component, derive resolvability reactively:

```svelte
<script lang="ts">
    import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";

    let { id }: { id: string } = $props();

    const node = $derived(artifactGraphSDK.resolve(id));
</script>

{#if node}
    <span>{node.title}</span>
{:else}
    <span class="text-warning">Unknown: {id}</span>
{/if}
```

### Navigating to an Artifact

Combine `resolve()` with `navigateToArtifact()` from the navigation store:

```typescript
import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
import { navigationStore } from "$lib/stores/navigation.svelte";

function openArtifact(id: string) {
    const node = artifactGraphSDK.resolve(id);
    if (!node) {
        console.warn(`[openArtifact] cannot resolve: ${id}`);
        return;
    }
    navigationStore.navigateToPath(node.path);
}
```

`navigationStore.navigateToArtifact(id)` wraps this pattern — it calls `resolve()` internally and falls back gracefully when the node is not found.

### Checking if a Value is a Valid Artifact Link

`FrontmatterHeader.svelte` uses `resolve()` to distinguish artifact IDs from plain strings in frontmatter values:

```typescript
function isArtifactId(value: string): boolean {
    return artifactGraphSDK.resolve(value.trim()) !== undefined;
}
```

Because `resolve()` is synchronous and reads from `$state`, using it inside a Svelte template expression is reactive — the result updates automatically when the graph refreshes.

### Reading Content with Graph Metadata

Combine `resolveByPath()` for metadata with `readContent()` for the body:

```typescript
import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";

async function loadArtifact(path: string) {
    const node = artifactGraphSDK.resolveByPath(path);
    const content = await artifactGraphSDK.readContent(path);
    return { node, content };
}
```

The `node.frontmatter` object contains the full parsed YAML — use it instead of parsing the content again. Only call `readContent()` when you need the raw markdown body.

### Subscribing to Changes (Plugin Pattern)

Subscribe to a specific artifact and clean up when the Svelte component is destroyed:

```svelte
<script lang="ts">
    import { onDestroy } from "svelte";
    import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";

    let { epicId }: { epicId: string } = $props();

    let epicTitle = $state<string | null>(null);

    const unsubscribe = artifactGraphSDK.subscribe(epicId, (node) => {
        epicTitle = node.title;
    });

    onDestroy(unsubscribe);
</script>
```

To subscribe to an entire artifact type:

```typescript
const unsubscribe = artifactGraphSDK.subscribeType("task", (tasks) => {
    const inProgress = tasks.filter(t => t.status === "in-progress");
    // update plugin state
});
```

Always call the returned unsubscribe function when the subscriber is no longer active. Subscriptions that are never unsubscribed will fire on every graph refresh for the lifetime of the application.

## Plugin Integration

Plugins consume the SDK by importing the singleton. No additional initialization is needed — the app shell calls `initialize()` when a project becomes active.

Synchronous reads (`resolve`, `byType`, `byStatus`, `referencesFrom`, `referencesTo`) work inside Svelte templates and `$derived` expressions because `graph` is `$state`. For live updates across graph refreshes, register a subscription via `subscribe()` or `subscribeType()` and cancel it with the returned function when the plugin is torn down (see `onDestroy` pattern in the Usage Patterns section above).

Content reads via `readContent()` are always async IPC calls — do not call them inside `$derived`. Cache the result in `$state` if needed, or call from a `$effect` or event handler.

## Migration Guide

### From ARTIFACT_PREFIX_MAP

The old approach used a hardcoded map from artifact ID prefix to directory path:

```typescript
// OLD — fragile, misses types without prefix entries
const ARTIFACT_PREFIX_MAP: Record<string, string> = {
    "EPIC": ".orqa/delivery/epics",
    "TASK": ".orqa/delivery/tasks",
    // ...
};
```

Replace with `resolve()`:

```typescript
// NEW — works for every artifact type with an id field
const node = artifactGraphSDK.resolve(id);
if (node) {
    navigationStore.navigateToPath(node.path);
}
```

### From invoke('read_artifact')

The old pattern called `invoke("read_artifact")` directly and managed a `viewerCache` in the store:

```typescript
// OLD — manual IPC + in-memory cache
const content = await invoke<string>("read_artifact", { path });
this.viewerCache.set(path, content);
```

Replace with the SDK:

```typescript
// NEW — SDK handles IPC, no frontend cache needed
const content = await artifactGraphSDK.readContent(path);
```

The SDK intentionally does not cache content — files change on disk, and stale caches cause subtle bugs. `readContent()` is always fresh.

### From parseFrontmatter()

The old pattern parsed frontmatter from raw content on every render:

```typescript
// OLD — parse overhead on every render
const { frontmatter } = parseFrontmatter(content);
const title = frontmatter.title;
```

Replace with a graph lookup:

```typescript
// NEW — frontmatter already parsed and stored in the graph
const node = artifactGraphSDK.resolveByPath(path);
const title = node?.title ?? humanize(path);
```

`parseFrontmatter()` remains available as a lightweight fallback for files that are not yet in the graph (for example, a newly created file before the watcher fires).

### From pendingArtifactId

The old navigation store used a `pendingArtifactId` field and `label.startsWith()` string matching to auto-select artifacts after a navigation request:

```typescript
// OLD — label matching, broke for tree-structured directories
this.pendingArtifactId = id;
// then in a $effect:
if (node.label.startsWith(this.pendingArtifactId)) { ... }
```

Replace with exact path resolution:

```typescript
// NEW — resolve to path, navigate directly
const node = artifactGraphSDK.resolve(id);
if (node) navigationStore.navigateToPath(node.path);
```

## Backend Integration

### Tauri Commands

The SDK communicates with the backend through eight Tauri commands:

| Command | Signature | Description |
|---------|-----------|-------------|
| `resolve_artifact` | `(id: string) → ArtifactNode \| null` | Resolve a node by its frontmatter ID |
| `resolve_artifact_path` | `(path: string) → ArtifactNode \| null` | Resolve a node by its relative file path |
| `get_references_from` | `(id: string) → ArtifactRef[]` | Get outgoing references from a node |
| `get_references_to` | `(id: string) → ArtifactRef[]` | Get incoming backlinks to a node |
| `get_artifacts_by_type` | `(artifact_type: string) → ArtifactNode[]` | Get all nodes of a given type |
| `read_artifact_content` | `(path: string) → string` | Read raw markdown from disk |
| `get_graph_stats` | `() → GraphStats` | Get summary statistics |
| `refresh_artifact_graph` | `() → void` | Rebuild the graph from disk |

The SDK calls `get_artifacts_by_type` for every known type in parallel during initialization, then assembles the full in-memory graph client-side. It calls `refresh_artifact_graph` followed by `get_artifacts_by_type` again on every auto-refresh.

Path traversal is rejected by all path-based commands — paths containing `..` return an error.

### File Watcher

The file watcher is started from `AppLayout.svelte` via `invoke("artifact_watch_start", { projectPath })`. It monitors `.orqa/` for file system changes (create, modify, delete, rename).

When changes are detected:

1. The watcher emits `"artifact-changed"` — consumed by `AppLayout` to refresh the nav tree and project settings.
2. The watcher emits `"artifact-graph-updated"` — consumed by the SDK, which calls `refresh()` automatically.

`refresh()` calls `refresh_artifact_graph` (rebuilds the Rust-side graph from disk) and then re-fetches all nodes. After the in-memory maps are replaced, all registered subscription callbacks fire.

### Graph Construction

The Rust backend builds the graph using a two-pass algorithm in `artifact_graph.rs`:

Pass 1: Walk every `.md` file under `.orqa/`. For each file that has a YAML `id` frontmatter field, create an `ArtifactNode` with `references_out` populated from well-known frontmatter fields.

**Single-value reference fields:** `milestone`, `epic`, `promoted-to`, `promoted-to`, `supersedes`, `superseded-by`, `surpassed-by`, `promoted-from`

**Array reference fields:** `depends-on`, `blocks`, `pillars`, `research-refs`

Pass 2: Invert every `references_out` entry into a `references_in` backlink on the target node.

Files without an `id` field are silently skipped — these are documentation pages, not typed governance artifacts. `README.md` files are also skipped regardless of content.

Artifact type is inferred from the directory path segment (e.g. a file under `/epics/` becomes `artifact_type: "epic"`). Files that do not match any known directory pattern become `artifact_type: "doc"`.

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | The SDK provides a single typed interface to all artifact metadata and relationships, replacing scattered ad-hoc patterns (hardcoded prefix maps, manual IPC calls, frontend caches). Every frontmatter link is resolved against the graph and surfaced as either a navigable link or a broken-link indicator, making the structure of the artifact system visible and inspectable. |
| Learning Through Reflection | The `brokenRefs()` and `orphans()` methods surface structural integrity issues in the artifact graph — broken links indicate deleted or missing artifacts, orphans indicate artifacts disconnected from the traceability chain. The `GraphStats` type exposes these counts for display, enabling continuous monitoring of governance health. |
