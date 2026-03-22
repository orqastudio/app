/**
 * Validation command — delegates to the Rust orqa-validation binary.
 *
 * The Rust binary is the single source of truth for validation. It reads
 * schemas from plugin manifests, validates frontmatter, relationships,
 * and graph integrity.
 *
 * Falls back to the TypeScript validator if the binary is not built.
 *
 * orqa validate [path] [--json] [--fix]
 */
export declare function runValidateCommand(args: string[]): Promise<void>;
//# sourceMappingURL=validate.d.ts.map