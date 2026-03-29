/**
 * Plugin manifest reader and validator.
 *
 * Reads `orqa-plugin.json` from a plugin directory and validates its structure.
 */
import type { PluginManifest } from "@orqastudio/types";
/**
 * Read a plugin manifest from a directory.
 * @param pluginDir - Absolute path to the plugin directory containing orqa-plugin.json.
 * @returns The parsed plugin manifest.
 */
export declare function readManifest(pluginDir: string): PluginManifest;
/**
 * Validate a plugin manifest, returning an array of error messages.
 * Empty array means valid.
 * @param manifest - The plugin manifest to validate.
 * @returns Array of error message strings; empty means valid.
 */
export declare function validateManifest(manifest: PluginManifest): string[];
/**
 * Check that a plugin manifest contributes at least one capability or content mapping.
 * A plugin that provides nothing is invalid.
 * @param manifest - The plugin manifest to check.
 * @returns True if the manifest provides at least one capability or content mapping.
 */
export declare function isPluginManifestNonEmpty(manifest: PluginManifest): boolean;
//# sourceMappingURL=manifest.d.ts.map