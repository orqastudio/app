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

/** Default port for the orqa-daemon health endpoint (matches health.rs DEFAULT_PORT). */
const DEFAULT_DAEMON_PORT = 9120;

/**
 * Resolve the daemon health port from `ORQA_PORT_BASE`.
 *
 * The daemon reads `ORQA_PORT_BASE` as the direct port number, not as a base
 * for an offset. This matches `daemon/src/health.rs resolve_port()`.
 */
export function getDaemonPort(): number {
	const raw = process.env["ORQA_PORT_BASE"];
	if (raw === undefined || raw === "") return DEFAULT_DAEMON_PORT;
	const n = parseInt(raw, 10);
	return Number.isNaN(n) ? DEFAULT_DAEMON_PORT : n;
}

/**
 * Resolve the port for a named service.
 *
 * @param service  One of `"daemon"`, `"vite"`, `"dashboard"`, `"sync"`.
 * @returns The port number for that service.
 * @throws If the service name is unknown.
 */
export function getPort(service: string): number {
	switch (service) {
		case "daemon":
			return getDaemonPort();
		case "vite":
			return 10420;
		case "dashboard":
			return 10401;
		case "sync":
			return 10402;
		default:
			throw new Error(
				`Unknown service "${service}". Known services: daemon, vite, dashboard, sync`,
			);
	}
}
