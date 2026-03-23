/**
 * Pure-function graph analysis — no Svelte reactivity.
 *
 * Each function takes a Cytoscape core instance and the graph data,
 * returns computed results. The reactive layer (Svelte $derived) lives
 * in the GraphVisualiser class.
 */

import type cytoscape from "cytoscape";
import type { ArtifactNode } from "@orqastudio/types";
import type { GraphHealth, BackboneArtifact, KnowledgeGaps, ImpactResult } from "./types.js";

/** Compute structural health metrics from a Cytoscape instance. */
export function computeGraphHealth(cy: cytoscape.Core): GraphHealth {
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
}

/** Compute top-N artifacts by PageRank. */
export function computeBackboneArtifacts(
    cy: cytoscape.Core,
    graph: ReadonlyMap<string, ArtifactNode>,
    topN = 10,
): BackboneArtifact[] {
    if (cy.nodes().length === 0) return [];

    const pr = cy.elements().pageRank({});
    const scored = cy.nodes().map((n) => ({
        id: n.id(),
        rank: pr.rank(n),
    }));

    scored.sort((a, b) => b.rank - a.rank);

    return scored.slice(0, topN).map(({ id, rank }) => {
        const node = graph.get(id);
        return {
            id,
            title: node?.title ?? id,
            type: node?.artifact_type ?? "unknown",
            rank,
        };
    });
}

/** Detect knowledge gaps from the relationship graph. */
export function computeKnowledgeGaps(graph: ReadonlyMap<string, ArtifactNode>): KnowledgeGaps {
    const ungroundedRules: string[] = [];
    const unusedKnowledge: string[] = [];
    const unenforcedDecisions: string[] = [];

    for (const node of graph.values()) {
        if (node.artifact_type === "rule") {
            const hasGrounding = node.references_out.some(
                (r) => r.relationship_type === "grounded-by" && graph.get(r.target_id)?.artifact_type === "pillar"
            );
            if (!hasGrounding) ungroundedRules.push(node.id);
        } else if (node.artifact_type === "knowledge") {
            const hasPractitioner = node.references_in.some(
                (r) => r.relationship_type === "grounded-by" && graph.get(r.source_id)?.artifact_type === "agent"
            );
            if (!hasPractitioner) unusedKnowledge.push(node.id);
        } else if (node.artifact_type === "decision") {
            const hasGovernsOrDrives = node.references_out.some(
                (r) => r.relationship_type === "governs" || r.relationship_type === "drives"
            );
            if (!hasGovernsOrDrives) unenforcedDecisions.push(node.id);
        }
    }

    return { ungroundedRules, unusedKnowledge, unenforcedDecisions };
}

/**
 * BFS trace from an artifact following the graph topology.
 *
 * - `'up'`   — follow edges TO the node (who references this?).
 * - `'down'` — follow edges FROM the node (what does this reference?).
 *
 * Returns an ordered array of artifact IDs visited during BFS,
 * not including the starting node itself.
 */
export function traceChain(cy: cytoscape.Core, id: string, direction: "up" | "down"): string[] {
    const startNode = cy.getElementById(id);
    if (startNode.empty()) return [];

    const visited: string[] = [];
    const seen = new Set<string>([id]);
    const queue: string[] = [id];

    while (queue.length > 0) {
        const current = queue.shift()!;
        const cyNode = cy.getElementById(current);

        const neighbours =
            direction === "up"
                ? cyNode.incomers("node")
                : cyNode.outgoers("node");

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
 * Return all artifacts within `maxDepth` hops from the given node.
 */
export function computeImpact(
    cy: cytoscape.Core,
    graph: ReadonlyMap<string, ArtifactNode>,
    id: string,
    maxDepth = 2,
): ImpactResult {
    const startNode = cy.getElementById(id);
    if (startNode.empty()) return { total: 0, artifacts: [] };

    const results: Array<{ id: string; type: string; distance: number }> = [];
    const seen = new Set<string>([id]);

    let frontier: string[] = [id];
    for (let depth = 1; depth <= maxDepth; depth++) {
        const nextFrontier: string[] = [];
        for (const current of frontier) {
            const cyNode = cy.getElementById(current);
            cyNode.neighborhood("node").forEach((n) => {
                const nid = n.id();
                if (!seen.has(nid)) {
                    seen.add(nid);
                    const node = graph.get(nid);
                    results.push({ id: nid, type: node?.artifact_type ?? "unknown", distance: depth });
                    nextFrontier.push(nid);
                }
            });
        }
        frontier = nextFrontier;
    }

    return { total: results.length, artifacts: results };
}
