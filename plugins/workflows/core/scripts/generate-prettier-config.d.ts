/**
 * Prettier config generator for the OrqaStudio enforcement pipeline.
 *
 * Scans all rule files under --rules-dir, filters enforcement entries where
 * engine=prettier, and produces a self-contained .prettierrc JSON at --output.
 *
 * Each enforcement entry may specify:
 *   - `option`: the Prettier option name (e.g., "printWidth", "tabWidth")
 *   - `value`: the value to set
 *
 * The generator starts from canonical OrqaStudio defaults and applies any
 * per-option overrides found in rule files. Later entries for the same option
 * win.
 *
 * Invocation:
 *   node generate-prettier-config.ts \
 *     --project-root <path> \
 *     --output <path> \
 *     --rules-dir <path> \
 *     [--dry-run]
 */
export {};
