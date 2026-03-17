/**
 * Plugin management commands.
 *
 * orqa plugin list|install|uninstall|update|registry|create
 */

import { installPlugin, uninstallPlugin, listInstalledPlugins } from "../lib/installer.js";
import { fetchRegistry, searchRegistry } from "../lib/registry.js";

const USAGE = `
Usage: orqa plugin <subcommand> [options]

Subcommands:
  list                              List installed plugins
  install <owner/repo|path> [-v]    Install a plugin
  uninstall <name>                  Remove a plugin
  update [name]                     Update one or all plugins
  registry [--official|--community] Browse available plugins
  create [template]                 Scaffold a new plugin from template
`.trim();

export async function runPluginCommand(args: string[]): Promise<void> {
	const subcommand = args[0];

	if (!subcommand || subcommand === "--help" || subcommand === "-h") {
		console.log(USAGE);
		return;
	}

	switch (subcommand) {
		case "list":
			await cmdList();
			break;
		case "install":
			await cmdInstall(args.slice(1));
			break;
		case "uninstall":
			await cmdUninstall(args.slice(1));
			break;
		case "update":
			await cmdUpdate(args.slice(1));
			break;
		case "registry":
			await cmdRegistry(args.slice(1));
			break;
		case "create":
			await cmdCreate(args.slice(1));
			break;
		default:
			console.error(`Unknown subcommand: ${subcommand}`);
			console.error(USAGE);
			process.exit(1);
	}
}

async function cmdList(): Promise<void> {
	const plugins = listInstalledPlugins();

	if (plugins.length === 0) {
		console.log("No plugins installed.");
		return;
	}

	console.log("Installed plugins:\n");
	for (const p of plugins) {
		console.log(`  ${p.name} @ ${p.version} (${p.source})`);
		console.log(`    ${p.path}`);
	}
}

async function cmdInstall(args: string[]): Promise<void> {
	if (args.length === 0) {
		console.error("Usage: orqa plugin install <owner/repo|path> [--version <tag>]");
		process.exit(1);
	}

	const source = args[0];
	const versionIdx = args.indexOf("--version");
	const version = versionIdx >= 0 ? args[versionIdx + 1] : undefined;

	const result = await installPlugin({ source, version });
	console.log(`\nInstalled: ${result.name} @ ${result.version}`);
	console.log(`Path: ${result.path}`);
}

async function cmdUninstall(args: string[]): Promise<void> {
	if (args.length === 0) {
		console.error("Usage: orqa plugin uninstall <name>");
		process.exit(1);
	}

	uninstallPlugin(args[0]);
}

async function cmdUpdate(args: string[]): Promise<void> {
	const name = args[0];
	const plugins = listInstalledPlugins().filter(
		(p) => p.source === "github" && (!name || p.name === name),
	);

	if (plugins.length === 0) {
		console.log(name ? `Plugin not found: ${name}` : "No updatable plugins.");
		return;
	}

	// Re-install from the same repo (fetches latest)
	for (const p of plugins) {
		console.log(`Checking ${p.name}...`);
		// The lockfile has the repo — read it
		const { readLockfile } = await import("../lib/lockfile.js");
		const lockfile = readLockfile(process.cwd());
		const locked = lockfile.plugins.find((l) => l.name === p.name);
		if (locked) {
			await installPlugin({ source: locked.repo });
		}
	}
}

async function cmdRegistry(args: string[]): Promise<void> {
	const source = args.includes("--official")
		? "official" as const
		: args.includes("--community")
			? "community" as const
			: "all" as const;

	const searchTerm = args.find((a) => !a.startsWith("--"));

	try {
		if (searchTerm) {
			const results = await searchRegistry(searchTerm, source);
			if (results.length === 0) {
				console.log(`No plugins matching "${searchTerm}" found.`);
				return;
			}
			printRegistryResults(results);
		} else {
			const catalog = await fetchRegistry(source);
			if (catalog.plugins.length === 0) {
				console.log("No plugins available yet.");
				return;
			}
			printRegistryResults(catalog.plugins);
		}
	} catch (err) {
		console.error(
			`Failed to fetch registry: ${err instanceof Error ? err.message : err}`,
		);
		process.exit(1);
	}
}

function printRegistryResults(
	plugins: Array<{
		name: string;
		displayName: string;
		description: string;
		category: string;
	}>,
): void {
	console.log("Available plugins:\n");
	for (const p of plugins) {
		console.log(`  ${p.displayName} (${p.name})`);
		console.log(`    ${p.description}`);
		console.log(`    Category: ${p.category}`);
		console.log();
	}
}

async function cmdCreate(args: string[]): Promise<void> {
	const template = args[0] ?? "full";
	const validTemplates = ["frontend", "sidecar", "cli-tool", "full"];

	if (!validTemplates.includes(template)) {
		console.error(`Invalid template: ${template}`);
		console.error(`Valid templates: ${validTemplates.join(", ")}`);
		process.exit(1);
	}

	// Phase 8: will scaffold from templates/
	console.log(`Scaffolding plugin from '${template}' template...`);
	console.log("(Template system not yet implemented — coming in Phase 8)");
}
