/**
 * Port resolution utility for OrqaStudio services.
 *
 * The daemon health endpoint is the canonical service with its own port.
 * `ORQA_PORT_BASE` is read directly as the daemon health port (default 9120),
 * matching `daemon/src/health.rs resolve_port()`. All CLI consumers that need
 * the daemon port call `getDaemonPort()` so there is one source of truth.
 *
 * Other services (vite, dashboard, sync) use fixed default ports and do not
 * share the daemon's port variable.
 */
/**
 * Resolve the daemon health port from `ORQA_PORT_BASE`.
 *
 * The daemon reads `ORQA_PORT_BASE` as the direct port number, not as a base
 * for an offset. This matches `daemon/src/health.rs resolve_port()`.
 * @returns The resolved daemon port number.
 */
export declare function getDaemonPort(): number;
/**
 * Resolve the port for a named service.
 * @param service - One of `"daemon"`, `"vite"`, `"dashboard"`, `"sync"`.
 * @returns The port number for that service.
 * @throws {Error} If the service name is unknown.
 */
export declare function getPort(service: string): number;
//# sourceMappingURL=ports.d.ts.map