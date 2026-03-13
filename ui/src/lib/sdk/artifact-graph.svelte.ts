/**
 * Artifact Graph SDK
 *
 * A Svelte 5 rune-based SDK that maintains an in-memory copy of the artifact
 * graph built by the Rust backend. After `initialize()` is called, all
 * resolution and query methods operate synchronously on the cached data —
 * no IPC round-trips needed for lookups.
 *
 * The SDK listens for the `"artifact-graph-updated"` Tauri event and
 * automatically refreshes its cache when the backend rebuilds the graph.
 */

import { SvelteMap } from "svelte/reactivity";
import { listen } from "@tauri-apps/api/event";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { invoke, extractErrorMessage } from "$lib/ipc/invoke";
import type { ArtifactNode, ArtifactRef, GraphStats, IntegrityCheck, AppliedFix } from "$lib/types/artifact-graph";
import { ARTIFACT_TYPES } from "$lib/types/artifact-graph";

// ---------------------------------------------------------------------------
// Subscription callback types
// ---------------------------------------------------------------------------

type NodeCallback = (node: ArtifactNode) => void;
type TypeCallback = (nodes: ArtifactNode[]) => void;

// ---------------------------------------------------------------------------
// SDK class
// ---------------------------------------------------------------------------

class ArtifactGraphSDK {
    // -----------------------------------------------------------------------
    // Reactive state
    // -----------------------------------------------------------------------

    /** In-memory node store keyed by artifact ID. */
    graph = $state<SvelteMap<string, ArtifactNode>>(new SvelteMap());

    /** Reverse-lookup index: relative file path → artifact ID. */
    pathIndex = $state<SvelteMap<string, string>>(new SvelteMap());

    /** Summary statistics from the last refresh. */
    stats = $state<GraphStats | null>(null);

    /** True while a refresh or initialization is in progress. */
    loading = $state(false);

    /** Timestamp of the last successful refresh. */
    lastRefresh = $state<Date | null>(null);

    /** Error message from the last failed operation, or null when healthy. */
    error = $state<string | null>(null);

    // -----------------------------------------------------------------------
    // Private subscription registries
    // -----------------------------------------------------------------------

    /** Per-ID subscribers: id → list of callbacks. */
    private nodeSubscribers = new Map<string, NodeCallback[]>();

    /** Per-type subscribers: artifact_type → list of callbacks. */
    private typeSubscribers = new Map<string, TypeCallback[]>();

    /** Tauri event unlisten function, set after initialize(). */
    private unlistenFn: UnlistenFn | null = null;

    /** Non-reactive flag to prevent $effect re-triggering on error. */
    private _initCalled = false;

    // -----------------------------------------------------------------------
    // Lifecycle
    // -----------------------------------------------------------------------

    /**
     * Initialize the SDK: fetch the full graph from the backend and register
     * for auto-refresh on backend `"artifact-graph-updated"` events.
     *
     * Safe to call multiple times — subsequent calls are no-ops if already
     * initialized. Uses a non-reactive flag to avoid $effect dependency
     * tracking on reactive state (which would cause infinite retry loops).
     */
    async initialize(): Promise<void> {
        if (this._initCalled) return;
        this._initCalled = true;
        await this._loadAll();
        if (!this.unlistenFn) {
            this.unlistenFn = await listen("artifact-graph-updated", () => {
                void this.refresh();
            });
        }
    }

    /**
     * Rebuild the backend graph from disk, then re-fetch all nodes into the
     * local cache. Updates `stats`, `graph`, `pathIndex`, and `lastRefresh`.
     */
    async refresh(): Promise<void> {
        if (this.loading) return;
        this.loading = true;
        this.error = null;
        try {
            await invoke<void>("refresh_artifact_graph");
            await this._fetchAll();
            this.lastRefresh = new Date();
        } catch (err: unknown) {
            this.error = extractErrorMessage(err);
        } finally {
            this.loading = false;
        }
    }

