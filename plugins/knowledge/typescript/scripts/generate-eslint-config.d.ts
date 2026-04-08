/**
 * ESLint config generator for the OrqaStudio enforcement pipeline.
 *
 * Scans all rule files under --rules-dir, filters enforcement entries where
 * engine=eslint, and produces a self-contained eslint.config.js at --output.
 *
 * The generated config is functionally equivalent to app/eslint.config.js:
 * - typescript-eslint recommended rules
 * - eslint-plugin-svelte flat/recommended for .svelte files
 * - eslint-plugin-jsdoc flat/recommended-typescript
 * - Per-rule overrides from enforcement entries
 *
 * Invocation:
 *   node generate-eslint-config.ts \
 *     --project-root <path> \
 *     --output <path> \
 *     --rules-dir <path> \
 *     [--dry-run]
 */
export {};
