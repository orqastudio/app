/**
 * Index command — triggers the daemon to index the codebase and generate
 * embeddings for semantic search.
 *
 * The daemon owns search state. This command calls daemon HTTP endpoints
 * instead of running the binary directly. The daemon must be running.
 *
 * orqa index [project-path] [--download-only] [--skip-download] [--status]
 */
/**
 * Dispatch the index command: call daemon HTTP endpoints to trigger indexing.
 * @param args - CLI arguments after "index".
 */
export declare function runIndexCommand(args: string[]): Promise<void>;
//# sourceMappingURL=index.d.ts.map