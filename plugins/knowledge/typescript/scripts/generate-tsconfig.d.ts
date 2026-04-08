/**
 * tsconfig base generator for the OrqaStudio enforcement pipeline.
 *
 * Scans all rule files under --rules-dir, filters enforcement entries where
 * engine=tsconfig, and produces a self-contained tsconfig.base.json at
 * --output.
 *
 * The base compiler options come from the plugin's canonical tsconfig preset.
 * Entries from rule files can override individual compiler options via the
 * `option` and `value` fields in their enforcement entry.
 *
 * Invocation:
 *   node generate-tsconfig.ts \
 *     --project-root <path> \
 *     --output <path> \
 *     --rules-dir <path> \
 *     [--dry-run]
 */
export {};
