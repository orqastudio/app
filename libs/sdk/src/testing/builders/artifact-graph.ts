/**
 * Test builder for artifact graphs.
 *
 * Takes an array of ArtifactNode objects, builds a Map keyed by id,
 * and populates referencesIn from referencesOut so the graph has
 * correct bidirectional edges.
 */
import type { ArtifactNode } from "./artifact-node.js";

/**
 * Build an artifact graph Map from an array of nodes.
 *
 * For each node, iterates its `referencesOut` and adds the node's id
 * to the target node's `referencesIn` array (if the target exists in
 * the provided nodes).
 *
 * ```ts
 * const epic = createTestNode({ id: "EPIC-001", referencesOut: ["TASK-001"] });
 * const task = createTestNode({ id: "TASK-001" });
 * const graph = createTestGraph([epic, task]);
 * // graph.get("TASK-001").referencesIn === ["EPIC-001"]
 * ```
 * @param nodes
 */
export function createTestGraph(nodes: ArtifactNode[]): Map<string, ArtifactNode> {
	const graph = new Map<string, ArtifactNode>();

	// Index all nodes by id
	for (const node of nodes) {
		graph.set(node.id, {
			...node,
			referencesIn: [...node.referencesIn],
			referencesOut: [...node.referencesOut],
		});
	}

	// Populate referencesIn from referencesOut
	for (const node of graph.values()) {
		for (const targetId of node.referencesOut) {
			const target = graph.get(targetId);
			if (target && !target.referencesIn.includes(node.id)) {
				target.referencesIn.push(node.id);
			}
		}
	}

	return graph;
}
