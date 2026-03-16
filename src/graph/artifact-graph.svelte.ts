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
 *
 * Graph visualization, topology analysis, and layout are provided by
 * @orqastudio/graph-visualiser — this SDK is data-only.
 */

import { SvelteMap, SvelteSet } from "svelte/reactivity";
import { listen } from "@tauri-apps/api/event";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import type { ArtifactNode, ArtifactRef, GraphStats, IntegrityCheck, AppliedFix, HealthSnapshot, ProposedTransition } from "@orqastudio/types";
import { ARTIFACT_TYPES, INVERSE_MAP } from "@orqastudio/types";

// ---------------------------------------------------------------------------
// Subscription callback types
// ---------------------------------------------------------------------------

type NodeCallback = (node: ArtifactNode) => void;
type TypeCallback = (nodes: ArtifactNode[]) => void;
type RefreshCallback = () => void;

/** Configuration for SDK initialization. */
export interface ArtifactGraphConfig {
    /** Project root path — used to start the file watcher. */
    projectPath: string;
    /** Whether to start the .orqa/ file watcher for auto-refresh. Default: true. */
    watchFiles?: boolean;
    /** Maximum health snapshots to retain when fetching trends. Default: 30. */
    snapshotLimit?: number;
}

// ---------------------------------------------------------------------------
// SDK class
// ---------------------------------------------------------------------------

export class ArtifactGraphSDK {
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

    /**
     * Pending status transitions proposed by the backend engine.
     *
     * Populated whenever the backend emits `"status-transitions-available"`.
     * These are transitions where `auto_apply: false` — they require human
     * confirmation before being applied.
     */
    pendingTransitions = $state<ProposedTransition[]>([]);

    // -----------------------------------------------------------------------
    // Private
    // -----------------------------------------------------------------------

    private nodeSubscribers = new Map<string, NodeCallback[]>();
    private typeSubscribers = new Map<string, TypeCallback[]>();
    private refreshCallbacks: RefreshCallback[] = [];
    private unlistenFn: UnlistenFn | null = null;
    private unlistenTransitionsFn: UnlistenFn | null = null;
    private _initCalled = false;
    private config: ArtifactGraphConfig | null = null;

    // -----------------------------------------------------------------------
    // Lifecycle
    // -----------------------------------------------------------------------

    async initialize(config: ArtifactGraphConfig): Promise<void> {
        if (this._initCalled) return;
        this._initCalled = true;
        this.config = config;

        if (config.watchFiles !== false) {
            await invoke<void>("artifact_watch_start", { projectPath: config.projectPath }).catch((err: unknown) => {
                console.warn("[artifact-graph-sdk] watcher failed to start:", err);
            });
        }

        await this._loadAll();
        if (!this.unlistenFn) {
            this.unlistenFn = await listen("artifact-graph-updated", () => {
                void this.refresh();
            });
        }
        if (!this.unlistenTransitionsFn) {
            this.unlistenTransitionsFn = await listen<ProposedTransition[]>(
                "status-transitions-available",
                (event) => {
                    this.pendingTransitions = event.payload;
                },
            );
        }
    }

    clearPendingTransitions(): void {
        this.pendingTransitions = [];
    }

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

    /**
     * Register a callback that fires after every graph refresh.
     * Returns an unlisten function.
     *
     * Used by @orqastudio/graph-visualiser to update its Cytoscape instance.
     */
    onRefresh(callback: RefreshCallback): () => void {
        this.refreshCallbacks.push(callback);
        return () => {
            this.refreshCallbacks = this.refreshCallbacks.filter((cb) => cb !== callback);
        };
    }

    // -----------------------------------------------------------------------
    // Resolution — synchronous in-memory lookups
    // -----------------------------------------------------------------------

    resolve(id: string): ArtifactNode | undefined {
        return this.graph.get(id);
    }

    resolveByPath(path: string): ArtifactNode | undefined {
        const id = this.pathIndex.get(path);
        if (!id) return undefined;
        return this.graph.get(id);
    }

    // -----------------------------------------------------------------------
    // Relationship queries — synchronous
    // -----------------------------------------------------------------------

    referencesFrom(id: string): ArtifactRef[] {
        return this.graph.get(id)?.references_out ?? [];
    }

