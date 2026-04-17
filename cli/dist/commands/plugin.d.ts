/**
 * Plugin management commands.
 *
 * orqa plugin list|install|uninstall|update|enable|disable|refresh|registry|create
 */
import type { PluginProjectConfig } from "@orqastudio/types";
/**
 * Dispatch the plugin command: install, uninstall, list, refresh, create, link.
 * @param args - CLI arguments after "plugin".
 */
export declare function runPluginCommand(args: string[]): Promise<void>;
/**
 * Read and parse .orqa/project.json.
 * @param projectRoot - Absolute path to the project root.
 * @returns Parsed project.json as a plain object.
 */
export declare function readProjectJson(projectRoot: string): Record<string, unknown>;
/**
 * Update the plugins section of .orqa/project.json for a single plugin.
 * Merges with any existing entry.
 * @param projectRoot - Absolute path to the project root.
 * @param name - Plugin name as it appears in project.json.
 * @param updates - Fields to merge into the existing plugin entry.
 */
export declare function updateProjectJsonPlugin(projectRoot: string, name: string, updates: Partial<PluginProjectConfig>): void;
//# sourceMappingURL=plugin.d.ts.map