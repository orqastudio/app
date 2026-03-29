/**
 * Code quality checks — CLI adapter for the shared validation engine +
 * plugin-provided lint tools.
 *
 * orqa check              Run validation engine + all plugin lint tools
 * orqa check validate     Run the shared validation engine only
 * orqa check <tool>       Run a specific plugin tool (eslint, clippy, etc.)
 * orqa check configure    Generate config files from coding standards rules
 * orqa check verify       Run all governance checks (integrity, version, license, readme)
 * orqa check enforce      Enforcement + integrity validation
 * orqa check audit        Full governance audit with escalation scanning
 * orqa check schema       Validate project.json and plugin manifests
 *
 * The validation engine (engine/validation/) runs the same checks as the LSP:
 * schema validation, relationship type checks, broken links, missing inverses,
 * status transitions, and more. Plugin tools (eslint, clippy, etc.) are
 * discovered from installed plugin manifests (orqa-plugin.json).
 */
/**
 * Dispatch the check command: validation engine, plugin lint tools, or a specific subcommand.
 * @param args - CLI arguments after "check".
 */
export declare function runCheckCommand(args: string[]): Promise<void>;
//# sourceMappingURL=check.d.ts.map