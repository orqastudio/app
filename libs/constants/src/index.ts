/**
 * `@orqastudio/constants` — shared constants for the OrqaStudio ecosystem.
 *
 * Re-exports all constant modules so consumers can import from the package
 * root without needing to know the internal file structure.
 */

export { DEFAULT_PORT_BASE, PORT_OFFSETS, getPortBase, getPort, getFixedPort } from "./ports.js";

export type { ServiceName, AnyServiceName } from "./ports.js";
