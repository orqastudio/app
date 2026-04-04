/**
 * Plugin installer — download, extract, and manage plugin installations.
 *
 * Plugins are distributed as .tar.gz archives from GitHub Releases.
 * Local path installs are also supported for development.
 *
 * Post-install hooks: callers pass an optional `postInstall` callback in InstallOptions.
 * Connector-specific setup (e.g. .claude/ wiring) lives in the connector package,
 * not here. The CLI installer is platform-agnostic.
 */
import type { PluginManifest } from "@orqastudio/types";
export interface InstallOptions {
    /** GitHub owner/repo or local filesystem path. */
    readonly source: string;
    /** Specific version tag (e.g. "v0.2.0"). Defaults to latest release. */
    readonly version?: string;
    /** Project root directory (defaults to cwd). */
    readonly projectRoot?: string;
    /**
     * Optional post-install callback invoked after the plugin is extracted and locked.
     * Connectors use this to perform their own wiring (e.g. .claude/ directory setup).
     * @param pluginDir - The directory where the plugin was installed.
     * @param projectRoot - The project root directory.
     */
    readonly postInstall?: (pluginDir: string, projectRoot: string) => void;
}
export interface InstallResult {
    readonly name: string;
    readonly version: string;
    readonly path: string;
    readonly source: "github" | "local";
    /** Key collisions detected during installation. Empty when none. */
    readonly collisions: KeyCollisionResult[];
    /**
     * True when the installed plugin declares affectsSchema: true.
     * The caller must trigger schema recomposition after installation (P5-28).
     */
    readonly requiresSchemaRecomposition: boolean;
    /**
     * True when the installed plugin declares affectsEnforcement: true.
     * The caller must trigger enforcement config regeneration after installation (P5-28).
     */
    readonly requiresEnforcementRegeneration: boolean;
}
export interface KeyCollisionResult {
    readonly key: string;
    readonly existingSource: string;
    readonly existingDescription: string;
    readonly existingSemantic?: string;
    readonly existingFrom: string[];
    readonly existingTo: string[];
    readonly incomingDescription: string;
    readonly incomingSemantic?: string;
    readonly incomingFrom: string[];
    readonly incomingTo: string[];
    readonly semanticMatch: boolean;
}
export interface MethodologyConflict {
    /** The incoming plugin that is being installed. */
    readonly incomingPlugin: string;
    /** The core role that conflicts (e.g. "core:discovery"). */
    readonly role: string;
    /** The already-installed plugin that occupies this role. */
    readonly existingPlugin: string;
}
/**
 * Detect methodology exclusivity conflicts (legacy role-based check).
 *
 * Plugins with a `core:*` role are exclusive — only one per domain
 * (framework, discovery, delivery, governance) is allowed per project.
 * This function scans installed plugins and returns a conflict if the
 * incoming plugin's core role is already occupied.
 * @param manifest - The incoming plugin's manifest.
 * @param projectRoot - Absolute path to the project root.
 * @returns A conflict object if a conflict is found, or null.
 */
export declare function detectMethodologyConflict(manifest: PluginManifest, projectRoot: string): MethodologyConflict | null;
/**
 * Install a plugin from a GitHub release archive or local path.
 * @param options - Installation options including source, version, and project root.
 * @returns Installation result with name, version, and path.
 */
export declare function installPlugin(options: InstallOptions): Promise<InstallResult>;
/**
 * Uninstall a plugin by name.
 * @param name - The plugin name to uninstall.
 * @param projectRoot - Optional absolute path to the project root (defaults to cwd).
 */
export declare function uninstallPlugin(name: string, projectRoot?: string): void;
/**
 * List all installed plugins by scanning plugins/, connectors/, and sidecars/.
 *
 * plugins/ is scanned two levels deep because plugins are organised into
 * taxonomy subdirectories: plugins/<taxonomy>/<plugin>/orqa-plugin.json.
 * connectors/ and sidecars/ are scanned one level deep.
 * @param projectRoot - Optional absolute path to the project root (defaults to cwd).
 * @returns Array of install results for all found plugins.
 */
export declare function listInstalledPlugins(projectRoot?: string): InstallResult[];
//# sourceMappingURL=installer.d.ts.map