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

import { SvelteMap, SvelteSet } from "svelte/reactivity";
import { listen } from "@tauri-apps/api/event";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import type { ArtifactNode, ArtifactRef, GraphStats, IntegrityCheck, AppliedFix, HealthSnapshot, ProposedTransition } from "@orqastudio/types";
import { ARTIFACT_TYPES, INVERSE_MAP } from "@orqastudio/types";
import cytoscape from "cytoscape";

// ---------------------------------------------------------------------------
// Visualization color map
// ---------------------------------------------------------------------------

/** Hex color per artifact type — used by visualization components. */
export const ARTIFACT_TYPE_COLORS: Record<string, string> = {
    epic: "#3b82f6",
    task: "#10b981",
    milestone: "#f59e0b",
    idea: "#a855f7",
    decision: "#ec4899",
    research: "#06b6d4",
    lesson: "#f97316",
    rule: "#ef4444",
    agent: "#8b5cf6",
    skill: "#14b8a6",
    hook: "#6366f1",
    pillar: "#d97706",
    doc: "#9ca3af",
};

/** Cached node position — shared between SDK and visualization components. */
export interface NodePosition {
    id: string;
    x: number;
    y: number;
}

// ---------------------------------------------------------------------------
// Analysis types
// ---------------------------------------------------------------------------

/** Structural health metrics derived from the artifact graph topology. */
export interface GraphHealth {
    /** Number of disconnected subgraphs (weakly connected components). */
    componentCount: number;
    /** Nodes with zero in-degree (nothing points to them). */
    orphanCount: number;
    /** orphanCount / totalNodes * 100, rounded to 1 decimal place. */
    orphanPercentage: number;
    /** Average number of connections (in + out) per node. */
    avgDegree: number;
    /** Largest component size / totalNodes. 1.0 means a fully connected graph. */
    largestComponentRatio: number;
    /** Total number of nodes in the graph. */
    totalNodes: number;
    /** Total number of directed edges in the graph. */
    totalEdges: number;
}

/** A high-PageRank artifact that many others reference or depend upon. */
export interface BackboneArtifact {
    /** Artifact ID (e.g. "RULE-006"). */
    id: string;
    /** Display title. */
    title: string;
    /** Artifact type string (e.g. "rule", "epic"). */
    type: string;
    /** Normalised PageRank score (0–1). */
    rank: number;
}

/** Knowledge gaps detected from the relationship graph. */
export interface KnowledgeGaps {
    /** Rule IDs that have no `grounded-by` edge pointing to any PILLAR artifact. */
    ungroundedRules: string[];
    /** Skill IDs that have no `grounded-by` edge from any AGENT artifact. */
    unusedSkills: string[];
    /** Decision IDs that carry no outgoing `governs` or `drives` edge. */
    unenforcedDecisions: string[];
}

// ---------------------------------------------------------------------------
// Subscription callback types
// ---------------------------------------------------------------------------

