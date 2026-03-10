---
title: Artifact Graph SDK
category: development
tags: [artifact-graph, sdk, typescript, svelte5, ipc, plugins]
created: 2026-03-10
updated: 2026-03-10
description: Development guide for the Artifact Graph SDK — the typed frontend interface to the bidirectional artifact node graph.
---

# Artifact Graph SDK

The Artifact Graph SDK is a Svelte 5 rune-based module that maintains an in-memory copy of the artifact graph built by the Rust backend. It is the single source of truth for artifact metadata and relationships in the frontend.

After `initialize()` is called, all resolution and query methods operate synchronously on the cached graph — no IPC round-trips are needed for lookups. Only `readContent()` goes to disk, because raw file content is never cached.

The SDK lives at `ui/lib/sdk/artifact-graph.svelte.ts` and is exported as a singleton called `artifactGraphSDK`. Every component, store, or plugin that needs to resolve an artifact ID, traverse relationships, or receive live updates uses this singleton.

## Architecture

The data pipeline from files on disk to components in the UI:

```text
.orqa/ markdown files
        |
        v
  Rust scanner (artifact_graph.rs)
  - Pass 1: walk all .md files, collect nodes + references_out
  - Pass 2: invert references_out into references_in (backlinks)
        |
        v
  AppState.artifact_graph (Mutex<Option<ArtifactGraph>>)
  - Lazy-init on first command call
  - Invalidated by file watcher when .orqa/ changes
        |
        v
  Tauri commands (graph_commands.rs)
  - resolve_artifact, resolve_artifact_path
  - get_references_from, get_references_to
  - get_artifacts_by_type
  - read_artifact_content
  - get_graph_stats
  - refresh_artifact_graph
        |
        v
  ArtifactGraphSDK (artifact-graph.svelte.ts)
  - In-memory SvelteMap: id → ArtifactNode
  - Path index: path → id
  - Reactive state: $state runes
  - Auto-refresh via "artifact-graph-updated" Tauri event
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

### Initialization

The SDK is initialized once in `AppLayout.svelte` when an active project is detected:

```typescript
// AppLayout.svelte
$effect(() => {
    const project = projectStore.activeProject;
    if (!project || needsSetup) return;
    void artifactGraphSDK.initialize();
});
```

`initialize()` is idempotent — calling it again when the graph is already loaded is a no-op, unless the SDK is in an error state. It fetches all artifact nodes by type in parallel, builds the in-memory maps, and registers a listener for `"artifact-graph-updated"` events.

### Importing

The SDK is a singleton. Import it directly from the module:

```typescript
import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
```

Do not instantiate `ArtifactGraphSDK` directly. The singleton is the only instance in the application.

## API Reference

### Types

The types are defined in `ui/lib/types/artifact-graph.ts` and mirror the Rust structs in `src-tauri/src/domain/artifact_graph.rs`. Field names use `snake_case` to match the Rust serde serialization output.

```typescript
interface ArtifactNode {
    /** Frontmatter `id` field (e.g. "EPIC-048"). */
    id: string;
    /** Relative path from the project root (e.g. ".orqa/planning/epics/EPIC-048.md"). */
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
| `graph` | `SvelteMap<string, ArtifactNode>` | In-memory node store, keyed by artifact ID |
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

Resolve an artifact node by its frontmatter ID (e.g. `"EPIC-048"`). Returns `undefined` when no artifact with the given ID exists in the graph.

```typescript
resolveByPath(path: string): ArtifactNode | undefined
```

Resolve an artifact node by its relative file path (e.g. `".orqa/planning/epics/EPIC-048.md"`). Returns `undefined` when no artifact at that path has been indexed.

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

Read the raw markdown content of an artifact file from disk. This is always an async IPC call — the SDK does not cache file content. Pass the relative path from the project root (e.g. `".orqa/planning/epics/EPIC-048.md"`).

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

## Migration Guide

### From ARTIFACT_PREFIX_MAP

The old approach used a hardcoded map from artifact ID prefix to directory path:

```typescript
// OLD — fragile, misses types without prefix entries
const ARTIFACT_PREFIX_MAP: Record<string, string> = {
    "EPIC": ".orqa/planning/epics",
    "TASK": ".orqa/planning/tasks",
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

**Single-value reference fields:** `milestone`, `epic`, `promoted-to`, `promoted_to`, `supersedes`, `superseded-by`, `surpassed-by`, `promoted_from`

**Array reference fields:** `depends-on`, `blocks`, `pillars`, `research-refs`

Pass 2: Invert every `references_out` entry into a `references_in` backlink on the target node.

Files without an `id` field are silently skipped — these are documentation pages, not typed governance artifacts. `README.md` files are also skipped regardless of content.

Artifact type is inferred from the directory path segment (e.g. a file under `/epics/` becomes `artifact_type: "epic"`). Files that do not match any known directory pattern become `artifact_type: "doc"`.

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | The SDK provides a single typed interface to all artifact metadata and relationships, replacing scattered ad-hoc patterns (hardcoded prefix maps, manual IPC calls, frontend caches). Every frontmatter link is resolved against the graph and surfaced as either a navigable link or a broken-link indicator, making the structure of the artifact system visible and inspectable. |
| Learning Through Reflection | The `brokenRefs()` and `orphans()` methods surface structural integrity issues in the artifact graph — broken links indicate deleted or missing artifacts, orphans indicate artifacts disconnected from the traceability chain. The `GraphStats` type exposes these counts for display, enabling continuous monitoring of governance health. |
