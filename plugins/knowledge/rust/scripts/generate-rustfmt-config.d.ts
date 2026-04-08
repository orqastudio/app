/**
 * rustfmt config generator for the OrqaStudio enforcement pipeline.
 *
 * Scans all rule files under --rules-dir, filters enforcement entries where
 * engine=rustfmt, and produces a self-contained .rustfmt.toml at --output.
 *
 * Each enforcement entry may specify:
 *   - `option`: the rustfmt option name (e.g., "max_width")
 *   - `value`: the value to set (string, number, or boolean)
 *
 * The generator produces a flat TOML file. Options from rule files augment
 * the canonical defaults below. Later entries override earlier entries for
 * the same option.
 *
 * Invocation:
 *   node generate-rustfmt-config.ts \
 *     --project-root <path> \
 *     --output <path> \
 *     --rules-dir <path> \
 *     [--dry-run]
 */
export {};
