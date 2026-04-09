/**
 * Port constants for all OrqaStudio services.
 *
 * All ports are defined in infrastructure/ports.json as the single source of
 * truth. This module reads that file and exposes the canonical getPortBase()
 * and getPort() API consumed by all TypeScript code. The vite offset (320)
 * gives 10100 + 320 = 10420, placing the app Vite server well outside the
 * base+0..base+99 cluster used by daemon subsystems.
 *
 * Static config files (tauri.conf.json, docker-compose.yml) that cannot import
 * at runtime must hardcode port values — they are validated by `orqa check ports`.
 */

import { createRequire } from "node:module";

// Load the canonical ports definition from infrastructure/ports.json.
// createRequire is used for JSON imports to remain compatible with both CJS
// and ESM output without --experimental-json-modules.
const require = createRequire(import.meta.url);
// Path is relative to this compiled file's location (libs/constants/src/).
// The JSON file lives at infrastructure/ports.json from the repo root.
const PORTS_JSON = require("../../../infrastructure/ports.json") as PortsJson;

/** Shape of infrastructure/ports.json. */
interface ServiceEntry {
	readonly offset: number | null;
	readonly port: number;
	readonly description: string;
}

interface PortsJson {
	readonly base: number;
	readonly services: Record<string, ServiceEntry>;
}

/** Default base port for all OrqaStudio services (daemon offset = 0). */
export const DEFAULT_PORT_BASE: number = PORTS_JSON.base;

/**
 * Service port offsets from the base.
 *
 * Only services that derive from the base via an offset are included.
 * forgejo_http and forgejo_ssh have fixed ports (not base-relative) and are
 * accessed via the full service map.
 */
export const PORT_OFFSETS = {
	daemon: PORTS_JSON.services["daemon"]!.offset as number,
	lsp: PORTS_JSON.services["lsp"]!.offset as number,
	mcp: PORTS_JSON.services["mcp"]!.offset as number,
	vite: PORTS_JSON.services["vite"]!.offset as number,
	dashboard: PORTS_JSON.services["dashboard"]!.offset as number,
	sync: PORTS_JSON.services["sync"]!.offset as number,
	devtools: PORTS_JSON.services["devtools"]!.offset as number,
	storybook: PORTS_JSON.services["storybook"]!.offset as number,
} as const;

/** Names of all known OrqaStudio services that derive from the base port. */
export type ServiceName = keyof typeof PORT_OFFSETS;

/** All service names including those with fixed ports (forgejo). */
export type AnyServiceName = keyof typeof PORTS_JSON.services;

/**
 * Resolve the port base from ORQA_PORT_BASE environment variable.
 *
 * Returns DEFAULT_PORT_BASE when the variable is absent or not a valid integer.
 * @returns The resolved port base number.
 */
export function getPortBase(): number {
	const raw = process.env["ORQA_PORT_BASE"];
	if (!raw) return DEFAULT_PORT_BASE;
	const n = parseInt(raw, 10);
	return Number.isNaN(n) ? DEFAULT_PORT_BASE : n;
}

/**
 * Resolve the port for a named OrqaStudio service.
 *
 * Computes getPortBase() + PORT_OFFSETS[service]. The daemon port is the base
 * with offset 0, so ORQA_PORT_BASE controls the entire offset-relative range.
 * @param service - The service name (e.g. "daemon", "lsp", "mcp", "vite").
 * @returns The port number for that service.
 */
export function getPort(service: ServiceName): number {
	return getPortBase() + PORT_OFFSETS[service];
}

/**
 * Resolve the fixed port for a forgejo service (http or ssh).
 *
 * Forgejo ports are not base-relative — they are always the fixed values from
 * infrastructure/ports.json regardless of ORQA_PORT_BASE.
 * @param service - "forgejo_http" or "forgejo_ssh".
 * @returns The fixed port number.
 */
export function getFixedPort(service: "forgejo_http" | "forgejo_ssh"): number {
	return PORTS_JSON.services[service]!.port;
}
