/**
 * Hosting commands — local git server management.
 *
 * orqa hosting up       Start the local git server
 * orqa hosting down     Stop the local git server
 * orqa hosting setup    First-time setup (admin user, org, repo, push)
 * orqa hosting status   Show server status
 * orqa hosting logs     Show server logs
 * orqa hosting push     Push monorepo to the local server
 * orqa hosting mirror   Show instructions for configuring a push mirror
 */
/**
 * Dispatch the hosting command: manage the local Forgejo git server.
 * @param args - CLI arguments after "hosting".
 */
export declare function runHostingCommand(args: string[]): Promise<void>;
//# sourceMappingURL=hosting.d.ts.map