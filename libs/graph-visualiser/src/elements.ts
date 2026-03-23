/**
 * Build Cytoscape element definitions from artifact graph data.
 */

import type cytoscape from "cytoscape";
import type { ArtifactNode } from "@orqastudio/types";
import { ARTIFACT_TYPE_COLORS } from "./colors.js";

/**
 * Build a headless Cytoscape-compatible element array from graph data.
 * Used internally by the visualiser for analysis (PageRank, components, BFS).
 */
export function buildAnalysisElements(graph: ReadonlyMap<string, ArtifactNode>): cytoscape.ElementDefinition[] {
    const elements: cytoscape.ElementDefinition[] = [];

    for (const node of graph.values()) {
        elements.push({
            group: "nodes",
            data: { id: node.id, type: node.artifact_type, status: node.status },
        });
        for (const ref of node.references_out) {
            if (graph.has(ref.target_id)) {
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

    return elements;
}

/**
 * Build visualization-ready Cytoscape elements with colors, labels, tooltips.
 * Edges are deduplicated by source→target pair.
 */
export function buildVisualizationElements(graph: ReadonlyMap<string, ArtifactNode>): cytoscape.ElementDefinition[] {
    const elements: cytoscape.ElementDefinition[] = [];
    const edgeKeys = new Set<string>();

    for (const node of graph.values()) {
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

    for (const node of graph.values()) {
        for (const ref of node.references_out) {
            if (!graph.has(ref.target_id)) continue;
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
