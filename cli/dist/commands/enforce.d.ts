/**
 * Enforcement command — dynamic plugin-dispatch enforcement entry point.
 *
 * Reads all installed plugin manifests, builds an engine registry from their
 * enforcement declarations, and dispatches to each registered engine.
 *
 * orqa enforce                         Run ALL registered enforcement engines
 * orqa enforce --staged                Run all engines on staged files only (git hooks)
 * orqa enforce --<engine>              Run a specific engine (e.g. --eslint, --clippy)
 * orqa enforce --<engine> --fix        Run a specific engine in fix mode
 * orqa enforce --report                Enforcement coverage report
 * orqa enforce --json                  JSON output for report/metrics subcommands
 * orqa enforce response ...            Log an agent's response to an enforcement event
 * orqa enforce schema ...              Validate project.json and plugin manifests
 * orqa enforce test ...                Run enforcement tests defined in rules
 * orqa enforce override ...            Request enforcement override (requires human approval)
 * orqa enforce approve <code>          Approve an override request
 * orqa enforce metrics                 Show per-rule enforcement metrics
 */
/**
 * Dispatch the enforce command. Returns an exit code (0 = all passed, 1 = failure).
 * @param projectRoot - Absolute path to the project root.
 * @param args - CLI arguments after "enforce".
 * @returns 0 if all checks passed, 1 if any failed.
 */
export declare function runEnforceCommand(projectRoot: string, args: string[]): Promise<number>;
//# sourceMappingURL=enforce.d.ts.map