import type { ArtifactNode } from "@orqastudio/types";

/** Cached node position from a layout computation. */
export interface NodePosition {
    readonly id: string;
    readonly x: number;
    readonly y: number;
}

/**
 * Read-only view of the artifact graph data.
 * Implemented by the SDK's ArtifactGraphSDK — this interface decouples
 * the visualiser from the SDK's full API.
 */
export interface GraphDataSource {
    /** All nodes keyed by artifact ID. */
    readonly graph: ReadonlyMap<string, ArtifactNode>;
}
