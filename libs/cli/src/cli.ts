#!/usr/bin/env node

/**
 * OrqaStudio CLI — general-purpose command-line interface.
 *
 * Usage:
 *   orqa plugin list                              List installed plugins
 *   orqa plugin install <owner/repo> [--version]  Install from GitHub release
 *   orqa plugin install <path>                    Install from local path
 *   orqa plugin uninstall <name>                  Remove a plugin
 *   orqa plugin update [name]                     Update one or all plugins
 *   orqa plugin registry [--official|--community] Browse available plugins
 *   orqa plugin create [template]                 Scaffold from template
 *   orqa validate [path]                          Run integrity check
 *   orqa debug [command]                          Run debug tool
 *   orqa graph [--type <type>] [--status <s>]     Browse the artifact graph
 */

import { parseArgs } from "node:util";
import { runPluginCommand } from "./commands/plugin.js";
import { runValidateCommand } from "./commands/validate.js";
import { runDebugCommand } from "./commands/debug.js";
import { runGraphCommand } from "./commands/graph.js";

const USAGE = `
OrqaStudio CLI v0.1.0

Usage: orqa <command> [options]

Commands:
  plugin      Plugin management (install, uninstall, list, update, registry, create)
  validate    Run integrity validation on the current project
  debug       Run the debug tool
  graph       Browse the artifact graph

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
		console.log("0.1.0-dev");
		return;
	}

	const command = args[0];
	const commandArgs = args.slice(1);

	switch (command) {
		case "plugin":
			await runPluginCommand(commandArgs);
			break;
		case "validate":
			await runValidateCommand(commandArgs);
			break;
		case "debug":
			await runDebugCommand(commandArgs);
			break;
		case "graph":
			await runGraphCommand(commandArgs);
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
