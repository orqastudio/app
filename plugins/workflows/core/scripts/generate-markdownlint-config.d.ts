/**
 * markdownlint config generator for the OrqaStudio enforcement pipeline.
 *
 * Scans all rule files under --rules-dir, filters enforcement entries where
 * engine=markdownlint, and produces a self-contained .markdownlint.json at
 * --output.
 *
 * Each enforcement entry may specify:
 *   - `rule`: the markdownlint rule name or alias (e.g., "MD013", "line-length")
 *   - `enabled`: boolean — true to enable, false to disable the rule
 *   - `options`: a mapping of rule-specific configuration values
 *
 * The generator starts from conservative OrqaStudio defaults and applies
 * overrides from rule files. Later entries for the same rule win.
 *
 * Invocation:
 *   node generate-markdownlint-config.ts \
 *     --project-root <path> \
 *     --output <path> \
 *     --rules-dir <path> \
 *     [--dry-run]
 */
export {};
