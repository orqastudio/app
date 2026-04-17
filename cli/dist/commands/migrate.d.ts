/**
 * Migrate command — status migrations and SurrealDB storage migration.
 *
 * Subcommands:
 *   orqa migrate storage ingest  — ingest user-authored .orqa/ artifacts into SurrealDB
 *   orqa migrate [options]       — apply status migrations from workflow definitions
 *
 * The `storage ingest` subcommand is the CLI face of TASK-S2-09: it pauses the
 * file watcher, calls POST /admin/migrate/storage/ingest on the daemon, then
 * resumes the watcher (in a finally block so resume always runs).
 *
 * The base command reads migration mappings from .orqa/workflows/*.resolved.json
 * and updates artifact frontmatter `status` fields accordingly. Only changes
 * status values that differ between old and new (i.e., actual renames, not
 * identity mappings).
 */
/**
 * Dispatch the migrate command: storage migration or workflow-driven status migrations.
 *
 * Routes `orqa migrate storage ingest` to the storage ingest subcommand.
 * All other invocations run the base workflow status migration logic.
 * @param args - CLI arguments after "migrate".
 */
export declare function runMigrateCommand(args: string[]): Promise<void>;
//# sourceMappingURL=migrate.d.ts.map