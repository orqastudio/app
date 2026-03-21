/**
 * Artifact Graph SDK
 *
 * A Svelte 5 rune-based SDK that maintains an in-memory copy of the artifact
 * graph built by the Rust backend. After `initialize()` is called, all
 * resolution and query methods operate synchronously on the cached data —
 * no IPC round-trips needed for lookups.
 *
 * The SDK receives a PluginRegistry reference at construction time and uses
 * it as the single authority for relationship resolution, inverse maps,
 * semantic queries, and type constraints. No hardcoded relationship keys
 * or artifact types anywhere in this module.
 */

import { SvelteMap, SvelteSet } from "svelte/reactivity";
import { listen } from "@tauri-apps/api/event";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import { logger } from "../logger.js";
import type {
	ArtifactNode,
	ArtifactRef,
	GraphStats,
	IntegrityCheck,
	AppliedFix,
	HealthSnapshot,
	GraphHealthData,
	ProposedTransition,
	RelationshipType,
	PlatformArtifactType,
	TraceabilityResult,
} from "@orqastudio/types";
import { PLATFORM_CONFIG } from "@orqastudio/types";
import type { PluginRegistry } from "../plugins/plugin-registry.svelte.js";

/** Options for project-filtered queries. */
export interface QueryOptions {
	/** Filter to a specific child project. Omit to include all projects. */
	project?: string;
}

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

