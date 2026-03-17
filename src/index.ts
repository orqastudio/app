/**
 * @orqastudio/cli — library exports for programmatic use.
 *
 * Used by @orqastudio/claude-code-cli and other consumers that need
 * plugin management, validation, or graph browsing without spawning a subprocess.
 */

export { installPlugin, uninstallPlugin, listInstalledPlugins } from "./lib/installer.js";
export { fetchRegistry } from "./lib/registry.js";
export { readLockfile, writeLockfile } from "./lib/lockfile.js";
export { readManifest, validateManifest } from "./lib/manifest.js";
export { scanArtifactGraph, queryGraph, type GraphNode, type GraphQueryOptions } from "./lib/graph.js";