type NodeCallback = (node: ArtifactNode) => void;
type TypeCallback = (nodes: ArtifactNode[]) => void;

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

    /**
     * Pending status transitions proposed by the backend engine.
     *
     * Populated whenever the backend emits `"status-transitions-available"`.
     * These are transitions where `auto_apply: false` — they require human
     * confirmation before being applied. Auto-apply transitions are written
     * to disk by the backend and never appear here.
     *
     * Consumers (e.g. DecisionQueueWidget) can read this to surface actionable
     * items. Call `clearPendingTransitions()` after the user has acted.
     */
    pendingTransitions = $state<ProposedTransition[]>([]);

    /**
     * Cached node positions from the last completed full-graph layout.
     * Visualization components write positions here after layout completes so
     * subsequent renders can use `preset` layout instead of re-running cose-bilkent.
     *
     * Cleared automatically when the graph refreshes (new nodes invalidate old positions).
     */
    cachedPositions = $state<NodePosition[]>([]);

    // -----------------------------------------------------------------------
    // Private subscription registries
    // -----------------------------------------------------------------------

    /** Per-ID subscribers: id → list of callbacks. */
    private nodeSubscribers = new Map<string, NodeCallback[]>();

    /** Per-type subscribers: artifact_type → list of callbacks. */
    private typeSubscribers = new Map<string, TypeCallback[]>();

    /** Tauri event unlisten function for `artifact-graph-updated`, set after initialize(). */
    private unlistenFn: UnlistenFn | null = null;

    /** Tauri event unlisten function for `status-transitions-available`, set after initialize(). */
    private unlistenTransitionsFn: UnlistenFn | null = null;

    /** Non-reactive flag to prevent $effect re-triggering on error. */
    private _initCalled = false;

    // -----------------------------------------------------------------------
    // Analysis — headless Cytoscape cache
    // -----------------------------------------------------------------------

    /**
     * Cached headless Cytoscape instance. Rebuilt whenever `graph` changes.
     * Access via `_getCy()` — never directly.
     */
    private _cy: cytoscape.Core | null = null;

    /**
     * Version counter incremented every time the graph map is replaced.
     * Allows `$derived.by()` computations to detect graph changes without
     * depending on the full SvelteMap internals.
     */
    private _graphVersion = $state(0);

    // -----------------------------------------------------------------------
    // Analysis — reactive derived properties
    // -----------------------------------------------------------------------

    /**
     * Structural health metrics for the artifact graph.
     * Recomputed automatically whenever the graph refreshes.
     */
    graphHealth = $derived.by((): GraphHealth => {
        const _v = this._graphVersion;
        void _v;
        const cy = this._getCy();
        const totalNodes = cy.nodes().length;
        const totalEdges = cy.edges().length;

        if (totalNodes === 0) {
            return { componentCount: 0, orphanCount: 0, orphanPercentage: 0, avgDegree: 0, largestComponentRatio: 0, totalNodes: 0, totalEdges: 0 };
        }

        const components = cy.elements().components();
        const componentCount = components.length;
        const largestComponentSize = Math.max(...components.map((c) => c.nodes().length));
        const largestComponentRatio = largestComponentSize / totalNodes;

        const orphanCount = cy.nodes().filter((n) => n.indegree(false) === 0).length;
        const orphanPercentage = Math.round((orphanCount / totalNodes) * 1000) / 10;

        const avgDegree = Math.round(((totalEdges * 2) / totalNodes) * 10) / 10;

        return { componentCount, orphanCount, orphanPercentage, avgDegree, largestComponentRatio, totalNodes, totalEdges };
    });

    /**
     * Top 10 artifacts by PageRank — the most structurally central nodes.
     * Recomputed automatically whenever the graph refreshes.
     */
    backboneArtifacts = $derived.by((): BackboneArtifact[] => {
        const _v = this._graphVersion;
        void _v;
        const cy = this._getCy();
        if (cy.nodes().length === 0) return [];

        const pr = cy.elements().pageRank({});
        const scored = cy.nodes().map((n) => ({
            id: n.id(),
            rank: pr.rank(n),
        }));

        scored.sort((a, b) => b.rank - a.rank);

        return scored.slice(0, 10).map(({ id, rank }) => {
            const node = this.graph.get(id);
            return {
                id,
                title: node?.title ?? id,
                type: node?.artifact_type ?? "unknown",
                rank,
            };
        });
    });

    /**
     * Knowledge gaps: rules without pillar grounding, skills not used by
     * any agent, and decisions that enforce nothing.
     * Recomputed automatically whenever the graph refreshes.
     */
    knowledgeGaps = $derived.by((): KnowledgeGaps => {
        const _v = this._graphVersion;
        void _v;

        const ungroundedRules: string[] = [];
        const unusedSkills: string[] = [];
        const unenforcedDecisions: string[] = [];

        for (const node of this.graph.values()) {
            if (node.artifact_type === "rule") {
                const hasGrounding = node.references_out.some(
                    (r) => r.relationship_type === "grounded-by" && this.graph.get(r.target_id)?.artifact_type === "pillar"
                );
                if (!hasGrounding) ungroundedRules.push(node.id);
            } else if (node.artifact_type === "skill") {
                const hasPractitioner = node.references_in.some(
                    (r) => r.relationship_type === "grounded-by" && this.graph.get(r.source_id)?.artifact_type === "agent"
                );
                if (!hasPractitioner) unusedSkills.push(node.id);
            } else if (node.artifact_type === "decision") {
                const hasGovernsOrDrives = node.references_out.some(
                    (r) => r.relationship_type === "governs" || r.relationship_type === "drives"
                );
                if (!hasGovernsOrDrives) unenforcedDecisions.push(node.id);
            }
        }

        return { ungroundedRules, unusedSkills, unenforcedDecisions };
    });

    /**
     * Cytoscape element definitions ready for visualization rendering.
     *
     * Includes:
     * - Nodes with `color` (resolved from type), `label`, and `tooltip`.
     * - Edges deduplicated by source→target pair (multiple relationship types
     *   between the same pair are collapsed to one visual edge).
     *
     * Recomputed whenever the graph refreshes. Visualization components pass
     * this directly to their DOM-attached cytoscape instance.
     */
    graphElements = $derived.by((): cytoscape.ElementDefinition[] => {
        void this._graphVersion;
        return this._buildVisualizationElements();
    });

    // -----------------------------------------------------------------------
    // Lifecycle
    // -----------------------------------------------------------------------

    /** Stored config from initialization. */
    private config: ArtifactGraphConfig | null = null;

    /**
     * Initialize the SDK: start the file watcher, fetch the full graph from
     * the backend, and register for auto-refresh on backend
     * `"artifact-graph-updated"` events.
     *
     * Safe to call multiple times — subsequent calls are no-ops if already
     * initialized. Uses a non-reactive flag to avoid $effect dependency
     * tracking on reactive state (which would cause infinite retry loops).
     */
    async initialize(config: ArtifactGraphConfig): Promise<void> {
        if (this._initCalled) return;
        this._initCalled = true;
        this.config = config;

        // Start the .orqa/ file watcher so the graph auto-refreshes on disk changes.
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
                    console.log(
                        "[artifact-graph-sdk] pending transitions received:",
                        event.payload.length,
                        event.payload.map((t) => `${t.artifact_id}: ${t.current_status} → ${t.proposed_status}`),
                    );
                    this.pendingTransitions = event.payload;
                },
            );
        }
    }

    /**
     * Clear the list of pending status transitions.
     *
     * Call this after the user has reviewed and acted on the transitions,
     * or when you want to dismiss the notification.
     */
    clearPendingTransitions(): void {
        this.pendingTransitions = [];
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
    // Content — async disk read/write
    // -----------------------------------------------------------------------

    /**
     * Read the raw markdown content of an artifact file from disk.
     * Always fetches from the backend (no local caching here).
     */
    async readContent(path: string): Promise<string> {
        return invoke<string>("read_artifact_content", { path });
    }

    /**
     * Update a single YAML frontmatter field in an artifact file on disk,
     * then refresh the in-memory graph.
     *
     * @param path  Relative file path from the project root.
     * @param field Frontmatter key to update (must already exist in the file).
     * @param value New string value for the field.
     */
    async updateField(path: string, field: string, value: string): Promise<void> {
        await invoke<void>("update_artifact_field", { path, field, value });
        await this.refresh();
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
        const visited = new SvelteSet<string>();

        // Upstream: follow grounded-by, informed-by, evolves-from, observed-by
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

        // Downstream: follow enforces, delivers, driven-by
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

    /**
     * Find relationship edges where A→B exists but the expected inverse B→A is missing.
     *
     * Uses the canonical INVERSE_MAP from @orqastudio/types as the source of truth.
     */
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
    // Analysis — graph traversal methods
    // -----------------------------------------------------------------------

    /**
     * BFS trace from an artifact following the graph topology.
     *
     * - `'up'`   — follow edges TO the node (who references this?).
     *              Traces toward pillars and milestones.
     * - `'down'` — follow edges FROM the node (what does this reference?).
     *              Traces toward tasks and implementations.
     *
     * Returns an ordered array of artifact IDs visited during BFS,
     * not including the starting node itself.
     */
    traceChain(id: string, direction: "up" | "down"): string[] {
        const cy = this._getCy();
        const startNode = cy.getElementById(id);
        if (startNode.empty()) return [];

        const visited: string[] = [];
        const seen = new SvelteSet<string>([id]);
        const queue: string[] = [id];

        while (queue.length > 0) {
            const current = queue.shift()!;
            const cyNode = cy.getElementById(current);

            const neighbours =
                direction === "up"
                    ? cyNode.incomers("node")   // edges TO current → sources
                    : cyNode.outgoers("node");  // edges FROM current → targets

            neighbours.forEach((n) => {
                const nid = n.id();
                if (!seen.has(nid)) {
                    seen.add(nid);
                    visited.push(nid);
                    queue.push(nid);
                }
            });
        }

        return visited;
    }

    /**
     * Return all artifacts within `maxDepth` hops from the given node,
     * grouped by relationship type.
     *
     * @param id       Starting artifact ID.
     * @param maxDepth Maximum BFS depth (default 2).
     */
    impactOf(id: string, maxDepth = 2): { total: number; artifacts: Array<{ id: string; type: string; distance: number }> } {
        const cy = this._getCy();
        const startNode = cy.getElementById(id);
        if (startNode.empty()) return { total: 0, artifacts: [] };

        const results: Array<{ id: string; type: string; distance: number }> = [];
        const seen = new SvelteSet<string>([id]);

        // BFS level by level so we track distance accurately.
        let frontier: string[] = [id];
        for (let depth = 1; depth <= maxDepth; depth++) {
            const nextFrontier: string[] = [];
            for (const current of frontier) {
                const cyNode = cy.getElementById(current);
                // Visit both successors and predecessors for impact analysis.
                cyNode.neighborhood("node").forEach((n) => {
                    const nid = n.id();
                    if (!seen.has(nid)) {
                        seen.add(nid);
                        const node = this.graph.get(nid);
                        results.push({ id: nid, type: node?.artifact_type ?? "unknown", distance: depth });
                        nextFrontier.push(nid);
                    }
                });
            }
            frontier = nextFrontier;
        }

        return { total: results.length, artifacts: results };
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
    // Health snapshots — async (requires backend storage)
    // -----------------------------------------------------------------------

    /** Store a health snapshot with the current graph metrics. */
    async storeHealthSnapshot(errorCount: number, warningCount: number): Promise<HealthSnapshot> {
        return invoke<HealthSnapshot>("store_health_snapshot", {
            errorCount,
            warningCount,
        });
    }

    /** Get the most recent health snapshots for trend display. */
    async getHealthSnapshots(limit?: number): Promise<HealthSnapshot[]> {
        const effectiveLimit = limit ?? this.config?.snapshotLimit ?? 30;
        return invoke<HealthSnapshot[]>("get_health_snapshots", { limit: effectiveLimit });
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

    /**
     * Return the cached headless Cytoscape instance, building it from the
     * current graph data if it doesn't exist yet. The instance is rebuilt
     * whenever `_fetchAll` replaces the graph map (via `_graphVersion`).
     */
    private _getCy(): cytoscape.Core {
        if (this._cy) return this._cy;

        const elements: cytoscape.ElementDefinition[] = [];

        for (const node of this.graph.values()) {
            elements.push({
                group: "nodes",
                data: { id: node.id, type: node.artifact_type, status: node.status },
            });
            for (const ref of node.references_out) {
                if (this.graph.has(ref.target_id)) {
                    elements.push({
                        group: "edges",
                        data: {
                            id: `${ref.source_id}→${ref.target_id}→${ref.relationship_type ?? ref.field}`,
                            source: ref.source_id,
                            target: ref.target_id,
                            type: ref.relationship_type ?? ref.field,
                        },
                    });
                }
            }
        }

        this._cy = cytoscape({ elements, headless: true });
        return this._cy;
    }

    /** Invalidate the cached Cytoscape instance so it is rebuilt on next access. */
    private _invalidateCy(): void {
        if (this._cy) {
            this._cy.destroy();
            this._cy = null;
        }
    }

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

        // Invalidate the headless Cytoscape cache so analysis properties
        // are recomputed against the new graph on next access.
        this._invalidateCy();
        this._graphVersion++;

        // Clear cached positions — new nodes may have appeared and the old
        // layout coordinates are no longer valid.
        this.cachedPositions = [];

        this._notifySubscribers(newGraph);
    }

    /**
     * Build Cytoscape element definitions for visualization.
     *
     * Nodes include `color` (type-based), `label`, and `tooltip`.
     * Edges are deduplicated by source→target pair — only one visual edge per pair,
     * even when multiple relationship types connect the same two nodes.
     */
    private _buildVisualizationElements(): cytoscape.ElementDefinition[] {
        const elements: cytoscape.ElementDefinition[] = [];
        const edgeKeys = new SvelteSet<string>();

        for (const node of this.graph.values()) {
            const color = ARTIFACT_TYPE_COLORS[node.artifact_type] ?? "#6b7280";

            elements.push({
                group: "nodes",
                data: {
                    id: node.id,
                    label: node.id,
                    color,
                    tooltip: `${node.title}\n${node.artifact_type}${node.status ? ` · ${node.status}` : ""}`,
                },
            });
        }

        for (const node of this.graph.values()) {
            for (const ref of node.references_out) {
                if (!this.graph.has(ref.target_id)) continue;
                const key = `${ref.source_id}->${ref.target_id}`;
                if (edgeKeys.has(key)) continue;
                edgeKeys.add(key);
                elements.push({
                    group: "edges",
                    data: {
                        id: key,
                        source: ref.source_id,
                        target: ref.target_id,
                    },
                });
            }
        }

        return elements;
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
