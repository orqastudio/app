/**
 * Config generator — reads coding standards rules and generates tool config files.
 *
 * Rules define enforcement entries keyed by plugin and tool. This module
 * reads those entries, merges org-level config with sub-project overrides,
 * and generates the tool config files (e.g. .eslintrc.json, clippy.toml).
 *
 * The generated config files are the OUTPUT of the governance system.
 * Developers edit rules, not config files.
 */
/** A single config line from a rule's enforcement entry. */
export interface ConfigEntry {
    [key: string]: unknown;
}
/** A parsed enforcement entry from a rule's frontmatter. */
export interface EnforcementEntry {
    plugin: string;
    tool: string;
    config: ConfigEntry[];
}
/** A tool definition from a plugin's orqa-plugin.json. */
export interface ToolDefinition {
    command: string;
    configFile: string | null;
    configFormat: "json" | "toml" | "ts" | "cli-args";
}
/** Result of config generation for one project. */
export interface GeneratedConfig {
    project: string;
    file: string;
    entries: number;
}
/**
 * Extract enforcement entries from all rules in a directory.
 * @param rulesDir - Absolute path to the rules directory to scan.
 * @returns Array of parsed enforcement entries from all rule files.
 */
export declare function extractEnforcementEntries(rulesDir: string): EnforcementEntry[];
/**
 * Load tool definitions from installed plugin manifests.
 * @param projectRoot - Absolute path to the project root.
 * @returns Map of plugin name to a map of tool name to tool definition.
 */
export declare function loadPluginTools(projectRoot: string): Map<string, Map<string, ToolDefinition>>;
/**
 * Generate config files from enforcement entries.
 * @param projectRoot - Absolute path to the project root where configs are written.
 * @param entries - Enforcement entries to generate configs from.
 * @param pluginTools - Map of plugin tools loaded from manifests.
 * @returns Array of generated config file records.
 */
export declare function generateConfigs(projectRoot: string, entries: EnforcementEntry[], pluginTools: Map<string, Map<string, ToolDefinition>>): GeneratedConfig[];
//# sourceMappingURL=config-generator.d.ts.map