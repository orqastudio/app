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

import { execSync } from "node:child_process";
import * as path from "node:path";
import { getRoot } from "../lib/root.js";

const COMPOSE_FILE = "infrastructure/orqastudio-git/docker-compose.yml";
const SETUP_SCRIPT = "infrastructure/orqastudio-git/setup.sh";

const USAGE = `
Usage: orqa hosting <subcommand>

Subcommands:
  up        Start the local git server
  down      Stop the local git server
  setup     First-time setup (admin user, org, repo, push)
  status    Show server container status
  logs      Follow server logs (Ctrl+C to stop)
  push      Push monorepo to the local server
  mirror    Show instructions for configuring a push mirror to GitHub

Options:
  --help, -h  Show this help message
`.trim();

function composeCmd(root: string, subArgs: string): string {
	const composePath = path.join(root, COMPOSE_FILE);
	return `docker compose -f "${composePath}" ${subArgs}`;
}

function run(cmd: string, root: string): void {
	execSync(cmd, { cwd: root, stdio: "inherit" });
}

export async function runHostingCommand(args: string[]): Promise<void> {
	const subcommand = args[0];

	if (!subcommand || subcommand === "--help" || subcommand === "-h") {
		console.log(USAGE);
		return;
	}

	const root = getRoot();

	switch (subcommand) {
		case "up":
			console.log("Starting local git server...");
			run(composeCmd(root, "up -d"), root);
			console.log("\nServer started. Web UI: http://localhost:10030");
			break;

		case "down":
			console.log("Stopping local git server...");
			run(composeCmd(root, "down"), root);
			console.log("Server stopped.");
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
			} catch {
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
  1. Open the server web UI: http://localhost:10030
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
