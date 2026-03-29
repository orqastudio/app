/**
 * Plugin content lifecycle — install, remove, diff, and refresh plugin content.
 *
 * Plugins declare content mappings in `orqa-plugin.json`:
 *   { "content": { "rules": { "source": "rules", "target": ".orqa/learning/rules" } } }
 *
 * When installed, plugin content is copied from plugin source dirs to `.orqa/` target
 * dirs under the project root. Ownership is tracked in `.orqa/manifest.json`.
 */
import type { PluginManifest, PluginContentMapping } from "@orqastudio/types";
export interface FileHashEntry {
    sourceHash: string;
    installedHash: string;
}
export type ThreeWayState = "clean" | "plugin-updated" | "user-modified" | "conflict" | "missing";
export interface ThreeWayFileStatus {
    path: string;
    state: ThreeWayState;
}
export interface CopyResult {
    copied: Record<string, FileHashEntry>;
    skipped: ThreeWayFileStatus[];
}
export interface ContentManifest {
    plugins: Record<string, ContentManifestEntry>;
}
export interface ContentManifestEntry {
    version: string;
    installed_at: string;
    /** Relative paths from project root, using forward slashes → hash entries. */
    files: Record<string, FileHashEntry>;
}
export interface ContentDiffResult {
    pluginName: string;
    /** Files whose content is identical between plugin source and .orqa/ copy. */
    identical: string[];
    /** Files in .orqa/ that differ from the plugin source. */
    modified: string[];
    /** Files in the manifest but deleted from .orqa/. */
    missing: string[];
    /** Files found in the plugin's target dirs that are not in the manifest. */
    orphaned: string[];
    /** Three-way state for each tracked file. */
    threeWay: ThreeWayFileStatus[];
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
 * Copy files from the plugin's content source dirs to the project's target dirs.
 *
 * Supports three-way diff: when `currentManifest` is provided, files that have
 * been modified by the user (user-modified or conflict) are skipped.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param projectRoot - Absolute path to the project root.
 * @param manifest - The plugin's `orqa-plugin.json` manifest.
 * @param currentManifest - Optional existing content manifest for three-way state.
 * @returns CopyResult with copied files (with hashes) and skipped files.
 */
export declare function copyPluginContent(pluginDir: string, projectRoot: string, manifest: PluginManifest, currentManifest?: ContentManifest): CopyResult;
/**
 * Remove all content files belonging to a plugin and update the manifest.
 * @param pluginName - The plugin's `name` field from its manifest.
 * @param projectRoot - Absolute path to the project root.
 */
export declare function removePluginContent(pluginName: string, projectRoot: string): void;
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
 * Compare the plugin's source content against the installed copies in `.orqa/`.
 *
 * For each file tracked in the content manifest:
 * - If it no longer exists in `.orqa/`: listed as `missing`.
 * - If its content matches the plugin source: listed as `identical`.
 * - If its content differs: listed as `modified`.
 *
 * Files found in the plugin's target dirs that are NOT in the manifest are `orphaned`.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param projectRoot - Absolute path to the project root.
 * @param pluginManifest - The plugin's manifest.
 * @returns Diff result categorizing files as identical, modified, missing, or orphaned.
 */
export declare function diffPluginContent(pluginDir: string, projectRoot: string, pluginManifest: PluginManifest): ContentDiffResult;
/**
 * Re-install a plugin's dependencies, rebuild, re-copy content, and update the manifest.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param projectRoot - Absolute path to the project root.
 * @param pluginManifest - The plugin's manifest.
 * @returns CopyResult with copied and skipped files.
 */
export declare function refreshPluginContent(pluginDir: string, projectRoot: string, pluginManifest: PluginManifest): CopyResult;
/**
 * Compute the three-way state of a file by comparing:
 * - The installed file on disk vs the installedHash at last install
 * - The current source hash vs the sourceHash at last install
 *
 * States:
 * - "clean": neither user nor plugin changed it
 * - "plugin-updated": plugin has a newer version, user hasn't touched it
 * - "user-modified": user changed it, plugin hasn't updated
 * - "conflict": both user and plugin changed it
 * - "missing": file doesn't exist on disk
 * @param relPath - Relative path from project root to the file.
 * @param projectRoot - Absolute path to the project root.
 * @param lastEntry - The hash entry from the last install.
 * @param currentSourceHash - Current hash of the file in the plugin source.
 * @returns The three-way state of the file.
 */
export declare function computeThreeWayState(relPath: string, projectRoot: string, lastEntry: FileHashEntry, currentSourceHash: string): ThreeWayState;
/**
 * Compute the SHA-256 hash of a file.
 * @param filePath - Absolute path to the file to hash.
 * @returns Hex-encoded SHA-256 hash of the file contents.
 */
export declare function computeFileHash(filePath: string): string;
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
/**
 * Given a relative path from the project root (e.g. `.orqa/learning/rules/RULE-abc.md`),
 * find the corresponding source file in the plugin directory by matching target mappings.
 *
 * Returns the absolute path to the source file, or null if no mapping covers this path.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param pluginManifest - The plugin's manifest containing content mappings.
 * @param relPath - Relative path from the project root to the installed file.
 * @returns Absolute path to the source file, or null if no mapping covers this path.
 */
export declare function findSourceFile(pluginDir: string, pluginManifest: PluginManifest, relPath: string): string | null;
//# sourceMappingURL=content-lifecycle.d.ts.map