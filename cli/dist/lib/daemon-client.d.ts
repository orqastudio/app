/**
 * HTTP client for the orqa-daemon.
 *
 * Daemon runs at localhost:10100 by default (port is ORQA_PORT_BASE, matched
 * from daemon/src/health.rs resolve_port()). Provides canonical graph
 * scanning, query, and validation endpoints. Falls back to spawning the
 * `orqa-validation` binary when the daemon is unreachable.
 */
/** Mirrors the Rust `ArtifactRef` struct. */
export interface DaemonArtifactRef {
    target_id: string;
    field: string;
    source_id: string;
    relationship_type: string | null;
}
/** Mirrors the Rust `ArtifactNode` struct returned by GET /artifacts. */
export interface DaemonArtifactNode {
    id: string;
    project?: string;
    path: string;
    artifact_type: string;
    title: string;
    description: string | null;
    status: string | null;
    priority: string | null;
    frontmatter: Record<string, unknown>;
    body?: string;
    references_out: DaemonArtifactRef[];
    references_in: DaemonArtifactRef[];
}
/** Response shape from GET /health. */
export interface DaemonHealthResponse {
    status: string;
    artifacts: number;
    rules: number;
}
/**
 * Call a daemon endpoint. Falls back to the `orqa-validation` binary if
 * the daemon is unreachable.
 * @param method - HTTP method (GET or POST)
 * @param path - Endpoint path (e.g. "/query")
 * @param body - JSON body for POST requests
 * @returns The parsed JSON response from the daemon or binary fallback.
 */
export declare function callDaemonGraph<T>(method: "GET" | "POST", path: string, body?: unknown): Promise<T>;
/**
 * Check if the daemon is reachable by hitting GET /health.
 * @returns True if the daemon responds to a health check within 1 second.
 */
export declare function isDaemonRunning(): Promise<boolean>;
//# sourceMappingURL=daemon-client.d.ts.map