    // -----------------------------------------------------------------------
    // Resolution — synchronous in-memory lookups
    // -----------------------------------------------------------------------

    /** Resolve an artifact node by its ID (e.g. "EPIC-048"). */
    resolve(id: string): ArtifactNode | undefined {
        return this.graph.get(id);
    }

    /** Resolve an artifact node by its relative file path. */
    resolveByPath(path: string): ArtifactNode | undefined {
        const id = this.pathIndex.get(path);
        if (!id) return undefined;
        return this.graph.get(id);
    }

    // -----------------------------------------------------------------------
    // Relationship queries — synchronous
    // -----------------------------------------------------------------------

    /** Return all forward references (outgoing edges) from an artifact. */
    referencesFrom(id: string): ArtifactRef[] {
        return this.graph.get(id)?.references_out ?? [];
    }

    /** Return all backlinks (incoming edges) to an artifact. */
    referencesTo(id: string): ArtifactRef[] {
        return this.graph.get(id)?.references_in ?? [];
    }

    // -----------------------------------------------------------------------
    // Bulk queries — synchronous
    // -----------------------------------------------------------------------

    /** Return all nodes of a given artifact type (e.g. "epic", "task"). */
    byType(type: string): ArtifactNode[] {
        const result: ArtifactNode[] = [];
        for (const node of this.graph.values()) {
            if (node.artifact_type === type) result.push(node);
        }
        return result;
    }

    /** Return all nodes with the given status value. */
    byStatus(status: string): ArtifactNode[] {
        const result: ArtifactNode[] = [];
        for (const node of this.graph.values()) {
            if (node.status === status) result.push(node);
        }
        return result;
    }

    // -----------------------------------------------------------------------
    // Content — async disk read
    // -----------------------------------------------------------------------

    /**
     * Read the raw markdown content of an artifact file from disk.
     * Always fetches from the backend (no local caching here).
     */
    async readContent(path: string): Promise<string> {
        return invoke<string>("read_artifact_content", { path });
    }

    // -----------------------------------------------------------------------
    // Graph health — synchronous
    // -----------------------------------------------------------------------

    /** Return all references whose target ID does not exist in the graph. */
    brokenRefs(): ArtifactRef[] {
        const result: ArtifactRef[] = [];
        for (const node of this.graph.values()) {
            for (const ref of node.references_out) {
                if (!this.graph.has(ref.target_id)) {
                    result.push(ref);
                }
            }
        }
        return result;
    }

    /** Return all nodes that have no outgoing or incoming references. */
    orphans(): ArtifactNode[] {
        const result: ArtifactNode[] = [];
        for (const node of this.graph.values()) {
            if (node.references_out.length === 0 && node.references_in.length === 0) {
                result.push(node);
            }
        }
        return result;
    }

    // -----------------------------------------------------------------------
    // Relationship traversal — synchronous
    // -----------------------------------------------------------------------

    /** Follow outgoing edges of a specific relationship type from an artifact. */
    traverse(id: string, relationshipType: string): ArtifactNode[] {
        const node = this.graph.get(id);
        if (!node) return [];
        const result: ArtifactNode[] = [];
        for (const ref of node.references_out) {
            if (ref.relationship_type === relationshipType) {
                const target = this.graph.get(ref.target_id);
                if (target) result.push(target);
            }
        }
        return result;
    }

    /** Follow incoming edges of a specific relationship type to an artifact. */
    traverseIncoming(id: string, relationshipType: string): ArtifactNode[] {
        const node = this.graph.get(id);
        if (!node) return [];
        const result: ArtifactNode[] = [];
        for (const ref of node.references_in) {
            if (ref.relationship_type === relationshipType) {
                const source = this.graph.get(ref.source_id);
                if (source) result.push(source);
            }
        }
        return result;
    }

