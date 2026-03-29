/**
 * Summarize command — generates structured summaries for knowledge artifacts.
 *
 * orqa summarize <path>         Summarize a single knowledge artifact
 * orqa summarize --all          Summarize all knowledge artifacts in .orqa/
 * orqa summarize --check        Check which artifacts are missing summaries
 *
 * Generates template-based summaries (100-150 token target) and writes them
 * to the artifact's YAML frontmatter `summary` field.
 */
/**
 * Dispatch the summarize command: generate or check summaries for knowledge artifacts.
 * @param args - CLI arguments after "summarize".
 */
export declare function runSummarizeCommand(args: string[]): Promise<void>;
//# sourceMappingURL=summarize.d.ts.map