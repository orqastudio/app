/**
 * Schema validation command — validates project.json and orqa-plugin.json
 * against their known schemas.
 *
 * orqa enforce schema [path] [--json]
 *
 * Called as a subcommand of `orqa enforce`.
 */
/**
 * Dispatch the validate-schema command: validate project.json and plugin manifests.
 * @param args - CLI arguments after "enforce schema".
 */
export declare function runValidateSchemaCommand(args: string[]): Promise<void>;
//# sourceMappingURL=validate-schema.d.ts.map