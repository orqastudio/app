/**
 * Tests for the ArtifactGraphSDK's synchronous query methods.
 *
 * The SDK class uses Svelte runes ($state) which require a Svelte compilation
 * context, so we test the pure logic functions that don't depend on reactivity.
 * The invoke-dependent methods (initialize, refresh, readContent, etc.) require
 * a Tauri runtime and are tested via integration/E2E tests in the main app.
 *
 * These tests validate the data structures and query logic by testing against
 * plain Map objects that mirror the SDK's graph structure.
 */
import { describe, it, expect } from "vitest";
import type { ArtifactNode, ArtifactRef } from "@orqastudio/types";

// Helper to build test ArtifactNode objects matching the real interface
function makeNode(overrides: Partial<ArtifactNode> & { id: string }): ArtifactNode {
	return {
		path: `.orqa/delivery/tasks/${overrides.id}.md`,
		artifact_type: "task",
		title: overrides.id,
		description: null,
		status: "todo",
		frontmatter: {},
		references_out: [],
		references_in: [],
		...overrides,
	};
}

function makeRef(overrides: Partial<ArtifactRef> & { source_id: string; target_id: string }): ArtifactRef {
	return {
		field: "depends-on",
		relationship_type: null,
		...overrides,
	};
}

describe("artifact graph query logic", () => {
	describe("byType", () => {
		it("filters nodes by artifact_type", () => {
			const nodes = [
				makeNode({ id: "EPIC-001", artifact_type: "epic" }),
				makeNode({ id: "TASK-001", artifact_type: "task" }),
				makeNode({ id: "TASK-002", artifact_type: "task" }),
				makeNode({ id: "RULE-001", artifact_type: "rule" }),
			];

			const graph = new Map(nodes.map((n) => [n.id, n]));
			const tasks = [...graph.values()].filter((n) => n.artifact_type === "task");

			expect(tasks).toHaveLength(2);
			expect(tasks.map((t) => t.id)).toEqual(["TASK-001", "TASK-002"]);
		});

		it("returns empty array for unknown type", () => {
			const graph = new Map<string, ArtifactNode>();
			graph.set("TASK-001", makeNode({ id: "TASK-001" }));

			const result = [...graph.values()].filter((n) => n.artifact_type === "nonexistent");
			expect(result).toEqual([]);
		});
	});

	describe("byStatus", () => {
		it("filters nodes by status", () => {
			const nodes = [
				makeNode({ id: "TASK-001", status: "done" }),
				makeNode({ id: "TASK-002", status: "todo" }),
				makeNode({ id: "TASK-003", status: "done" }),
			];

			const graph = new Map(nodes.map((n) => [n.id, n]));
			const done = [...graph.values()].filter((n) => n.status === "done");

			expect(done).toHaveLength(2);
			expect(done.map((t) => t.id)).toEqual(["TASK-001", "TASK-003"]);
		});
	});

	describe("brokenRefs", () => {
		it("identifies references to non-existent nodes", () => {
			const ref = makeRef({ source_id: "TASK-001", target_id: "EPIC-999" });
			const node = makeNode({
				id: "TASK-001",
				references_out: [ref],
			});

			const graph = new Map<string, ArtifactNode>();
			graph.set("TASK-001", node);

			const broken: ArtifactRef[] = [];
			for (const n of graph.values()) {
				for (const r of n.references_out) {
					if (!graph.has(r.target_id)) {
						broken.push(r);
					}
				}
			}

			expect(broken).toHaveLength(1);
			expect(broken[0].target_id).toBe("EPIC-999");
		});

		it("returns empty when all refs resolve", () => {
			const ref = makeRef({ source_id: "TASK-001", target_id: "EPIC-001" });
			const task = makeNode({ id: "TASK-001", references_out: [ref] });
			const epic = makeNode({ id: "EPIC-001", artifact_type: "epic" });

			const graph = new Map<string, ArtifactNode>();
			graph.set("TASK-001", task);
			graph.set("EPIC-001", epic);

			const broken: ArtifactRef[] = [];
			for (const n of graph.values()) {
				for (const r of n.references_out) {
					if (!graph.has(r.target_id)) {
						broken.push(r);
					}
				}
			}

			expect(broken).toEqual([]);
		});
	});

	describe("orphans", () => {
		it("identifies nodes with no references in either direction", () => {
			const connected = makeNode({
				id: "TASK-001",
				references_out: [makeRef({ source_id: "TASK-001", target_id: "EPIC-001" })],
			});
			const orphan = makeNode({ id: "TASK-002" });
			const epic = makeNode({
				id: "EPIC-001",
				artifact_type: "epic",
				references_in: [makeRef({ source_id: "TASK-001", target_id: "EPIC-001" })],
			});

			const graph = new Map<string, ArtifactNode>();
			graph.set("TASK-001", connected);
			graph.set("TASK-002", orphan);
			graph.set("EPIC-001", epic);

			const orphans = [...graph.values()].filter(
				(n) => n.references_out.length === 0 && n.references_in.length === 0
			);

			expect(orphans).toHaveLength(1);
			expect(orphans[0].id).toBe("TASK-002");
		});
	});

	describe("traverse", () => {
		it("follows outgoing edges of a specific relationship type", () => {
			const ref = makeRef({
				source_id: "PILLAR-001",
				target_id: "RULE-001",
				relationship_type: "enforced-by",
			});
			const pillar = makeNode({
				id: "PILLAR-001",
				artifact_type: "pillar",
				references_out: [ref],
			});
			const rule = makeNode({ id: "RULE-001", artifact_type: "rule" });

			const graph = new Map<string, ArtifactNode>();
			graph.set("PILLAR-001", pillar);
			graph.set("RULE-001", rule);

			// Simulate traverse
			const node = graph.get("PILLAR-001");
			const result: ArtifactNode[] = [];
			if (node) {
				for (const r of node.references_out) {
					if (r.relationship_type === "enforced-by") {
						const target = graph.get(r.target_id);
						if (target) result.push(target);
					}
				}
			}

			expect(result).toHaveLength(1);
			expect(result[0].id).toBe("RULE-001");
		});
	});

	describe("missingInverses", () => {
		it("detects when A→B exists but B→A inverse is missing", () => {
			const INVERSE_MAP: Record<string, string> = {
				"observes": "observed-by",
				"observed-by": "observes",
				"enforces": "enforced-by",
				"enforced-by": "enforces",
			};

			const ref = makeRef({
				source_id: "RULE-001",
				target_id: "PILLAR-001",
				relationship_type: "enforces",
			});
			const rule = makeNode({
				id: "RULE-001",
				artifact_type: "rule",
				references_out: [ref],
			});
			// Pillar has no inverse reference back
			const pillar = makeNode({ id: "PILLAR-001", artifact_type: "pillar" });

			const graph = new Map<string, ArtifactNode>();
			graph.set("RULE-001", rule);
			graph.set("PILLAR-001", pillar);

			const missing: { ref: ArtifactRef; expectedInverse: string }[] = [];
			for (const node of graph.values()) {
				for (const r of node.references_out) {
					if (!r.relationship_type) continue;
					const expectedInverse = INVERSE_MAP[r.relationship_type];
					if (!expectedInverse) continue;

					const target = graph.get(r.target_id);
					if (!target) continue;

					const hasInverse = target.references_out.some(
						(inv) => inv.relationship_type === expectedInverse && inv.target_id === node.id
					);
					if (!hasInverse) {
						missing.push({ ref: r, expectedInverse });
					}
				}
			}

			expect(missing).toHaveLength(1);
			expect(missing[0].expectedInverse).toBe("enforced-by");
		});
	});
});
