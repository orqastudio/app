/**
 * Graph browsing command — scan and query the artifact graph from CLI.
 *
 * orqa graph [options]
 *
 * Delegates to the orqa-validation daemon for all graph operations.
 * Falls back to the orqa-validation binary when the daemon is unreachable.
 */
/**
 * Dispatch the graph command: list, tree, or artifact detail views.
 * @param args - CLI arguments after "graph".
 */
export declare function runGraphCommand(args: string[]): Promise<void>;
//# sourceMappingURL=graph.d.ts.map