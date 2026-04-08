/**
 * Clippy config generator for the OrqaStudio enforcement pipeline.
 *
 * Scans all rule files under --rules-dir, filters enforcement entries where
 * engine=clippy, and produces a self-contained clippy.toml at --output.
 *
 * Each enforcement entry may specify:
 *   - `lint`: the clippy lint name (e.g., "clippy::unwrap_used")
 *   - `level`: "deny", "warn", or "allow"
 *
 * The generator produces a flat TOML file. Clippy TOML supports `warn`,
 * `deny`, and `allow` arrays at the top level. Lints without a `level` field
 * default to "warn".
 *
 * Invocation:
 *   node generate-clippy-config.ts \
 *     --project-root <path> \
 *     --output <path> \
 *     --rules-dir <path> \
 *     [--dry-run]
 */
export {};
