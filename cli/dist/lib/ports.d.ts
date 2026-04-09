/**
 * Port resolution for OrqaStudio CLI.
 *
 * Re-exports the canonical port constants and helpers from `@orqastudio/constants`.
 * All CLI code imports from this module so that there is one import path to
 * update if the package layout ever changes.
 */
export { DEFAULT_PORT_BASE, PORT_OFFSETS, getPortBase, getPort, getFixedPort, } from "@orqastudio/constants";
export type { ServiceName, AnyServiceName } from "@orqastudio/constants";
//# sourceMappingURL=ports.d.ts.map