const log = logger("graph");

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

	/** Pending status transitions proposed by the backend engine. */
	pendingTransitions = $state<ProposedTransition[]>([]);

	// -----------------------------------------------------------------------
	// Private
	// -----------------------------------------------------------------------

	private registry: PluginRegistry;
	private nodeSubscribers = new Map<string, NodeCallback[]>();
	private typeSubscribers = new Map<string, TypeCallback[]>();
	private refreshCallbacks: RefreshCallback[] = [];
	private unlistenFn: UnlistenFn | null = null;
	private unlistenTransitionsFn: UnlistenFn | null = null;
	private _initCalled = false;
	private config: ArtifactGraphConfig | null = null;

	constructor(registry: PluginRegistry) {
		this.registry = registry;
	}

	// -----------------------------------------------------------------------
	// Schema access — all from the PluginRegistry
	// -----------------------------------------------------------------------

	/** All known artifact type keys (platform + plugin-registered). */
	private get allTypeKeys(): string[] {
		const platformKeys = PLATFORM_CONFIG.artifactTypes.map((t) => t.key);
		const pluginKeys = this.registry.allSchemas.map((s) => s.key);
		return [...new Set([...platformKeys, ...pluginKeys])];
	}

	/** Get the inverse of a relationship key, or undefined if unknown. */
	getInverse(key: string): string | undefined {
		const rel = this.registry.getRelationship(key);
		return rel?.inverse;
	}

	/** Get all relationship keys for a given semantic category. */
	keysForSemantic(semantic: string): string[] {
		const keys: string[] = [];
		for (const rel of this.registry.allRelationships) {
			if (rel.semantic === semantic) {
				keys.push(rel.key);
				if (rel.inverse !== rel.key) keys.push(rel.inverse);
			}
		}
		return keys;
	}

	/** Validate a relationship between two artifact types. Returns null if valid. */
	validateRelationship(key: string, fromType: string, toType: string): string | null {
		return this.registry.validateRelationship(key, fromType, toType);
	}

	// -----------------------------------------------------------------------
	// Lifecycle
	// -----------------------------------------------------------------------

	async initialize(config: ArtifactGraphConfig): Promise<void> {
		if (this._initCalled) return;
		this._initCalled = true;
		this.config = config;

		if (config.watchFiles !== false) {
			await invoke<void>("artifact_watch_start", { projectPath: config.projectPath }).catch((err: unknown) => {
				log.warn("watcher failed to start", err);
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

	onRefresh(callback: RefreshCallback): () => void {
		this.refreshCallbacks.push(callback);
		return () => {
			this.refreshCallbacks = this.refreshCallbacks.filter((cb) => cb !== callback);
		};
	}

	// -----------------------------------------------------------------------
	// Resolution — synchronous in-memory lookups
	// -----------------------------------------------------------------------

	resolve(id: string, options?: QueryOptions): ArtifactNode | undefined {
		const direct = this.graph.get(id);
		if (direct) return direct;

		if (options?.project) {
			const qualified = `${options.project}::${id}`;
			return this.graph.get(qualified);
		}

		for (const node of this.graph.values()) {
			if (node.id === id) return node;
		}
		return undefined;
	}

	resolveByPath(path: string): ArtifactNode | undefined {
		const id = this.pathIndex.get(path);
		if (!id) return undefined;
		return this.graph.get(id);
	}

	// -----------------------------------------------------------------------
	// Organisation-mode getters
	// -----------------------------------------------------------------------

	get projects(): string[] {
		const names = new Set<string>();
		for (const node of this.graph.values()) {
			if (node.project) names.add(node.project);
		}
		return [...names];
	}

	get isOrganisation(): boolean {
		return this.projects.length > 0;
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

	byType(type: string, options?: QueryOptions): ArtifactNode[] {
		const result: ArtifactNode[] = [];
		for (const node of this.graph.values()) {
			if (node.artifact_type === type) {
				if (options?.project && node.project !== options.project) continue;
				result.push(node);
			}
		}
		return result;
	}

	byStatus(status: string, options?: QueryOptions): ArtifactNode[] {
		const result: ArtifactNode[] = [];
		for (const node of this.graph.values()) {
			if (node.status === status) {
				if (options?.project && node.project !== options.project) continue;
				result.push(node);
			}
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

	/** Get all outgoing relationships from an artifact with resolved targets. */
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

	/**
	 * Walk upstream and downstream from an artifact following relationship semantics.
	 *
	 * Upstream: follows inverse keys from foundation, knowledge-flow, observation
	 * (things that feed INTO this artifact — pillars, research, lessons)
	 *
	 * Downstream: follows forward keys from governance, lineage
	 * (things this artifact drives — epics, rules, decisions)
	 *
	 * All relationship keys are derived from the registry — nothing hardcoded.
	 */
	pipelineChain(id: string): { upstream: ArtifactNode[]; downstream: ArtifactNode[] } {
		const upstream: ArtifactNode[] = [];
		const downstream: ArtifactNode[] = [];
		const visited = new SvelteSet<string>();

		// Build upstream/downstream key sets from all registered relationships
		const upstreamKeys = new Set<string>();
		const downstreamKeys = new Set<string>();

		for (const rel of this.registry.allRelationships) {
			const sem = rel.semantic;
			if (sem === "foundation" || sem === "knowledge-flow" || sem === "observation") {
				upstreamKeys.add(rel.inverse);
			}
			if (sem === "governance" || sem === "lineage") {
				downstreamKeys.add(rel.key);
			}
			// Plugin semantics: hierarchy and dependency are downstream
			if (sem === "hierarchy" || sem === "dependency") {
				downstreamKeys.add(rel.inverse); // "delivered-by" is downstream from the parent's perspective
			}
		}

		const walkUp = (currentId: string) => {
			if (visited.has(currentId)) return;
			visited.add(currentId);
			for (const type of upstreamKeys) {
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
		const walkDown = (currentId: string) => {
			if (visited.has(currentId) && currentId !== id) return;
			visited.add(currentId);
			for (const type of downstreamKeys) {
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

	/** Find relationships where the inverse edge is missing. Uses registry inverse map. */
	missingInverses(): { ref: ArtifactRef; expectedInverse: string }[] {
		const result: { ref: ArtifactRef; expectedInverse: string }[] = [];
		for (const node of this.graph.values()) {
			for (const ref of node.references_out) {
				if (!ref.relationship_type) continue;
				const expectedInverse = this.getInverse(ref.relationship_type);
				if (!expectedInverse) continue;

				const target = this.graph.get(ref.target_id);
				if (!target) continue;

				const hasInverse = target.references_out.some(
					(r) => r.relationship_type === expectedInverse && r.target_id === node.id,
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

	/** Fetch extended structural health metrics from the backend.
	 *
	 * Returns `GraphHealthData` computed by the Rust `compute_graph_health` function,
	 * including component count, orphan percentage, density, traceability, and
	 * bidirectionality. Use this instead of the client-side Cytoscape analysis.
	 */
	async getGraphHealth(): Promise<GraphHealthData> {
		return invoke<GraphHealthData>("get_graph_health");
	}

	/** Fetch full traceability data for an artifact.
	 *
	 * Returns ancestry chains to pillar/vision roots, all downstream descendants
	 * with BFS depth, sibling artifacts sharing a common parent, impact radius,
	 * and a disconnected flag when no path to any pillar exists.
	 */
	async getTraceability(id: string): Promise<TraceabilityResult> {
		return invoke<TraceabilityResult>("get_artifact_traceability", { id });
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
		// Use all known type keys from platform + registered plugins
		const typeKeys = this.allTypeKeys;

		const [statsResult, ...typedNodes] = await Promise.all([
			invoke<GraphStats>("get_graph_stats"),
			...typeKeys.map((t) =>
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

		for (const cb of this.refreshCallbacks) {
			try { cb(); } catch (err: unknown) { log.warn("subscriber callback failed", err); }
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
