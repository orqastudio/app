/**
 * Tests for ArtifactGraphSDK synchronous query methods.
 *
 * ArtifactGraphSDK uses Svelte 5 $state runes and a $effect in its constructor.
 * Svelte's `effect_root()` from `svelte/internal/client` creates a standalone
 * reactive root outside a component, so we can instantiate the class in tests.
 *
 * All query methods tested here (resolve, byType, byStatus, brokenRefs, orphans,
 * traverse, traverseIncoming, missingInverses, subscribe, onRefresh) are pure
 * synchronous reads of the in-memory graph. We populate `sdk.graph` directly,
 * bypassing initialize() and IPC, to isolate the query logic.
 *
 * Tauri IPC is mocked globally to prevent errors when importing SDK modules.
 */

import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

// Mock Tauri before importing SDK modules that reference them at module level.
vi.mock("@tauri-apps/api/core", () => ({
	invoke: vi.fn().mockResolvedValue(undefined),
	Channel: class MockChannel {
		onmessage: ((msg: unknown) => void) | null = null;
	},
}));

vi.mock("@tauri-apps/api/event", () => ({
	listen: vi.fn().mockResolvedValue(() => {}),
	emit: vi.fn(),
}));

import { effect_root } from "svelte/internal/client";
import { SvelteMap } from "svelte/reactivity";
import { ArtifactGraphSDK } from "@orqastudio/sdk";
import type { ArtifactNode, ArtifactRef } from "@orqastudio/types";
import type { PluginRegistry } from "@orqastudio/sdk";

// ---------------------------------------------------------------------------
// Svelte effect root management
// ---------------------------------------------------------------------------

// Each test creates an effect root, instantiates the SDK inside it, then
// destroys the root in afterEach to avoid leaked effects across tests.
let destroyRoot: (() => void) | null = null;

afterEach(() => {
	if (destroyRoot) {
		destroyRoot();
		destroyRoot = null;
	}
});

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function makeRef(overrides: Partial<ArtifactRef> = {}): ArtifactRef {
	return {
		target_id: "TEST-002",
		field: "relationships",
		source_id: "TEST-001",
		relationship_type: null,
		...overrides,
	};
}

function makeNode(id: string, overrides: Partial<ArtifactNode> = {}): ArtifactNode {
	return {
		id,
		path: `.orqa/tasks/${id}.md`,
		artifact_type: "task",
		title: `Task ${id}`,
		description: null,
		status: "active",
		priority: null,
		frontmatter: {},
		references_out: [],
		references_in: [],
		...overrides,
	};
}

/**
 * Create a minimal PluginRegistry stub with no schemas or relationships.
 * @param overrides
 */
function makeRegistry(overrides: Partial<PluginRegistry> = {}): PluginRegistry {
	return {
		allSchemas: [],
		allRelationships: [],
		plugins: new Map(),
		getRelationship: () => null,
		validateRelationship: () => null,
		getSchema: () => undefined,
		resolveNavigationItem: (item: unknown) => ({ ...(item as object), label: "" }) as never,
		...overrides,
	} as unknown as PluginRegistry;
}

/**
 * Build an ArtifactGraphSDK inside a Svelte effect root and pre-populate the graph.
 * Returns the sdk. The root is tracked for cleanup in afterEach.
 * @param nodes
 * @param registry
 */
function buildSDK(nodes: ArtifactNode[], registry?: PluginRegistry): ArtifactGraphSDK {
	let sdk!: ArtifactGraphSDK;
	const reg = registry ?? makeRegistry();

	// effect_root creates an owner context for $effect / $state.
	// Assigning destroyRoot to the returned cleanup fn tears it down in afterEach.
	destroyRoot = effect_root(() => {
		sdk = new ArtifactGraphSDK(reg);
		const map = new SvelteMap<string, ArtifactNode>();
		for (const node of nodes) {
			map.set(node.id, node);
		}
		sdk.graph = map;
	});

	return sdk;
}

// ---------------------------------------------------------------------------
// resolve
// ---------------------------------------------------------------------------

describe("ArtifactGraphSDK.resolve", () => {
	it("returns a node by its id", () => {
		const node = makeNode("EPIC-001");
		const sdk = buildSDK([node]);
		expect(sdk.resolve("EPIC-001")).toBe(node);
	});

	it("returns undefined for an unknown id", () => {
		const sdk = buildSDK([]);
		expect(sdk.resolve("MISSING")).toBeUndefined();
	});
});

// ---------------------------------------------------------------------------
// byType
// ---------------------------------------------------------------------------

