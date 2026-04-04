/**
 * Build Cytoscape element definitions from artifact graph data.
 */

import type cytoscape from "cytoscape";
import type { ArtifactNode } from "@orqastudio/types";
import { ARTIFACT_TYPE_COLORS } from "./colors.js";

/**
 * Build visualization-ready Cytoscape elements with colors, labels, tooltips.
 * Edges are deduplicated by source→target pair.
 */
export function buildVisualizationElements(graph: ReadonlyMap<string, ArtifactNode>): cytoscape.ElementDefinition[] {
    const nodeElements: cytoscape.ElementDefinition[] = [...graph.values()].map((node) => ({
        group: "nodes" as const,
        data: {
            id: node.id,
            label: node.id,
            color: ARTIFACT_TYPE_COLORS[node.artifact_type] ?? "#6b7280",
            tooltip: `${node.title}\n${node.artifact_type}${node.status ? ` · ${node.status}` : ""}`,
        },
    }));

    // Edge deduplication requires tracking seen keys across iterations — a for loop is clearest here.
    const edgeElements: cytoscape.ElementDefinition[] = [];
    const edgeKeys = new Set<string>();
    for (const node of graph.values()) {
        for (const ref of node.references_out) {
            if (!graph.has(ref.target_id)) continue;
            const key = `${ref.source_id}->${ref.target_id}`;
            if (edgeKeys.has(key)) continue;
            edgeKeys.add(key);
            edgeElements.push({
                group: "edges",
                data: {
                    id: key,
                    source: ref.source_id,
                    target: ref.target_id,
                },
            });
        }
    }

    return [...nodeElements, ...edgeElements];
}
