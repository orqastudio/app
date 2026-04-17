/**
 * Plugin content lifecycle — manifest tracking, dependency installation, builds, and hooks.
 *
 * Plugins declare content mappings in `orqa-plugin.json`. Ownership of installed
 * content is tracked in `.orqa/manifest.json`. Content is no longer copied to `.orqa/`
 * subdirs by this module — SurrealDB is the source of truth for artifact content.
 */
import type { PluginManifest, PluginContentMapping } from "@orqastudio/types";
export interface FileHashEntry {
    readonly sourceHash: string;
    readonly installedHash: string;
}
export interface ContentManifest {
    plugins: Record<string, ContentManifestEntry>;
}
export interface ContentManifestEntry {
    version: string;
    installed_at: string;
    /** SHA-256 hash of the plugin's orqa-plugin.json at install time. Used by outdated checks to detect manifest changes even when version doesn't bump. */
    manifestHash?: string;
    /** Relative paths from project root, using forward slashes → hash entries. */
    files: Record<string, FileHashEntry>;
}
/**
 * Read `.orqa/manifest.json` from the project root.
 * Returns an empty manifest if the file does not exist.
 * @param projectRoot - Absolute path to the project root.
 * @returns The parsed content manifest, or an empty manifest if not found.
 */
export declare function readContentManifest(projectRoot: string): ContentManifest;
/**
 * Write `.orqa/manifest.json` to the project root with pretty-printed JSON.
 * @param projectRoot - Absolute path to the project root.
 * @param manifest - The content manifest to write.
 */
export declare function writeContentManifest(projectRoot: string, manifest: ContentManifest): void;
/**
 * Install plugin dependencies.
 *
 * - If `pluginManifest.dependencies.npm` is non-empty, runs `npm install` in pluginDir.
 * - If `pluginManifest.dependencies.system` is non-empty, checks each binary exists.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param pluginManifest - The plugin's manifest.
 * @throws {Error} If any required system binary is not found on PATH.
 */
export declare function installPluginDeps(pluginDir: string, pluginManifest: PluginManifest): void;
/**
 * Run the plugin's build command, if declared.
 * @param pluginDir - Absolute path to the plugin directory (cwd for the command).
 * @param pluginManifest - The plugin's manifest.
 */
export declare function buildPlugin(pluginDir: string, pluginManifest: PluginManifest): void;
/**
 * Run a plugin lifecycle hook command (`install` or `uninstall`), if declared.
 * @param pluginDir - Absolute path to the plugin directory (cwd for the command).
 * @param pluginManifest - The plugin's manifest.
 * @param hook - Which hook to run.
 */
export declare function runLifecycleHook(pluginDir: string, pluginManifest: PluginManifest, hook: "install" | "uninstall"): void;
/**
 * Process symlink declarations from a plugin manifest.
 * Creates symlinks in the project directory as declared.
 * @param projectRoot - Absolute path to the project root.
 * @param pluginManifest - The plugin's manifest containing symlink declarations.
 */
export declare function processPluginSymlinks(projectRoot: string, pluginManifest: PluginManifest): void;
/**
 * Process aggregated file declarations from all installed plugins.
 * Collects values from plugin manifests and writes them to a single output file.
 * @param projectRoot - Absolute path to the project root.
 */
export declare function processAggregatedFiles(projectRoot: string): void;
/**
 * Set up config extension for a content mapping with strategy "extends".
 * Creates the target config file that extends the source.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param projectRoot - Absolute path to the project root.
 * @param mapping - The content mapping declaring the extension.
 */
export declare function setupConfigExtends(pluginDir: string, projectRoot: string, mapping: PluginContentMapping): void;
//# sourceMappingURL=content-lifecycle.d.ts.map