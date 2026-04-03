/**
 * Port constants for all OrqaStudio services.
 *
 * All services derive their ports from a single base (ORQA_PORT_BASE, default
 * 10100) plus a fixed offset. This module is the single source of truth for
 * port numbers — every TypeScript consumer imports from here rather than
 * duplicating constants inline.
 */

/** Default base port for all OrqaStudio services. */
export const DEFAULT_PORT_BASE = 10100;

/** Service port offsets from the base. */
export const PORT_OFFSETS = {
  daemon: 0,
  lsp: 1,
  mcp: 2,
  vite: 20,
  dashboard: 30,
  sync: 31,
  devtools: 40,
  storybook: 50,
} as const;

/** Names of all known OrqaStudio services. */
export type ServiceName = keyof typeof PORT_OFFSETS;

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
 * with offset 0, so ORQA_PORT_BASE controls the entire port range.
 * @param service - The service name (e.g. "daemon", "lsp", "mcp").
 * @returns The port number for that service.
 */
export function getPort(service: ServiceName): number {
  return getPortBase() + PORT_OFFSETS[service];
}