    /** Return enriched relationship data from an artifact's outgoing relationship edges. */
    relationshipsFrom(id: string): { target: ArtifactNode; type: string; rationale?: string }[] {
        const node = this.graph.get(id);
        if (!node) return [];
        const result: { target: ArtifactNode; type: string; rationale?: string }[] = [];

        // Get rationales from frontmatter relationships array
        const fmRelationships = (node.frontmatter as Record<string, unknown>)?.relationships;
        const rationales = new Map<string, string>();
        if (Array.isArray(fmRelationships)) {
            for (const rel of fmRelationships) {
                const r = rel as Record<string, unknown>;
                if (typeof r.target === "string" && typeof r.rationale === "string") {
                    rationales.set(`${r.target}:${r.type}`, r.rationale);
                }
            }
        }

        for (const ref of node.references_out) {
            if (ref.relationship_type) {
                const target = this.graph.get(ref.target_id);
                if (target) {
                    const rationale = rationales.get(`${ref.target_id}:${ref.relationship_type}`);
                    result.push({
                        target,
                        type: ref.relationship_type,
                        ...(rationale ? { rationale } : {}),
                    });
                }
            }
        }
        return result;
    }

    /** Walk the pipeline chain upstream and downstream from an artifact. */
    pipelineChain(id: string): { upstream: ArtifactNode[]; downstream: ArtifactNode[] } {
        const upstream: ArtifactNode[] = [];
        const downstream: ArtifactNode[] = [];
        const visited = new Set<string>();

        // Upstream: follow grounded/grounded-by, informed-by, observed-by
        const upstreamTypes = ["grounded", "informed-by", "observed-by"];
        const walkUp = (currentId: string) => {
            if (visited.has(currentId)) return;
            visited.add(currentId);
            for (const type of upstreamTypes) {
                for (const node of this.traverse(currentId, type)) {
                    if (!visited.has(node.id)) {
                        upstream.push(node);
                        walkUp(node.id);
                    }
                }
            }
        };
        walkUp(id);

        // Downstream: follow enforced-by, practiced-by, verified-by
        visited.clear();
        visited.add(id);
        const downstreamTypes = ["enforced-by", "practiced-by", "verified-by"];
        const walkDown = (currentId: string) => {
            if (visited.has(currentId) && currentId !== id) return;
            visited.add(currentId);
            for (const type of downstreamTypes) {
                for (const node of this.traverse(currentId, type)) {
                    if (!visited.has(node.id)) {
                        downstream.push(node);
                        walkDown(node.id);
                    }
                }
            }
        };
        walkDown(id);

        return { upstream, downstream };
    }

    /** Find relationship edges where A→B exists but the expected inverse B→A is missing. */
    missingInverses(): { ref: ArtifactRef; expectedInverse: string }[] {
        const INVERSE_MAP: Record<string, string> = {
            "observes": "observed-by",
            "observed-by": "observes",
            "grounded": "grounded-by",
            "grounded-by": "grounded",
            "practices": "practiced-by",
            "practiced-by": "practices",
            "enforces": "enforced-by",
            "enforced-by": "enforces",
            "verifies": "verified-by",
            "verified-by": "verifies",
            "informs": "informed-by",
            "informed-by": "informs",
        };

        const result: { ref: ArtifactRef; expectedInverse: string }[] = [];
        for (const node of this.graph.values()) {
            for (const ref of node.references_out) {
                if (!ref.relationship_type) continue;
                const expectedInverse = INVERSE_MAP[ref.relationship_type];
                if (!expectedInverse) continue;

                // Check if the target has the inverse relationship pointing back
                const target = this.graph.get(ref.target_id);
                if (!target) continue;

                const hasInverse = target.references_out.some(
                    (r) => r.relationship_type === expectedInverse && r.target_id === node.id
                );
                if (!hasInverse) {
                    result.push({ ref, expectedInverse });
                }
            }
        }
        return result;
    }

    // -----------------------------------------------------------------------
    // Integrity checks — async (requires backend scan)
    // -----------------------------------------------------------------------

    /** Run integrity checks via the backend and return all findings. */
    async runIntegrityScan(): Promise<IntegrityCheck[]> {
        return invoke<IntegrityCheck[]>("run_integrity_scan");
    }

