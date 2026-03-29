/**
 * Prompt registry generator — builds .orqa/prompt-registry.json from installed plugins.
 *
 * Reads knowledge_declarations from all installed plugin manifests and merges them
 * into a single registry file. Each declaration's content_file is resolved from the
 * plugin directory to a project-root-relative path.
 *
 * Called by orqa plugin install (after schema composition) and orqa install.
 * Satisfies P3 (Generated, Not Loaded): the registry is generated from the plugin
 * registry, not hand-maintained.
 */
/**
 * Generate .orqa/prompt-registry.json from all installed plugins.
 *
 * Scans all installed plugins, reads each plugin's knowledge_declarations,
 * resolves content_file paths to project-root-relative paths, and writes the
 * merged registry to .orqa/prompt-registry.json.
 *
 * Duplicate ids (same plugin + id) are deduplicated — last declaration wins
 * in case a plugin is scanned more than once.
 * @param projectRoot - Absolute path to the project root.
 * @returns The absolute path to the written registry file.
 */
export declare function generatePromptRegistry(projectRoot: string): string;
//# sourceMappingURL=prompt-registry.d.ts.map