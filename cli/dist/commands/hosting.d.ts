/**
 * Hosting commands — local git server management.
 *
 * orqa hosting up       (redirects to orqa dev — Forgejo is now part of the dev stack)
 * orqa hosting down     (redirects to orqa dev — use orqa dev kill to stop everything)
 * orqa hosting setup    First-time setup (admin user, org, repo, push)
 * orqa hosting status   Show server container status
 * orqa hosting logs     Follow server logs
 * orqa hosting push     Push monorepo to the local server
 * orqa hosting mirror   Show instructions for configuring a push mirror
 */
/**
 * Dispatch the hosting command: manage the local Forgejo git server.
 * @param args - CLI arguments after "hosting".
 */
export declare function runHostingCommand(args: string[]): Promise<void>;
//# sourceMappingURL=hosting.d.ts.map