describe("ArtifactGraphSDK.byType", () => {
	it("returns all nodes of a given type", () => {
		const nodes = [
			makeNode("EPIC-001", { artifact_type: "epic" }),
			makeNode("TASK-001", { artifact_type: "task" }),
			makeNode("EPIC-002", { artifact_type: "epic" }),
		];
		const sdk = buildSDK(nodes);
		const epics = sdk.byType("epic");
		expect(epics).toHaveLength(2);
		expect(epics.every((n) => n.artifact_type === "epic")).toBe(true);
	});

	it("returns empty array when no nodes match", () => {
		const sdk = buildSDK([makeNode("TASK-001", { artifact_type: "task" })]);
		expect(sdk.byType("epic")).toHaveLength(0);
	});
});

// ---------------------------------------------------------------------------
// byStatus
// ---------------------------------------------------------------------------

describe("ArtifactGraphSDK.byStatus", () => {
	it("returns all nodes with a given status", () => {
		const nodes = [
			makeNode("T-001", { status: "active" }),
			makeNode("T-002", { status: "done" }),
			makeNode("T-003", { status: "active" }),
		];
		const sdk = buildSDK(nodes);
		expect(sdk.byStatus("active")).toHaveLength(2);
		expect(sdk.byStatus("done")).toHaveLength(1);
	});

	it("returns empty array when no nodes match", () => {
		const sdk = buildSDK([makeNode("T-001", { status: "active" })]);
		expect(sdk.byStatus("archived")).toHaveLength(0);
	});
});

// ---------------------------------------------------------------------------
// referencesFrom / referencesTo
// ---------------------------------------------------------------------------

describe("ArtifactGraphSDK.referencesFrom and referencesTo", () => {
	it("returns outgoing references for a node", () => {
		const ref = makeRef({ target_id: "T-002", source_id: "T-001" });
		const sdk = buildSDK([makeNode("T-001", { references_out: [ref] })]);
		expect(sdk.referencesFrom("T-001")).toEqual([ref]);
	});

	it("returns incoming references for a node", () => {
		const ref = makeRef({ target_id: "T-002", source_id: "T-001" });
		const sdk = buildSDK([makeNode("T-002", { references_in: [ref] })]);
		expect(sdk.referencesTo("T-002")).toEqual([ref]);
	});

	it("returns empty array for unknown id", () => {
		const sdk = buildSDK([]);
		expect(sdk.referencesFrom("MISSING")).toEqual([]);
		expect(sdk.referencesTo("MISSING")).toEqual([]);
	});
});

// ---------------------------------------------------------------------------
// brokenRefs
// ---------------------------------------------------------------------------

describe("ArtifactGraphSDK.brokenRefs", () => {
	it("returns refs whose target_id is not in the graph", () => {
		const danglingRef = makeRef({ target_id: "GHOST-001", source_id: "T-001" });
		const sdk = buildSDK([makeNode("T-001", { references_out: [danglingRef] })]);
		const broken = sdk.brokenRefs();
		expect(broken).toHaveLength(1);
		expect(broken[0].target_id).toBe("GHOST-001");
	});

	it("does not report refs whose target exists in the graph", () => {
		const validRef = makeRef({ target_id: "T-002", source_id: "T-001" });
		const sdk = buildSDK([makeNode("T-001", { references_out: [validRef] }), makeNode("T-002")]);
		expect(sdk.brokenRefs()).toHaveLength(0);
	});

	it("returns empty array when graph has no refs", () => {
		const sdk = buildSDK([makeNode("T-001")]);
		expect(sdk.brokenRefs()).toHaveLength(0);
	});
});

// ---------------------------------------------------------------------------
// orphans
// ---------------------------------------------------------------------------

describe("ArtifactGraphSDK.orphans", () => {
	it("returns nodes with no outgoing or incoming references", () => {
		const sdk = buildSDK([makeNode("T-001"), makeNode("T-002")]);
		expect(sdk.orphans()).toHaveLength(2);
	});

	it("does not report nodes that have at least one reference", () => {
		const ref = makeRef({ target_id: "T-002", source_id: "T-001" });
		const sdk = buildSDK([
			makeNode("T-001", { references_out: [ref] }),
			makeNode("T-002", { references_in: [ref] }),
		]);
		expect(sdk.orphans()).toHaveLength(0);
	});

	it("returns empty array for an empty graph", () => {
		expect(buildSDK([]).orphans()).toHaveLength(0);
	});
});

// ---------------------------------------------------------------------------
// traverse / traverseIncoming
// ---------------------------------------------------------------------------

describe("ArtifactGraphSDK.traverse", () => {
	it("returns nodes connected by the given relationship type", () => {
		const ref = makeRef({ target_id: "T-002", source_id: "T-001", relationship_type: "delivers" });
		const sdk = buildSDK([makeNode("T-001", { references_out: [ref] }), makeNode("T-002")]);
		const result = sdk.traverse("T-001", "delivers");
		expect(result).toHaveLength(1);
		expect(result[0].id).toBe("T-002");
	});

	it("does not return nodes connected by a different relationship type", () => {
		const ref = makeRef({ target_id: "T-002", source_id: "T-001", relationship_type: "blocks" });
		const sdk = buildSDK([makeNode("T-001", { references_out: [ref] }), makeNode("T-002")]);
		expect(sdk.traverse("T-001", "delivers")).toHaveLength(0);
	});

	it("returns empty array for unknown source id", () => {
		expect(buildSDK([]).traverse("MISSING", "delivers")).toHaveLength(0);
	});
});

