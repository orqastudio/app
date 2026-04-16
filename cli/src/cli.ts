#!/usr/bin/env node

/**
 * OrqaStudio CLI — general-purpose command-line interface.
 *
 * 16 primary commands. No legacy aliases — pre-release, breaking changes expected.
 */

import { createRequire } from "node:module";
import { runPluginCommand } from "./commands/plugin.js";
import { runIdCommand } from "./commands/id.js";
import { runMcpCommand } from "./commands/mcp.js";
import { runLspCommand } from "./commands/lsp.js";
import { runDevCommand } from "./commands/dev.js";
import { runGraphCommand } from "./commands/graph.js";
import { runVersionCommand } from "./commands/version.js";
import { runInstallCommand } from "./commands/install.js";
import { runCheckCommand } from "./commands/check.js";
import { runTestCommand } from "./commands/test.js";
import { runDaemonCommand } from "./commands/daemon.js";
import { runGitCommand } from "./commands/git.js";
import { runBuildCommand } from "./commands/build.js";
import { runSummarizeCommand } from "./commands/summarize.js";
import { runMetricsCommand } from "./commands/metrics.js";
import { runMigrateCommand } from "./commands/migrate.js";
import { runEnforceCommand } from "./commands/enforce.js";
import { runImportCommand } from "./commands/import.js";

/** Read version dynamically from package.json so it never drifts from the published value. */
const require = createRequire(import.meta.url);
const pkg = require("../package.json") as { version: string };
const VERSION = pkg.version;

const USAGE = `
OrqaStudio CLI v${VERSION}

Usage: orqa <command> [options]

Commands:
  install     Dev environment setup (prereqs, submodules, deps, link)
  plugin      Plugin management (install, uninstall, list, update, link)
  enforce     Run enforcement checks dispatched from installed plugin manifests
  check       Code quality + governance (lint, format, validate, verify, audit)
  test        Run test suites (rust, app)
  build       Production build (full, rust, app)
  graph       Browse the artifact graph
  import      Import a directory of markdown artifacts into SurrealDB
  daemon      Manage the validation daemon (start, stop, restart, status)
  mcp         MCP server + search indexing (index)
  metrics     Token usage and cost metrics
  summarize   Generate knowledge artifact summaries (single, --all, --check)
  lsp         Start LSP server
  version     Version management (sync, bump, check, show)
  id          Artifact ID management (generate, check, migrate)
  migrate     Apply status migrations from workflow definitions
  git         Git operations + repo maintenance (status, pr, license, readme, hosting)
  dev         Dev environment + debug tooling (stop, kill, restart, icons, tool)

Options:
  --help, -h     Show this help message
  --version, -v  Show version

Run 'orqa <command> --help' for more information on a specific command.
`.trim();

async function main(): Promise<void> {
	const args = process.argv.slice(2);

	if (args.length === 0 || args[0] === "--help" || args[0] === "-h") {
		console.log(USAGE);
		return;
	}

	if (args[0] === "--version" || args[0] === "-v") {
		console.log(VERSION);
		return;
	}

	const command = args[0];
	const commandArgs = args.slice(1);

	switch (command) {
		case "install":
			await runInstallCommand(commandArgs);
			break;
		case "plugin":
			await runPluginCommand(commandArgs);
			break;
		case "enforce": {
			const root = (await import("./lib/root.js")).getRoot();
			const enforceExit = await runEnforceCommand(root, commandArgs);
			if (enforceExit !== 0) process.exit(enforceExit);
			break;
		}
		case "check":
			await runCheckCommand(commandArgs);
			break;
		case "test":
			await runTestCommand(commandArgs);
			break;
		case "build":
			await runBuildCommand(commandArgs);
			break;
		case "graph":
			await runGraphCommand(commandArgs);
			break;
		case "daemon":
			await runDaemonCommand(commandArgs);
			break;
		case "mcp":
			await runMcpCommand(commandArgs);
			break;
		case "metrics":
			await runMetricsCommand(commandArgs);
			break;
		case "summarize":
			await runSummarizeCommand(commandArgs);
			break;
		case "lsp":
			await runLspCommand(commandArgs);
			break;
		case "version":
			await runVersionCommand(commandArgs);
			break;
		case "id":
			await runIdCommand(commandArgs);
			break;
		case "migrate":
			await runMigrateCommand(commandArgs);
			break;
		case "import":
			await runImportCommand(commandArgs);
			break;
		case "git":
			await runGitCommand(commandArgs);
			break;
		case "dev":
			await runDevCommand(commandArgs);
			break;

		default:
			console.error(`Unknown command: ${command}`);
			console.error("Run 'orqa --help' for available commands.");
			process.exit(1);
	}
}

main().catch((err) => {
	console.error("Fatal error:", err instanceof Error ? err.message : err);
	process.exit(1);
});
