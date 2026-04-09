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
import { execSync } from "node:child_process";
import * as path from "node:path";
import { getRoot } from "../lib/root.js";
import { getFixedPort } from "@orqastudio/constants";
const COMPOSE_FILE = "infrastructure/orqastudio-git/docker-compose.yml";
const SETUP_SCRIPT = "infrastructure/orqastudio-git/setup.sh";
const USAGE = `
Usage: orqa hosting <subcommand>

Subcommands:
  up        (use 'orqa dev' — Forgejo starts automatically as part of the dev stack)
  down      (use 'orqa dev kill' — stops all dev services including Forgejo)
  setup     First-time setup (admin user, org, repo, push)
  status    Show server container status
  logs      Follow server logs (Ctrl+C to stop)
  push      Push monorepo to the local server
  mirror    Show instructions for configuring a push mirror to GitHub

Options:
  --help, -h  Show this help message
`.trim();
function composeCmd(root, subArgs) {
    const composePath = path.join(root, COMPOSE_FILE);
    return `docker compose -f "${composePath}" ${subArgs}`;
}
function run(cmd, root) {
    execSync(cmd, { cwd: root, stdio: "inherit" });
}
/**
 * Dispatch the hosting command: manage the local Forgejo git server.
 * @param args - CLI arguments after "hosting".
 */
export async function runHostingCommand(args) {
    const subcommand = args[0];
    if (!subcommand || subcommand === "--help" || subcommand === "-h") {
        console.log(USAGE);
        return;
    }
    const root = getRoot();
    const forgejoPort = getFixedPort("forgejo_http");
    switch (subcommand) {
        case "up":
            console.log(`Forgejo is now managed by 'orqa dev' and starts automatically with the dev stack.\n` +
                `Run 'orqa dev' to start all dev services including the git server.\n` +
                `Web UI will be available at http://localhost:${forgejoPort} once the stack is up.`);
            break;
        case "down":
            console.log(`Forgejo is now managed by 'orqa dev'.\n` +
                `Run 'orqa dev kill' to stop all dev services including the git server.`);
            break;
        case "setup": {
            console.log("Running first-time server setup...");
            const scriptPath = path.join(root, SETUP_SCRIPT);
            run(`bash "${scriptPath}"`, root);
            break;
        }
        case "status":
            run(composeCmd(root, "ps"), root);
            break;
        case "logs":
            try {
                run(composeCmd(root, "logs -f"), root);
            }
            catch {
                // User pressed Ctrl+C — expected exit
            }
            break;
        case "push":
            console.log("Pushing monorepo to local git server...");
            run("git push local main", root);
            console.log("Push complete.");
            break;
        case "mirror":
            console.log(`
Push Mirror Setup
=================

A push mirror replicates commits from the local git server to GitHub automatically.
This must be configured via the server web UI because it requires a GitHub token.

Steps:
  1. Open the server web UI: http://localhost:${forgejoPort}
  2. Navigate to the repository settings
  3. Click "Mirror Settings"
  4. Add a push mirror:
     - URL:  https://github.com/<org>/<repo>.git
     - Auth: GitHub personal access token with 'repo' scope
  5. Set sync interval (e.g. every push, or hourly)

Note: The target repository must already exist on GitHub.
`.trim());
            break;
        default:
            console.error(`Unknown hosting subcommand: ${subcommand}`);
            console.error("Run 'orqa hosting --help' for available subcommands.");
            process.exit(1);
    }
}
//# sourceMappingURL=hosting.js.map