    /** Apply auto-fixes for the given integrity checks and return what was changed. */
    async applyAutoFixes(checks: IntegrityCheck[]): Promise<AppliedFix[]> {
        return invoke<AppliedFix[]>("apply_auto_fixes", { checks });
    }

    // -----------------------------------------------------------------------
    // Subscriptions — plugin API
    // -----------------------------------------------------------------------

    /**
     * Subscribe to changes for a specific artifact by ID.
     * Returns an unlisten function — call it to cancel the subscription.
     */
    subscribe(id: string, callback: NodeCallback): () => void {
        const existing = this.nodeSubscribers.get(id) ?? [];
        existing.push(callback);
        this.nodeSubscribers.set(id, existing);
        return () => {
            const cbs = this.nodeSubscribers.get(id);
            if (!cbs) return;
            const filtered = cbs.filter((cb) => cb !== callback);
            if (filtered.length === 0) {
                this.nodeSubscribers.delete(id);
            } else {
                this.nodeSubscribers.set(id, filtered);
            }
        };
    }

    /**
     * Subscribe to changes for all artifacts of a given type.
     * Returns an unlisten function — call it to cancel the subscription.
     */
    subscribeType(type: string, callback: TypeCallback): () => void {
        const existing = this.typeSubscribers.get(type) ?? [];
        existing.push(callback);
        this.typeSubscribers.set(type, existing);
        return () => {
            const cbs = this.typeSubscribers.get(type);
            if (!cbs) return;
            const filtered = cbs.filter((cb) => cb !== callback);
            if (filtered.length === 0) {
                this.typeSubscribers.delete(type);
            } else {
                this.typeSubscribers.set(type, filtered);
            }
        };
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    /** First-load wrapper: sets loading/error state then delegates. */
    private async _loadAll(): Promise<void> {
        if (this.loading) return;
        this.loading = true;
        this.error = null;
        try {
            await invoke<void>("refresh_artifact_graph");
            await this._fetchAll();
            this.lastRefresh = new Date();
        } catch (err: unknown) {
            this.error = extractErrorMessage(err);
        } finally {
            this.loading = false;
        }
    }

    /**
     * Fetch all artifact nodes from the backend by requesting each known type
     * in parallel, then assemble the in-memory graph and path index.
     *
     * Also fetches graph stats.
     */
    private async _fetchAll(): Promise<void> {
        // Fetch all types in parallel to minimise latency.
        const [statsResult, ...typedNodes] = await Promise.all([
            invoke<GraphStats>("get_graph_stats"),
            ...ARTIFACT_TYPES.map((t) =>
                invoke<ArtifactNode[]>("get_artifacts_by_type", { artifactType: t }),
            ),
        ]);

        const newGraph = new SvelteMap<string, ArtifactNode>();
        const newPathIndex = new SvelteMap<string, string>();

        for (const nodes of typedNodes) {
            for (const node of nodes) {
                newGraph.set(node.id, node);
                newPathIndex.set(node.path, node.id);
            }
        }

        this.graph = newGraph;
        this.pathIndex = newPathIndex;
        this.stats = statsResult;

        this._notifySubscribers(newGraph);
    }

    /** Fire all registered subscriptions after a graph refresh. */
    private _notifySubscribers(newGraph: Map<string, ArtifactNode>): void {
        // Node-level subscribers.
        for (const [id, callbacks] of this.nodeSubscribers) {
            const node = newGraph.get(id);
            if (node) {
                for (const cb of callbacks) cb(node);
            }
        }

        // Type-level subscribers.
        for (const [type, callbacks] of this.typeSubscribers) {
            const nodes: ArtifactNode[] = [];
            for (const node of newGraph.values()) {
                if (node.artifact_type === type) nodes.push(node);
            }
            for (const cb of callbacks) cb(nodes);
        }
    }
}

// ---------------------------------------------------------------------------
// Singleton export
// ---------------------------------------------------------------------------

export const artifactGraphSDK = new ArtifactGraphSDK();