describe("ArtifactGraphSDK.traverseIncoming", () => {
	it("returns nodes that point to the given node via the given relationship type", () => {
		const ref = makeRef({ source_id: "T-001", target_id: "T-002", relationship_type: "delivers" });
		const sdk = buildSDK([makeNode("T-001"), makeNode("T-002", { references_in: [ref] })]);
		const result = sdk.traverseIncoming("T-002", "delivers");
		expect(result).toHaveLength(1);
		expect(result[0].id).toBe("T-001");
	});
});

// ---------------------------------------------------------------------------
// missingInverses
// ---------------------------------------------------------------------------

describe("ArtifactGraphSDK.missingInverses", () => {
	it("detects a missing inverse edge", () => {
		const registry = makeRegistry({
			getRelationship: (key: string) =>
				key === "delivers"
					? ({ key: "delivers", inverse: "delivered-by", semantic: "lineage" } as never)
					: null,
		});

		const deliversRef = makeRef({
			source_id: "EPIC-001",
			target_id: "TASK-001",
			relationship_type: "delivers",
		});
		const sdk = buildSDK(
			[makeNode("EPIC-001", { references_out: [deliversRef] }), makeNode("TASK-001")],
			registry,
		);

		const missing = sdk.missingInverses();
		expect(missing).toHaveLength(1);
		expect(missing[0].expectedInverse).toBe("delivered-by");
	});

	it("does not flag edges when the inverse is present", () => {
		const registry = makeRegistry({
			getRelationship: (key: string) =>
				key === "delivers"
					? ({ key: "delivers", inverse: "delivered-by", semantic: "lineage" } as never)
					: null,
		});

		const deliversRef = makeRef({
			source_id: "EPIC-001",
			target_id: "TASK-001",
			relationship_type: "delivers",
		});
		const inverseRef = makeRef({
			source_id: "TASK-001",
			target_id: "EPIC-001",
			relationship_type: "delivered-by",
		});

		const sdk = buildSDK(
			[
				makeNode("EPIC-001", { references_out: [deliversRef] }),
				makeNode("TASK-001", { references_out: [inverseRef] }),
			],
			registry,
		);

		expect(sdk.missingInverses()).toHaveLength(0);
	});
});

// ---------------------------------------------------------------------------
// onRefresh subscription
// ---------------------------------------------------------------------------

describe("ArtifactGraphSDK.onRefresh", () => {
	it("registers and calls a refresh callback", () => {
		const sdk = buildSDK([]);
		const cb = vi.fn();
		sdk.onRefresh(cb);

		const internal = sdk as unknown as { refreshCallbacks: (() => void)[] };
		for (const fn of internal.refreshCallbacks) fn();

		expect(cb).toHaveBeenCalledOnce();
	});

	it("returns an unsubscribe function that removes the callback", () => {
		const sdk = buildSDK([]);
		const cb = vi.fn();
		const unsub = sdk.onRefresh(cb);
		unsub();

		const internal = sdk as unknown as { refreshCallbacks: (() => void)[] };
		for (const fn of internal.refreshCallbacks) fn();
		expect(cb).not.toHaveBeenCalled();
	});
});

// ---------------------------------------------------------------------------
// subscribe (node-level subscription)
// ---------------------------------------------------------------------------

describe("ArtifactGraphSDK.subscribe", () => {
	it("registers a per-node callback and allows unsubscription", () => {
		const sdk = buildSDK([]);
		const cb = vi.fn();
		const unsub = sdk.subscribe("EPIC-001", cb);

		const internal = sdk as unknown as {
			nodeSubscribers: Map<string, ((n: ArtifactNode) => void)[]>;
		};
		expect(internal.nodeSubscribers.get("EPIC-001")).toHaveLength(1);

		unsub();
		expect(internal.nodeSubscribers.has("EPIC-001")).toBe(false);
	});

	it("removes only the unsubscribed callback when multiple are registered", () => {
		const sdk = buildSDK([]);
		const cb1 = vi.fn();
		const cb2 = vi.fn();
		sdk.subscribe("EPIC-001", cb1);
		const unsub2 = sdk.subscribe("EPIC-001", cb2);
		unsub2();

		const internal = sdk as unknown as {
			nodeSubscribers: Map<string, ((n: ArtifactNode) => void)[]>;
		};
		const remaining = internal.nodeSubscribers.get("EPIC-001");
		expect(remaining).toHaveLength(1);
		expect(remaining?.[0]).toBe(cb1);
	});
});
