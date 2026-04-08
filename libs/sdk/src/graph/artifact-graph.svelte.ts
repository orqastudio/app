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

import { SvelteMap, SvelteSet, SvelteDate } from "svelte/reactivity";
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
	TraceabilityResult,
} from "@orqastudio/types";
import { PLATFORM_CONFIG } from "@orqastudio/types";
import type { PluginRegistry } from "../plugins/plugin-registry.svelte.js";

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

/**
 * In-memory artifact graph SDK backed by Svelte 5 reactive state.
 *
 * After initialize() is called, all resolution and query methods operate
 * synchronously on the cached data — no IPC round-trips for lookups.
 */
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
	lastRefresh = $state<SvelteDate | null>(null);

	/** Error message from the last failed operation, or null when healthy. */
	error = $state<string | null>(null);

	/** Pending status transitions proposed by the backend engine. */
	pendingTransitions = $state<ProposedTransition[]>([]);

	// -----------------------------------------------------------------------
	// Private
	// -----------------------------------------------------------------------

	private registry: PluginRegistry;
	private nodeSubscribers = new SvelteMap<string, NodeCallback[]>();
	private typeSubscribers = new SvelteMap<string, TypeCallback[]>();
	private refreshCallbacks: RefreshCallback[] = [];
	private unlistenFn: UnlistenFn | null = null;
	private unlistenTransitionsFn: UnlistenFn | null = null;
	private _initCalled = false;
	private _initialized = false;
	private config: ArtifactGraphConfig | null = null;
	private _lastSchemaCount = 0;

	/**
	 * Create a new SDK instance bound to a plugin registry.
	 * @param registry - The plugin registry used for schema and relationship resolution.
	 */
	constructor(registry: PluginRegistry) {
		this.registry = registry;

		// Watch for plugin registrations that add new schemas after initial load.
		// When plugins register asynchronously, allTypeKeys is empty on first
		// _fetchAll(). This effect re-fetches once new schemas appear.
		$effect(() => {
			const schemaCount = this.registry.allSchemas.length;
			if (schemaCount > this._lastSchemaCount && this._initialized) {
				this._lastSchemaCount = schemaCount;
				void this._fetchAll();
			}
		});
	}

	// -----------------------------------------------------------------------
	// Schema access — all from the PluginRegistry
	// -----------------------------------------------------------------------

	/**
	 * All known artifact type keys (platform + plugin-registered).
	 * @returns Deduplicated array of type key strings from platform config and registered plugins.
	 */
	private get allTypeKeys(): string[] {
		const platformKeys = PLATFORM_CONFIG.artifactTypes.map((t) => t.key);
		const pluginKeys = this.registry.allSchemas.map((s) => s.key);
		return [...new SvelteSet([...platformKeys, ...pluginKeys])];
	}

	/**
	 * Get the inverse of a relationship key, or undefined if unknown.
	 * @param key - The forward relationship key to look up.
	 * @returns The inverse relationship key, or undefined when the key is not registered.
	 */
	getInverse(key: string): string | undefined {
		const rel = this.registry.getRelationship(key);
		return rel?.inverse;
	}

	/**
	 * Get all relationship keys for a given semantic category.
	 * @param semantic - The semantic category name (e.g. "foundation", "governance").
	 * @returns Array of relationship keys (both forward and inverse) for that semantic.
	 */
	keysForSemantic(semantic: string): string[] {
		return this.registry.allRelationships
			.filter((rel) => rel.semantic === semantic)
			.flatMap((rel) => (rel.inverse !== rel.key ? [rel.key, rel.inverse] : [rel.key]));
	}

	/**
	 * Validate a relationship between two artifact types. Returns null if valid.
	 * @param key - The relationship key to validate.
	 * @param fromType - The artifact type of the source node.
	 * @param toType - The artifact type of the target node.
	 * @returns Null when valid, or an error message string when invalid.
	 */
	validateRelationship(key: string, fromType: string, toType: string): string | null {
		return this.registry.validateRelationship(key, fromType, toType);
	}

	// -----------------------------------------------------------------------
	// Lifecycle
	// -----------------------------------------------------------------------

	/**
	 * Initialize the SDK: start the file watcher, load all artifacts, and subscribe to events.
	 * @param config - SDK configuration including project path and watcher options.
	 */
	async initialize(config: ArtifactGraphConfig): Promise<void> {
		if (this._initCalled) return;
		this._initCalled = true;
		this.config = config;

		const initStart = performance.now();
		log.info(`initialize: starting for ${config.projectPath}`);

		if (config.watchFiles !== false) {
			await invoke<void>("artifact_watch_start", { projectPath: config.projectPath }).catch(
				(err: unknown) => {
					log.warn("watcher failed to start", err);
				},
			);
		}

		await this._loadAll();
		this._lastSchemaCount = this.allTypeKeys.length;
		this._initialized = true;

		const elapsed_ms = (performance.now() - initStart).toFixed(1);
		log.info(`initialize: complete in ${elapsed_ms}ms, ${this.graph.size} nodes`);

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

	/**
	 * Clear the list of pending status transitions proposed by the backend engine.
	 */
	clearPendingTransitions(): void {
		this.pendingTransitions = [];
	}

	/**
	 * Trigger a full graph refresh: re-invoke the backend scan and fetch all nodes.
	 * @returns A promise that resolves when the graph has been updated.
	 */
	async refresh(): Promise<void> {
		if (this.loading) return;
		this.loading = true;
		this.error = null;
		const refreshStart = performance.now();
		try {
			await invoke<void>("refresh_artifact_graph");
			await this._fetchAll();
			this.lastRefresh = new SvelteDate();
			const elapsed_ms = (performance.now() - refreshStart).toFixed(1);
			log.info(`refresh: complete in ${elapsed_ms}ms`);
		} catch (err: unknown) {
			this.error = extractErrorMessage(err);
		} finally {
			this.loading = false;
		}
	}

	/**
	 * Register a callback to be invoked after every successful graph refresh.
	 * @param callback - Function to call when the graph has been refreshed.
	 * @returns An unsubscribe function that removes the callback when called.
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

	/**
	 * Resolve an artifact by ID from the in-memory graph.
	 * @param id - The artifact ID to look up.
	 * @returns The matching ArtifactNode, or undefined when not found.
	 */
	resolve(id: string): ArtifactNode | undefined {
		const direct = this.graph.get(id);
		if (direct) return direct;

		for (const node of this.graph.values()) {
			if (node.id === id) return node;
		}
		return undefined;
	}

	/**
	 * Resolve an artifact by its file path using the path index.
	 * @param path - The relative file path to look up.
	 * @returns The matching ArtifactNode, or undefined when not indexed.
	 */
	resolveByPath(path: string): ArtifactNode | undefined {
		const id = this.pathIndex.get(path);
		if (!id) return undefined;
		return this.graph.get(id);
	}

	// -----------------------------------------------------------------------
	// Relationship queries — synchronous
	// -----------------------------------------------------------------------

	/**
	 * Get all outgoing references from an artifact.
	 * @param id - The artifact ID to query.
	 * @returns Read-only array of outgoing ArtifactRef records, or empty when not found.
	 */
	referencesFrom(id: string): readonly ArtifactRef[] {
		return this.graph.get(id)?.references_out ?? [];
	}

	/**
	 * Get all incoming references to an artifact.
	 * @param id - The artifact ID to query.
	 * @returns Read-only array of incoming ArtifactRef records, or empty when not found.
	 */
	referencesTo(id: string): readonly ArtifactRef[] {
		return this.graph.get(id)?.references_in ?? [];
	}

	// -----------------------------------------------------------------------
	// Bulk queries — synchronous
	// -----------------------------------------------------------------------

	/**
	 * Get all artifacts of a given type from the in-memory graph.
	 * @param type - The artifact type key to filter by (e.g. "task", "epic").
	 * @returns Array of matching ArtifactNode records.
	 */
	byType(type: string): ArtifactNode[] {
		return Array.from(this.graph.values()).filter((n) => n.artifact_type === type);
	}

	/**
	 * Get all artifacts with a given status from the in-memory graph.
	 * @param status - The status value to filter by (e.g. "active", "done").
	 * @returns Array of matching ArtifactNode records.
	 */
	byStatus(status: string): ArtifactNode[] {
		return Array.from(this.graph.values()).filter((n) => n.status === status);
	}

	// -----------------------------------------------------------------------
	// Content — async disk read/write
	// -----------------------------------------------------------------------

	/**
	 * Read the raw file content for an artifact from disk.
	 * @param path - The relative file path to read.
	 * @returns The raw file content as a string.
	 */
	async readContent(path: string): Promise<string> {
		return invoke<string>("read_artifact_content", { path });
	}

	/**
	 * Update a single frontmatter field in an artifact file and refresh the graph.
	 * @param path - The relative file path of the artifact to update.
	 * @param field - The frontmatter field name to update.
	 * @param value - The new string value to write.
	 */
	async updateField(path: string, field: string, value: string): Promise<void> {
		await invoke<void>("update_artifact_field", { path, field, value });
		await this.refresh();
	}

	// -----------------------------------------------------------------------
	// Graph health — synchronous
	// -----------------------------------------------------------------------

	/**
	 * Find all outgoing references that point to non-existent artifact IDs.
	 * @returns Array of ArtifactRef records whose target_id is not in the graph.
	 */
	brokenRefs(): ArtifactRef[] {
		return Array.from(this.graph.values()).flatMap((node) =>
			node.references_out.filter((ref) => !this.graph.has(ref.target_id)),
		);
	}

	/**
	 * Find all artifacts with no incoming or outgoing references.
	 * @returns Array of ArtifactNode records that are completely disconnected from the graph.
	 */
	orphans(): ArtifactNode[] {
		return Array.from(this.graph.values()).filter(
			(n) => n.references_out.length === 0 && n.references_in.length === 0,
		);
	}

	// -----------------------------------------------------------------------
	// Relationship traversal — synchronous
	// -----------------------------------------------------------------------

	/**
	 * Follow outgoing references of a specific relationship type from an artifact.
	 * @param id - The starting artifact ID.
	 * @param relationshipType - The relationship key to follow (e.g. "delivers").
	 * @returns Array of ArtifactNode records reachable via that relationship type.
	 */
	traverse(id: string, relationshipType: string): ArtifactNode[] {
		const node = this.graph.get(id);
		if (!node) return [];
		return node.references_out
			.filter((ref) => ref.relationship_type === relationshipType)
			.flatMap((ref) => {
				const target = this.graph.get(ref.target_id);
				return target ? [target] : [];
			});
	}

	/**
	 * Follow incoming references of a specific relationship type to an artifact.
	 * @param id - The target artifact ID.
	 * @param relationshipType - The relationship key to follow in reverse (e.g. "delivers").
	 * @returns Array of ArtifactNode records that reference this artifact via that type.
	 */
	traverseIncoming(id: string, relationshipType: string): ArtifactNode[] {
		const node = this.graph.get(id);
		if (!node) return [];
		return node.references_in
			.filter((ref) => ref.relationship_type === relationshipType)
			.flatMap((ref) => {
				const source = this.graph.get(ref.source_id);
				return source ? [source] : [];
			});
	}

	/**
	 * Get all outgoing relationships from an artifact with resolved targets.
	 * @param id - The artifact ID to query.
	 * @returns Array of objects containing the resolved target node, relationship type, and optional rationale.
	 */
	relationshipsFrom(id: string): { target: ArtifactNode; type: string; rationale?: string }[] {
		const node = this.graph.get(id);
		if (!node) return [];
		const result: { target: ArtifactNode; type: string; rationale?: string }[] = [];

		const fmRelationships = (node.frontmatter as Record<string, unknown>)?.relationships;
		const rationales = new SvelteMap<string, string>();
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
	 * @param id - The artifact ID to use as the starting point.
	 * @returns Object with upstream and downstream arrays of resolved ArtifactNode records.
	 */
	pipelineChain(id: string): { upstream: ArtifactNode[]; downstream: ArtifactNode[] } {
		const upstream: ArtifactNode[] = [];
		const downstream: ArtifactNode[] = [];
		const visited = new SvelteSet<string>();

		// Build upstream/downstream key sets from all registered relationships
		const upstreamKeys = new SvelteSet<string>();
		const downstreamKeys = new SvelteSet<string>();

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

	/**
	 * Find relationships where the inverse edge is missing. Uses registry inverse map.
	 * @returns Array of objects containing the ref with no inverse and the expected inverse key.
	 */
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

	/**
	 * Run a full integrity scan on all artifacts via the backend.
	 * @returns Array of IntegrityCheck results identifying structural issues.
	 */
	async runIntegrityScan(): Promise<IntegrityCheck[]> {
		return invoke<IntegrityCheck[]>("run_integrity_scan");
	}

	/**
	 * Apply automatic fixes for the provided integrity check results.
	 * @param checks - The integrity check results to attempt fixing.
	 * @returns Array of AppliedFix records describing what was changed.
	 */
	async applyAutoFixes(checks: IntegrityCheck[]): Promise<AppliedFix[]> {
		return invoke<AppliedFix[]>("apply_auto_fixes", { checks });
	}

	// -----------------------------------------------------------------------
	// Health snapshots — async (requires backend storage)
	// -----------------------------------------------------------------------

	/**
	 * Persist a health snapshot to the backend database.
	 * @param errorCount - The number of errors recorded in this snapshot.
	 * @param warningCount - The number of warnings recorded in this snapshot.
	 * @returns The stored HealthSnapshot record with timestamp and ID.
	 */
	async storeHealthSnapshot(errorCount: number, warningCount: number): Promise<HealthSnapshot> {
		return invoke<HealthSnapshot>("store_health_snapshot", {
			errorCount,
			warningCount,
		});
	}

	/**
	 * Fetch recent health snapshots from the backend database.
	 * @param limit - Maximum number of snapshots to return; defaults to snapshotLimit from config.
	 * @returns Array of HealthSnapshot records ordered by timestamp, newest first.
	 */
	async getHealthSnapshots(limit?: number): Promise<HealthSnapshot[]> {
		const effectiveLimit = limit ?? this.config?.snapshotLimit ?? 30;
		return invoke<HealthSnapshot[]>("get_health_snapshots", { limit: effectiveLimit });
	}

	/**
	 * Fetch extended structural health metrics from the backend.
	 *
	 * Returns `GraphHealthData` computed by the Rust `compute_graph_health` function,
	 * including component count, orphan percentage, density, traceability, and
	 * bidirectionality. Use this instead of the client-side Cytoscape analysis.
	 * @returns GraphHealthData with structural metrics computed by the backend.
	 */
	async getGraphHealth(): Promise<GraphHealthData> {
		return invoke<GraphHealthData>("get_graph_health");
	}

	/**
	 * Fetch full traceability data for an artifact.
	 *
	 * Returns ancestry chains to pillar/vision roots, all downstream descendants
	 * with BFS depth, sibling artifacts sharing a common parent, impact radius,
	 * and a disconnected flag when no path to any pillar exists.
	 * @param id - The artifact ID to compute traceability for.
	 * @returns TraceabilityResult with ancestors, descendants, siblings, and connectivity data.
	 */
	async getTraceability(id: string): Promise<TraceabilityResult> {
		return invoke<TraceabilityResult>("get_artifact_traceability", { id });
	}

	// -----------------------------------------------------------------------
	// Subscriptions
	// -----------------------------------------------------------------------

	/**
	 * Subscribe to updates for a specific artifact by ID.
	 * @param id - The artifact ID to watch for changes.
	 * @param callback - Function called with the updated ArtifactNode on each refresh.
	 * @returns An unsubscribe function that removes this callback when called.
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
	 * Subscribe to updates for all artifacts of a given type.
	 * @param type - The artifact type key to watch (e.g. "task", "epic").
	 * @param callback - Function called with the full array of matching nodes on each refresh.
	 * @returns An unsubscribe function that removes this callback when called.
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

	private async _loadAll(): Promise<void> {
		if (this.loading) return;
		this.loading = true;
		this.error = null;
		try {
			await invoke<void>("refresh_artifact_graph");
			await this._fetchAll();
			this.lastRefresh = new SvelteDate();
		} catch (err: unknown) {
			this.error = extractErrorMessage(err);
		} finally {
			this.loading = false;
		}
	}

	private async _fetchAll(): Promise<void> {
		const fetchStart = performance.now();

		const [statsResult, allNodes] = await Promise.all([
			invoke<GraphStats>("get_graph_stats"),
			invoke<ArtifactNode[]>("get_all_artifacts"),
		]);

		const newGraph = new SvelteMap<string, ArtifactNode>();
		const newPathIndex = new SvelteMap<string, string>();

		for (const node of allNodes) {
			newGraph.set(node.id, node);
			newPathIndex.set(node.path, node.id);
		}

		const elapsed_ms = (performance.now() - fetchStart).toFixed(1);
		log.info(
			`_fetchAll complete in ${elapsed_ms}ms: ${newGraph.size} total nodes (stats reports ${statsResult.node_count})`,
		);

		this.graph = newGraph;
		this.pathIndex = newPathIndex;
		this.stats = statsResult;

		this._notifySubscribers(newGraph);

		for (const cb of this.refreshCallbacks) {
			try {
				cb();
			} catch (err: unknown) {
				log.warn("subscriber callback failed", err);
			}
		}
	}

	private _notifySubscribers(newGraph: ReadonlyMap<string, ArtifactNode>): void {
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
