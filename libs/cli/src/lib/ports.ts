/**
 * Port resolution utility for OrqaStudio services.
 *
 * Reads `ORQA_PORT_BASE` from the environment (default: 10200) and applies
 * fixed offsets to derive each service's port.  This lets multiple OrqaStudio
 * instances coexist on different port ranges.
 *
 * Port map (default base = 10200):
 *   daemon    = base + 58  = 10258
 *   vite      = base + 220 = 10420
 *   dashboard = base + 201 = 10401
 *   sync      = base + 202 = 10402
 */

const DEFAULT_PORT_BASE = 10200;

const SERVICE_OFFSETS: Record<string, number> = {
	daemon: 58,
	vite: 220,
	dashboard: 201,
	sync: 202,
};

/**
 * Read the port base from `ORQA_PORT_BASE`, falling back to 10200.
 */
export function getPortBase(): number {
	const raw = process.env["ORQA_PORT_BASE"];
	if (raw === undefined || raw === "") return DEFAULT_PORT_BASE;
	const n = parseInt(raw, 10);
	return Number.isNaN(n) ? DEFAULT_PORT_BASE : n;
}

/**
 * Resolve the port for a named service.
 *
 * @param service  One of `"daemon"`, `"vite"`, `"dashboard"`, `"sync"`.
 * @returns The port number (port base + offset).
 * @throws If the service name is unknown.
 */
export function getPort(service: string): number {
	const offset = SERVICE_OFFSETS[service];
	if (offset === undefined) {
		throw new Error(
			`Unknown service "${service}". Known services: ${Object.keys(SERVICE_OFFSETS).join(", ")}`,
		);
	}
	return getPortBase() + offset;
}
