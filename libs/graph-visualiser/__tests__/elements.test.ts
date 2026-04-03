/**
 * Tests for buildVisualizationElements.
 * Covers: empty graph, single node with no edges, nodes with edges,
 * edge deduplication, unknown artifact types, node tooltip format.
 */
import { describe, it, expect } from "vitest";
import { buildVisualizationElements } from "../src/elements.js";
import type { ArtifactNode, ArtifactRef } from "@orqastudio/types";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function makeNode(overrides: Partial<ArtifactNode> & { id: string }): ArtifactNode {
	return {
		path: `.orqa/implementation/tasks/${overrides.id}.md`,
		artifact_type: "task",
		title: `Title ${overrides.id}`,
		description: null,
		status: null,
		priority: null,
		frontmatter: {},
		references_out: [],
		references_in: [],
		...overrides,
	};
}

function makeRef(sourceId: string, targetId: string): ArtifactRef {
	return {
		source_id: sourceId,
		target_id: targetId,
		field: "delivers",
		relationship_type: "delivers",
	};
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("buildVisualizationElements", () => {
	it("returns empty array for empty graph", () => {
		const result = buildVisualizationElements(new Map());
		expect(result).toEqual([]);
	});

	it("returns a single node element for a graph with one node and no edges", () => {
		const node = makeNode({ id: "TASK-001" });
		const graph = new Map([["TASK-001", node]]);
		const elements = buildVisualizationElements(graph);
		expect(elements).toHaveLength(1);
		expect(elements[0]).toMatchObject({
			group: "nodes",
			data: { id: "TASK-001", label: "TASK-001" },
		});
	});

	it("assigns color from ARTIFACT_TYPE_COLORS for known type", () => {
		const node = makeNode({ id: "EPIC-001", artifact_type: "epic" });
		const elements = buildVisualizationElements(new Map([["EPIC-001", node]]));
		expect(elements[0]?.data?.color).toBe("#3b82f6");
	});

	it("uses fallback color #6b7280 for unknown artifact type", () => {
		const node = makeNode({ id: "UNKNOWN-001", artifact_type: "unknown-type" });
		const elements = buildVisualizationElements(new Map([["UNKNOWN-001", node]]));
		expect(elements[0]?.data?.color).toBe("#6b7280");
	});

	it("includes title and type in tooltip, no status when null", () => {
		const node = makeNode({ id: "TASK-001", title: "My Task", artifact_type: "task", status: null });
		const elements = buildVisualizationElements(new Map([["TASK-001", node]]));
		expect(elements[0]?.data?.tooltip).toBe("My Task\ntask");
	});

	it("includes status in tooltip when present", () => {
		const node = makeNode({ id: "TASK-001", title: "My Task", artifact_type: "task", status: "active" });
		const elements = buildVisualizationElements(new Map([["TASK-001", node]]));
		expect(elements[0]?.data?.tooltip).toBe("My Task\ntask · active");
	});

	it("creates an edge element for a reference between two graph nodes", () => {
		const nodeA = makeNode({ id: "TASK-001", references_out: [makeRef("TASK-001", "EPIC-001")] });
		const nodeB = makeNode({ id: "EPIC-001", artifact_type: "epic" });
		const graph = new Map([
			["TASK-001", nodeA],
			["EPIC-001", nodeB],
		]);
		const elements = buildVisualizationElements(graph);
		const edges = elements.filter((e) => e.group === "edges");
		expect(edges).toHaveLength(1);
		expect(edges[0]).toMatchObject({
			group: "edges",
			data: { id: "TASK-001->EPIC-001", source: "TASK-001", target: "EPIC-001" },
		});
	});

	it("skips edges whose target is not in the graph", () => {
		const node = makeNode({
			id: "TASK-001",
			references_out: [makeRef("TASK-001", "EPIC-NOT-IN-GRAPH")],
		});
		const elements = buildVisualizationElements(new Map([["TASK-001", node]]));
		expect(elements.filter((e) => e.group === "edges")).toHaveLength(0);
	});

	it("deduplicates edges with the same source→target key", () => {
		// Two references from TASK-001 to EPIC-001 (different fields)
		const nodeA = makeNode({
			id: "TASK-001",
			references_out: [
				makeRef("TASK-001", "EPIC-001"),
				{ source_id: "TASK-001", target_id: "EPIC-001", field: "blocked-by", relationship_type: null },
			],
		});
		const nodeB = makeNode({ id: "EPIC-001", artifact_type: "epic" });
		const graph = new Map([
			["TASK-001", nodeA],
			["EPIC-001", nodeB],
		]);
		const elements = buildVisualizationElements(graph);
		const edges = elements.filter((e) => e.group === "edges");
		expect(edges).toHaveLength(1);
	});

	it("produces correct counts for a multi-node graph with multiple edges", () => {
		const nodeA = makeNode({
			id: "TASK-001",
			references_out: [makeRef("TASK-001", "EPIC-001"), makeRef("TASK-001", "EPIC-002")],
		});
		const nodeB = makeNode({ id: "EPIC-001", artifact_type: "epic", references_out: [] });
		const nodeC = makeNode({ id: "EPIC-002", artifact_type: "epic", references_out: [] });
		const graph = new Map([
			["TASK-001", nodeA],
			["EPIC-001", nodeB],
			["EPIC-002", nodeC],
		]);
		const elements = buildVisualizationElements(graph);
		const nodes = elements.filter((e) => e.group === "nodes");
		const edges = elements.filter((e) => e.group === "edges");
		expect(nodes).toHaveLength(3);
		expect(edges).toHaveLength(2);
	});
});
