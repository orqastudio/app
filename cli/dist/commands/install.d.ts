/**
 * Install commands — dev environment bootstrapping.
 *
 * orqa install              Full setup (prereqs + deps + build + plugin sync + verify)
 * orqa install prereqs      Check and install prerequisites (node 22+, rust)
 * orqa install deps         Install npm (workspaces) and cargo dependencies
 * orqa install build        Build all libs in dependency order
 * orqa install publish      Publish all libs to GitHub Package Registry
 *
 * Uses npm workspaces (root package.json) and Cargo workspace (root Cargo.toml).
 * No submodules, no npm link — workspaces resolve all \@orqastudio/* packages.
 */
import type { PluginManifest } from "@orqastudio/types";
/**
 * Dispatch the install command: full setup or individual install steps.
 * @param args - CLI arguments after "install".
 */
export declare function runInstallCommand(args: string[]): Promise<void>;
/**
 * Run enforcement generators declared in a plugin manifest.
 *
 * For each enforcement entry with role "generator" and a generator script path,
 * resolves the script and output paths then invokes the script via node so that
 * .orqa/configs/ is populated at install time. The rules-dir is always the
 * project's .orqa/learning/rules directory.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param pluginManifest - Parsed plugin manifest.
 * @param projectRoot - Absolute path to the project root.
 */
export declare function runPluginGenerators(pluginDir: string, pluginManifest: PluginManifest, projectRoot: string): void;
/**
 * Sync all enabled plugins from project.json to .orqa/.
 *
 * project.json is the source of truth for which plugins are active and where
 * they live. This function processes every plugin with enabled: true in order,
 * then runs the aggregation pipeline (schema, workflows, prompt registry).
 * @param root - Absolute path to the project root.
 */
export declare function cmdPluginSync(root: string): void;
//# sourceMappingURL=install.d.ts.map