    referencesTo(id: string): ArtifactRef[] {
        return this.graph.get(id)?.references_in ?? [];
    }

    // -----------------------------------------------------------------------
    // Bulk queries — synchronous
    // -----------------------------------------------------------------------

    byType(type: string): ArtifactNode[] {
        const result: ArtifactNode[] = [];
        for (const node of this.graph.values()) {
            if (node.artifact_type === type) result.push(node);
        }
        return result;
    }

    byStatus(status: string): ArtifactNode[] {
        const result: ArtifactNode[] = [];
        for (const node of this.graph.values()) {
            if (node.status === status) result.push(node);
        }
        return result;
    }

    // -----------------------------------------------------------------------
    // Content — async disk read/write
    // -----------------------------------------------------------------------

    async readContent(path: string): Promise<string> {
        return invoke<string>("read_artifact_content", { path });
    }

    async updateField(path: string, field: string, value: string): Promise<void> {
        await invoke<void>("update_artifact_field", { path, field, value });
        await this.refresh();
    }

    // -----------------------------------------------------------------------
    // Graph health — synchronous
    // -----------------------------------------------------------------------

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

    relationshipsFrom(id: string): { target: ArtifactNode; type: string; rationale?: string }[] {
        const node = this.graph.get(id);
        if (!node) return [];
        const result: { target: ArtifactNode; type: string; rationale?: string }[] = [];

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

    pipelineChain(id: string): { upstream: ArtifactNode[]; downstream: ArtifactNode[] } {
        const upstream: ArtifactNode[] = [];
        const downstream: ArtifactNode[] = [];
        const visited = new SvelteSet<string>();

        const upstreamTypes = ["grounded-by", "informed-by", "evolves-from", "observed-by"];
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

        visited.clear();
        visited.add(id);
        const downstreamTypes = ["enforces", "delivers", "driven-by"];
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

    missingInverses(): { ref: ArtifactRef; expectedInverse: string }[] {
        const result: { ref: ArtifactRef; expectedInverse: string }[] = [];
        for (const node of this.graph.values()) {
            for (const ref of node.references_out) {
                if (!ref.relationship_type) continue;
                const expectedInverse = INVERSE_MAP.get(ref.relationship_type);
                if (!expectedInverse) continue;

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

    async runIntegrityScan(): Promise<IntegrityCheck[]> {
        return invoke<IntegrityCheck[]>("run_integrity_scan");
    }

    async applyAutoFixes(checks: IntegrityCheck[]): Promise<AppliedFix[]> {
        return invoke<AppliedFix[]>("apply_auto_fixes", { checks });
    }

    // -----------------------------------------------------------------------
    // Health snapshots — async (requires backend storage)
    // -----------------------------------------------------------------------

    async storeHealthSnapshot(errorCount: number, warningCount: number): Promise<HealthSnapshot> {
        return invoke<HealthSnapshot>("store_health_snapshot", {
            errorCount,
            warningCount,
        });
    }

    async getHealthSnapshots(limit?: number): Promise<HealthSnapshot[]> {
        const effectiveLimit = limit ?? this.config?.snapshotLimit ?? 30;
        return invoke<HealthSnapshot[]>("get_health_snapshots", { limit: effectiveLimit });
    }

    // -----------------------------------------------------------------------
    // Subscriptions
    // -----------------------------------------------------------------------

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

    private async _fetchAll(): Promise<void> {
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

        // Notify refresh listeners (e.g. graph-visualiser)
        for (const cb of this.refreshCallbacks) {
            try { cb(); } catch { /* ignore listener errors */ }
        }
    }

    private _notifySubscribers(newGraph: Map<string, ArtifactNode>): void {
        for (const [id, callbacks] of this.nodeSubscribers) {
            const node = newGraph.get(id);
            if (node) {
                for (const cb of callbacks) cb(node);
            }
        }

        for (const [type, callbacks] of this.typeSubscribers) {
            const nodes: ArtifactNode[] = [];
            for (const node of newGraph.values()) {
                if (node.artifact_type === type) nodes.push(node);
            }
            for (const cb of callbacks) cb(nodes);
        }
    }
}
