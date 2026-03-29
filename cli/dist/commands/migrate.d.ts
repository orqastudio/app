/**
 * Migrate command — apply status migrations from workflow definitions.
 *
 * orqa migrate [options]
 *
 * Reads migration mappings from .orqa/workflows/*.resolved.json and updates
 * artifact frontmatter `status` fields accordingly. Only changes status values
 * that differ between old and new (i.e., actual renames, not identity mappings).
 */
/**
 * Dispatch the migrate command: apply workflow-driven status migrations across the graph.
 * @param args - CLI arguments after "migrate".
 */
export declare function runMigrateCommand(args: string[]): Promise<void>;
//# sourceMappingURL=migrate.d.ts.map