/**
 * Git commands — monorepo-aware git operations.
 *
 * orqa git status    Show which components have changes
 * orqa git pr        Create a pull request on the local git server
 * orqa git sync      Push to all remotes
 * orqa git audit     Check git infrastructure health
 * orqa git license   Audit LICENSE files across all repos
 * orqa git readme    Audit README.md files across all repos
 * orqa git hosting   Local git server management
 */
/**
 * Dispatch the git command: status, log, diff, pr subcommands.
 * @param args - CLI arguments after "git".
 */
export declare function runGitCommand(args: string[]): Promise<void>;
//# sourceMappingURL=git.d.ts.map