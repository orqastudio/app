/**
 * Plugin management commands.
 *
 * orqa plugin list|install|uninstall|update|enable|disable|refresh|diff|registry|create
 */

import * as fs from "node:fs";
import * as path from "node:path";
import { installPlugin, uninstallPlugin, listInstalledPlugins, detectMethodologyConflict } from "../lib/installer.js";
import { fetchRegistry, searchRegistry } from "../lib/registry.js";
import { readManifest } from "../lib/manifest.js";
import {
	readContentManifest,
	writeContentManifest,
	copyPluginContent,
	removePluginContent,
	installPluginDeps,
	buildPlugin,
	runLifecycleHook,
	diffPluginContent,
	computeThreeWayState,
	findSourceFile,
	processAggregatedFiles,
	computeFileHash,
} from "../lib/content-lifecycle.js";
import type { ContentManifest, FileHashEntry, ThreeWayFileStatus } from "../lib/content-lifecycle.js";
import { createHash } from "node:crypto";
import { runWorkflowResolution } from "../lib/workflow-resolver.js";
import { writeComposedSchema } from "../lib/schema-composer.js";
import { generatePromptRegistry } from "../lib/prompt-registry.js";
import { runPluginGenerators } from "./install.js";
import type { PluginProjectConfig, PluginManifest } from "@orqastudio/types";

const USAGE = `
Usage: orqa plugin <subcommand> [options]

Subcommands:
  list                              List installed plugins
  install <owner/repo|path> [-v]    Install a plugin
  uninstall <name>                  Remove a plugin
  update [name]                     Update one or all plugins
  outdated                          List plugins whose source version or manifest has changed
  enable <name>                     Enable a plugin (copy content to .orqa/)
  disable <name>                    Disable a plugin (remove content from .orqa/)
  refresh [name]                    Re-sync content for one or all enabled plugins
  diff [name]                       Show content drift for one or all installed plugins
  status [name]                     Show three-way state for tracked files
  registry [--official|--community] Browse available plugins
  create [template]                 Scaffold a new plugin from template
  template-validate [template]      Validate a plugin template directory
  link [create|verify|remove|status] Cross-platform symlink management
`.trim();

/**
 * Dispatch the plugin command: install, uninstall, list, refresh, create, link.
 * @param args - CLI arguments after "plugin".
 */
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
		case "outdated":
			await cmdOutdated();
			break;
		case "enable":
			await cmdEnable(args.slice(1));
			break;
		case "disable":
			await cmdDisable(args.slice(1));
			break;
		case "refresh":
			await cmdRefresh(args.slice(1));
			break;
		case "diff":
			await cmdDiff(args.slice(1));
			break;
		case "registry":
			await cmdRegistry(args.slice(1));
			break;
		case "status":
			await cmdStatus(args.slice(1));
			break;
		case "create":
			await cmdCreate(args.slice(1));
			break;
		case "template-validate":
			await cmdTemplateValidate(args.slice(1));
			break;
		case "link": {
			const { runLinkCommand } = await import("./link.js");
			await runLinkCommand(args.slice(1));
			break;
		}
		default:
			console.error(`Unknown subcommand: ${subcommand}`);
			console.error(USAGE);
			process.exit(1);
	}
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/**
 * Read and parse .orqa/project.json.
 * @param projectRoot - Absolute path to the project root.
 * @returns Parsed project.json as a plain object.
 */
export function readProjectJson(projectRoot: string): Record<string, unknown> {
	const p = path.join(projectRoot, ".orqa", "project.json");
	const raw = fs.readFileSync(p, "utf-8");
	return JSON.parse(raw) as Record<string, unknown>;
}

function writeProjectJson(projectRoot: string, data: Record<string, unknown>): void {
	const p = path.join(projectRoot, ".orqa", "project.json");
	fs.writeFileSync(p, JSON.stringify(data, null, 2) + "\n", "utf-8");
}

/**
 * Look up a plugin directory by name. Checks:
 *   1. plugins/<short-name>/orqa-plugin.json
 *   2. connectors/<short-name>/orqa-plugin.json
 *   3. .orqa/project.json plugins section for the path field
 *
 * Returns the absolute plugin directory path or null if not found.
 * @param name - Plugin name (short or scoped, e.g. "software" or "@orqastudio/software").
 * @param projectRoot - Absolute path to the project root.
 * @returns Absolute plugin directory path, or null if not found.
 */
function resolvePluginDir(name: string, projectRoot: string): string | null {
	const shortName = name.replace(/^@[^/]+\//, "");

	// 1. plugins/ directory
	const pluginsDir = path.join(projectRoot, "plugins", shortName);
	if (fs.existsSync(path.join(pluginsDir, "orqa-plugin.json"))) {
		return pluginsDir;
	}

	// 2. connectors/ directory
	const connectorsDir = path.join(projectRoot, "connectors", shortName);
	if (fs.existsSync(path.join(connectorsDir, "orqa-plugin.json"))) {
		return connectorsDir;
	}

	// 3. project.json plugins section
	const projectJsonPath = path.join(projectRoot, ".orqa", "project.json");
	if (fs.existsSync(projectJsonPath)) {
		try {
			const projectJson = readProjectJson(projectRoot);
			const pluginsSection = projectJson["plugins"] as
				| Record<string, { path?: string }>
				| undefined;
			if (pluginsSection) {
				const pluginConfig = pluginsSection[name];
				if (pluginConfig?.path) {
					const resolved = path.isAbsolute(pluginConfig.path)
						? pluginConfig.path
						: path.join(projectRoot, pluginConfig.path);
					if (fs.existsSync(path.join(resolved, "orqa-plugin.json"))) {
						return resolved;
					}
				}
			}
		} catch {
			// project.json not parseable — fall through
		}
	}

	return null;
}

/**
 * Update the plugins section of .orqa/project.json for a single plugin.
 * Merges with any existing entry.
 * @param projectRoot - Absolute path to the project root.
 * @param name - Plugin name as it appears in project.json.
 * @param updates - Fields to merge into the existing plugin entry.
 */
export function updateProjectJsonPlugin(
	projectRoot: string,
	name: string,
	updates: Partial<PluginProjectConfig>,
): void {
	const projectJsonPath = path.join(projectRoot, ".orqa", "project.json");

	if (!fs.existsSync(projectJsonPath)) {
		throw new Error(`project.json not found at ${projectJsonPath}`);
	}

	const data = readProjectJson(projectRoot);
	const plugins = (data["plugins"] ?? {}) as Record<string, Partial<PluginProjectConfig>>;
	const existing = plugins[name] ?? {};
	plugins[name] = { ...existing, ...updates };
	data["plugins"] = plugins;
	writeProjectJson(projectRoot, data);
}

/**
 * Remove a plugin entry from .orqa/project.json.
 * @param projectRoot - Absolute path to the project root.
 * @param name - Plugin name to remove from project.json.
 */
function removeProjectJsonPlugin(projectRoot: string, name: string): void {
	const projectJsonPath = path.join(projectRoot, ".orqa", "project.json");

	if (!fs.existsSync(projectJsonPath)) {
		return;
	}

	const data = readProjectJson(projectRoot);
	const plugins = (data["plugins"] ?? {}) as Record<string, unknown>;
	delete plugins[name];
	data["plugins"] = plugins;
	writeProjectJson(projectRoot, data);
}

/**
 * Delete all files listed in the content manifest entry for a plugin without
 * removing the manifest entry itself (used by disable — keeps entry for re-enable).
 * @param projectRoot - Absolute path to the project root.
 * @param pluginName - Plugin name whose content files should be deleted.
 */
function deleteContentFiles(projectRoot: string, pluginName: string): void {
	const contentManifest = readContentManifest(projectRoot);
	const entry = contentManifest.plugins[pluginName];

	if (!entry) {
		return;
	}

	for (const relPath of Object.keys(entry.files)) {
		const absPath = path.join(projectRoot, relPath);
		if (fs.existsSync(absPath)) {
			fs.unlinkSync(absPath);
		}
	}
}

// ---------------------------------------------------------------------------
// list
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// install
// ---------------------------------------------------------------------------

async function cmdInstall(args: string[]): Promise<void> {
	if (args.length === 0) {
		console.error("Usage: orqa plugin install <owner/repo|path> [--version <tag>]");
		process.exit(1);
	}

	const source = args[0];
	const versionIdx = args.indexOf("--version");
	const version = versionIdx >= 0 ? args[versionIdx + 1] : undefined;

	const projectRoot = process.cwd();

	// Check if this is a first-party plugin already inside the project
	const absSource = path.resolve(source);
	const isFirstParty =
		absSource.startsWith(path.join(projectRoot, "plugins")) ||
		absSource.startsWith(path.join(projectRoot, "connectors"));

	if (isFirstParty && fs.existsSync(path.join(absSource, "orqa-plugin.json"))) {
		await cmdInstallFirstParty(absSource, projectRoot);
		return;
	}

	const result = await installPlugin({ source, version, projectRoot });

	if (result.collisions.length > 0) {
		console.log(`\nInstalled: ${result.name} @ ${result.version}`);
		console.log(`Path: ${result.path}`);
		console.log(`\n${result.collisions.length} relationship key collision(s) detected:\n`);

		for (const c of result.collisions) {
			console.log(`  Key: "${c.key}"`);
			console.log(`    Existing (${c.existingSource}): ${c.existingDescription || "(no description)"}`);
			console.log(`      semantic: ${c.existingSemantic ?? "none"}, from: [${c.existingFrom.join(", ")}], to: [${c.existingTo.join(", ")}]`);
			console.log(`    Incoming: ${c.incomingDescription || "(no description)"}`);
			console.log(`      semantic: ${c.incomingSemantic ?? "none"}, from: [${c.incomingFrom.join(", ")}], to: [${c.incomingTo.join(", ")}]`);
			console.log(`    Intent match: ${c.semanticMatch ? "YES — same semantic, likely safe to merge" : "NO — different semantic, should rename"}`);
			console.log();
		}

		// Interactive resolution
		const readline = await import("node:readline");
		const rl = readline.createInterface({ input: process.stdin, output: process.stdout });
		const ask = (q: string): Promise<string> => new Promise((resolve) => rl.question(q, resolve));

		const decisions: Array<{ key: string; decision: "merged" | "renamed"; existingSource: string; originalKey?: string }> = [];

		for (const c of result.collisions) {
			const suggestion = c.semanticMatch ? "merge" : "rename";
			const answer = await ask(`  "${c.key}" — [m]erge or [r]ename? (suggested: ${suggestion}) `);
			const choice = answer.trim().toLowerCase();

			if (choice === "r" || choice === "rename") {
				decisions.push({ key: c.key, decision: "renamed", existingSource: c.existingSource, originalKey: c.key });
				console.log(`    -> Will namespace as plugin-specific key\n`);
			} else {
				decisions.push({ key: c.key, decision: "merged", existingSource: c.existingSource });
				console.log(`    -> Will merge from/to constraints\n`);
			}
		}

		rl.close();

		// Write decisions to the installed manifest
		if (decisions.length > 0) {
			const manifestPath = path.join(result.path, "orqa-plugin.json");
			const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf-8")) as Record<string, unknown>;
			manifest["mergeDecisions"] = decisions;
			fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + "\n");
			console.log(`Recorded ${decisions.length} merge decision(s) in plugin manifest.`);
		}
	} else {
		console.log(`\nInstalled: ${result.name} @ ${result.version}`);
		console.log(`Path: ${result.path}`);
	}

	// --- Methodology exclusivity check ---
	const pluginManifest = readManifest(result.path);
	const methodologyConflict = detectMethodologyConflict(pluginManifest, projectRoot);
	if (methodologyConflict) {
		console.error(
			`\nMethodology conflict: role "${methodologyConflict.role}" is already provided by ${methodologyConflict.existingPlugin}.` +
			`\nOnly one plugin per core role is allowed. Uninstall ${methodologyConflict.existingPlugin} first, or choose a different plugin.`,
		);
		// Clean up the just-installed files
		uninstallPlugin(result.name, projectRoot);
		process.exit(1);
	}

	// --- Content lifecycle: post-install steps ---
	const shortPath = path.relative(projectRoot, result.path).replace(/\\/g, "/");

	console.log(`\nRunning post-install lifecycle for ${result.name}...`);

	// Install npm dependencies and build
	installPluginDeps(result.path, pluginManifest);
	buildPlugin(result.path, pluginManifest);

	// Copy content to .orqa/
	const copyResult = copyPluginContent(result.path, projectRoot, pluginManifest);
	const copiedCount = Object.keys(copyResult.copied).length;
	if (copiedCount > 0) {
		console.log(`  Copied ${copiedCount} content file(s) to .orqa/`);
	}

	// Record ownership in .orqa/manifest.json including manifestHash for outdated detection.
	const contentManifest: ContentManifest = readContentManifest(projectRoot);
	const manifestFileInstall = path.join(result.path, "orqa-plugin.json");
	const manifestHashInstall = createHash("sha256")
		.update(fs.readFileSync(manifestFileInstall))
		.digest("hex");
	contentManifest.plugins[result.name] = {
		version: result.version,
		installed_at: new Date().toISOString(),
		manifestHash: manifestHashInstall,
		files: copyResult.copied,
	};
	writeContentManifest(projectRoot, contentManifest);

	// Register in .orqa/project.json — include version so outdated checks work.
	const projectJsonPath = path.join(projectRoot, ".orqa", "project.json");
	if (fs.existsSync(projectJsonPath)) {
		updateProjectJsonPlugin(projectRoot, result.name, {
			installed: true,
			enabled: true,
			path: shortPath,
			version: result.version,
		});
		console.log(`  Registered ${result.name} in .orqa/project.json`);
	}

	// Run install lifecycle hook
	runLifecycleHook(result.path, pluginManifest, "install");

	// Run enforcement generators declared in the plugin manifest so .orqa/configs/
	// is populated immediately after install.
	try {
		runPluginGenerators(result.path, pluginManifest, projectRoot);
	} catch (e) {
		// Non-fatal — generator failure should not block install
		console.error(`  Generator run failed: ${e instanceof Error ? e.message : String(e)}`);
	}

	// P5-28: gate recomposition and workflow resolution on manifest flags.
	const { requiresSchemaRecomposition, requiresEnforcementRegeneration } = result;

	if (requiresSchemaRecomposition) {
		try {
			writeComposedSchema(projectRoot);
		} catch {
			// Non-fatal
		}
	}

	if (requiresSchemaRecomposition || requiresEnforcementRegeneration) {
		try {
			runWorkflowResolution(projectRoot);
		} catch {
			// Non-fatal — workflow resolution is best-effort during install
		}
	}

	// Regenerate prompt registry after schema composition so knowledge declarations are current.
	try {
		generatePromptRegistry(projectRoot);
	} catch {
		// Non-fatal — registry generation is best-effort during install
	}

	console.log(`\nPlugin ${result.name} installed successfully.`);
}

/**
 * Install a first-party plugin (already inside plugins/ or connectors/).
 * Does NOT copy the directory — just registers, copies content, installs deps, builds, runs hooks.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param projectRoot - Absolute path to the project root.
 */
async function cmdInstallFirstParty(pluginDir: string, projectRoot: string): Promise<void> {
	const pluginManifest = readManifest(pluginDir);
	const shortPath = path.relative(projectRoot, pluginDir).replace(/\\/g, "/");

	// --- Methodology exclusivity check ---
	const methodologyConflict = detectMethodologyConflict(pluginManifest, projectRoot);
	if (methodologyConflict) {
		console.error(
			`\nMethodology conflict: role "${methodologyConflict.role}" is already provided by ${methodologyConflict.existingPlugin}.` +
			`\nOnly one plugin per core role is allowed. Uninstall ${methodologyConflict.existingPlugin} first, or choose a different plugin.`,
		);
		process.exit(1);
	}

	console.log(`\nInstalling first-party plugin: ${pluginManifest.name} @ ${pluginManifest.version}`);
	console.log(`Path: ${shortPath}`);

	// Install npm dependencies and build
	installPluginDeps(pluginDir, pluginManifest);
	buildPlugin(pluginDir, pluginManifest);

	// Copy content to .orqa/
	const copyResult = copyPluginContent(pluginDir, projectRoot, pluginManifest);
	const copiedCount = Object.keys(copyResult.copied).length;
	if (copiedCount > 0) {
		console.log(`  Copied ${copiedCount} content file(s) to .orqa/`);
	}

	// Record ownership in .orqa/manifest.json including manifestHash for outdated detection.
	const contentManifest: ContentManifest = readContentManifest(projectRoot);
	const manifestFileFirstParty = path.join(pluginDir, "orqa-plugin.json");
	const manifestHashFirstParty = createHash("sha256")
		.update(fs.readFileSync(manifestFileFirstParty))
		.digest("hex");
	contentManifest.plugins[pluginManifest.name] = {
		version: pluginManifest.version,
		installed_at: new Date().toISOString(),
		manifestHash: manifestHashFirstParty,
		files: copyResult.copied,
	};
	writeContentManifest(projectRoot, contentManifest);

	// Register in .orqa/project.json — include version so outdated checks work.
	updateProjectJsonPlugin(projectRoot, pluginManifest.name, {
		installed: true,
		enabled: true,
		path: shortPath,
		version: pluginManifest.version,
	});
	console.log(`  Registered in .orqa/project.json`);

	// Run install lifecycle hook
	runLifecycleHook(pluginDir, pluginManifest, "install");

	// Run enforcement generators declared in the plugin manifest so .orqa/configs/
	// is populated immediately after install.
	try {
		runPluginGenerators(pluginDir, pluginManifest, projectRoot);
	} catch (e) {
		// Non-fatal — generator failure should not block install
		console.error(`  Generator run failed: ${e instanceof Error ? e.message : String(e)}`);
	}

	// Recompose schema and workflows — always run after install to pick up
	// any new schemas, relationships, or enforcement declarations.
	try {
		writeComposedSchema(projectRoot);
	} catch {
		// Non-fatal
	}

	try {
		runWorkflowResolution(projectRoot);
	} catch {
		// Non-fatal
	}

	// Regenerate prompt registry after schema composition so knowledge declarations are current.
	try {
		generatePromptRegistry(projectRoot);
	} catch {
		// Non-fatal — registry generation is best-effort during install
	}

	console.log(`\nPlugin ${pluginManifest.name} installed successfully.`);
}

// ---------------------------------------------------------------------------
// uninstall
// ---------------------------------------------------------------------------

async function cmdUninstall(args: string[]): Promise<void> {
	if (args.length === 0) {
		console.error("Usage: orqa plugin uninstall <name>");
		process.exit(1);
	}

	const name = args[0];
	const projectRoot = process.cwd();

	// Resolve plugin directory for lifecycle operations
	const pluginDir = resolvePluginDir(name, projectRoot);

	if (pluginDir) {
		let pluginManifest;
		try {
			pluginManifest = readManifest(pluginDir);
		} catch {
			// If manifest is not readable, proceed with best-effort cleanup
		}

		if (pluginManifest) {
			// Run uninstall lifecycle hook before removing anything
			runLifecycleHook(pluginDir, pluginManifest, "uninstall");
		}
	}

	// Remove content from .orqa/ and clear manifest entry
	removePluginContent(name, projectRoot);

	// Remove from .orqa/project.json
	removeProjectJsonPlugin(projectRoot, name);

	// Remove plugin directory and update lockfile — but only for GitHub-installed plugins.
	// First-party plugins (local source) keep their directory in the repo.
	try {
		const { readLockfile } = await import("../lib/lockfile.js");
		const lockfile = readLockfile(projectRoot);
		const locked = lockfile.plugins.find((l) => l.name === name);
		if (locked) {
			uninstallPlugin(name, projectRoot);
		}
	} catch {
		// No lockfile or not in lockfile — skip directory removal
	}

	console.log(`Uninstalled ${name}`);
}

// ---------------------------------------------------------------------------
// update
// ---------------------------------------------------------------------------

async function cmdUpdate(args: string[]): Promise<void> {
	const name = args[0];
	const plugins = listInstalledPlugins().filter(
		(p) => p.source === "github" && (!name || p.name === name),
	);

	if (plugins.length === 0) {
		console.log(name ? `Plugin not found: ${name}` : "No updatable plugins.");
		return;
	}

	const projectRoot = process.cwd();

	for (const p of plugins) {
		console.log(`Checking ${p.name}...`);
		const { readLockfile } = await import("../lib/lockfile.js");
		const lockfile = readLockfile(projectRoot);
		const locked = lockfile.plugins.find((l) => l.name === p.name);
		if (locked) {
			// Re-install from the same repo (fetches latest)
			const result = await installPlugin({ source: locked.repo, projectRoot });

			// Re-sync content
			const pluginManifest = readManifest(result.path);
			const copyResult = copyPluginContent(result.path, projectRoot, pluginManifest);
			const copiedCount = Object.keys(copyResult.copied).length;

			if (copiedCount > 0) {
				console.log(`  Re-synced ${copiedCount} content file(s)`);
			}

			// Update content manifest
			const contentManifest = readContentManifest(projectRoot);
			contentManifest.plugins[result.name] = {
				version: result.version,
				installed_at: new Date().toISOString(),
				files: copyResult.copied,
			};
			writeContentManifest(projectRoot, contentManifest);

			// Run install hook again
			runLifecycleHook(result.path, pluginManifest, "install");

			console.log(`Updated ${result.name} to ${result.version}`);
		}
	}
}

// ---------------------------------------------------------------------------
// outdated
// ---------------------------------------------------------------------------

/**
 * List all installed plugins whose source manifest hash or version differs
 * from what is recorded in .orqa/manifest.json.
 *
 * For each enabled plugin in project.json this command:
 *   1. Reads the current orqa-plugin.json version from the plugin directory.
 *   2. Compares the version against the version recorded in manifest.json.
 *   3. Compares the manifest hash against the manifestHash recorded in manifest.json.
 * Plugins that fail either check are reported as outdated.
 */
async function cmdOutdated(): Promise<void> {
	const projectRoot = process.cwd();

	let projectJson: Record<string, unknown>;
	try {
		projectJson = readProjectJson(projectRoot);
	} catch (e) {
		console.error(`Could not read project.json: ${e instanceof Error ? e.message : String(e)}`);
		process.exit(1);
	}

	const pluginsSection = (projectJson["plugins"] ?? {}) as Record<string, Partial<PluginProjectConfig>>;
	const contentManifest = readContentManifest(projectRoot);

	const outdatedPlugins: Array<{ name: string; installedVersion: string; sourceVersion: string; reason: string }> = [];

	for (const [name, cfg] of Object.entries(pluginsSection)) {
		if (!cfg.path) continue;

		const pluginDir = path.isAbsolute(cfg.path) ? cfg.path : path.join(projectRoot, cfg.path);
		const manifestFile = path.join(pluginDir, "orqa-plugin.json");
		if (!fs.existsSync(manifestFile)) continue;

		let sourceManifest;
		try {
			sourceManifest = readManifest(pluginDir);
		} catch {
			continue;
		}

		const manifestEntry = contentManifest.plugins[name];
		const installedVersion = manifestEntry?.version ?? cfg.version ?? "(unknown)";
		const reasons: string[] = [];

		// Check version mismatch against manifest.json record.
		if (manifestEntry?.version && manifestEntry.version !== sourceManifest.version) {
			reasons.push(`version ${installedVersion} → ${sourceManifest.version}`);
		}

		// Check manifest hash mismatch — detects content changes even without a version bump.
		if (manifestEntry) {
			const currentHash = createHash("sha256")
				.update(fs.readFileSync(manifestFile))
				.digest("hex");
			if (manifestEntry.manifestHash && currentHash !== manifestEntry.manifestHash) {
				reasons.push("manifest content changed");
			} else if (!manifestEntry.manifestHash) {
				// Entry predates manifestHash tracking — flag for refresh.
				reasons.push("no manifest hash (run orqa install to record)");
			}
		} else {
			reasons.push("not in manifest.json (run orqa install to record)");
		}

		if (reasons.length > 0) {
			outdatedPlugins.push({
				name,
				installedVersion,
				sourceVersion: sourceManifest.version,
				reason: reasons.join("; "),
			});
		}
	}

	if (outdatedPlugins.length === 0) {
		console.log("All plugins are up to date.");
		return;
	}

	console.log(`${outdatedPlugins.length} plugin(s) need updating:\n`);
	for (const p of outdatedPlugins) {
		console.log(`  ${p.name}`);
		console.log(`    Installed: ${p.installedVersion}  Source: ${p.sourceVersion}`);
		console.log(`    Reason: ${p.reason}`);
	}
	console.log("\nRun 'orqa plugin refresh [name]' or 'orqa install' to sync.");
}

// ---------------------------------------------------------------------------
// enable
// ---------------------------------------------------------------------------

async function cmdEnable(args: string[]): Promise<void> {
	if (args.length === 0) {
		console.error("Usage: orqa plugin enable <name>");
		process.exit(1);
	}

	const name = args[0];
	const projectRoot = process.cwd();

	const pluginDir = resolvePluginDir(name, projectRoot);
	if (!pluginDir) {
		console.error(`Plugin not found: ${name}`);
		console.error("Run 'orqa plugin install' first.");
		process.exit(1);
	}

	const pluginManifest = readManifest(pluginDir);

	// Copy content from plugin -> .orqa/
	const copyResult = copyPluginContent(pluginDir, projectRoot, pluginManifest);
	const copiedCount = Object.keys(copyResult.copied).length;

	if (copiedCount > 0) {
		console.log(`Copied ${copiedCount} content file(s) to .orqa/`);
	}

	// Update content manifest (add or refresh entry)
	const contentManifest = readContentManifest(projectRoot);
	contentManifest.plugins[name] = {
		version: pluginManifest.version,
		installed_at: new Date().toISOString(),
		files: copyResult.copied,
	};
	writeContentManifest(projectRoot, contentManifest);

	// Set enabled: true in project.json
	const projectJsonPath = path.join(projectRoot, ".orqa", "project.json");
	if (fs.existsSync(projectJsonPath)) {
		updateProjectJsonPlugin(projectRoot, name, { enabled: true });
	}

	console.log(`Plugin ${name} enabled.`);
}

// ---------------------------------------------------------------------------
// disable
// ---------------------------------------------------------------------------

async function cmdDisable(args: string[]): Promise<void> {
	if (args.length === 0) {
		console.error("Usage: orqa plugin disable <name>");
		process.exit(1);
	}

	const name = args[0];
	const projectRoot = process.cwd();

	// Delete files from .orqa/ but keep the manifest entry (for re-enable)
	deleteContentFiles(projectRoot, name);

	// Set enabled: false in project.json
	const projectJsonPath = path.join(projectRoot, ".orqa", "project.json");
	if (fs.existsSync(projectJsonPath)) {
		updateProjectJsonPlugin(projectRoot, name, { enabled: false });
	}

	console.log(`Plugin ${name} disabled. Content removed from .orqa/ (manifest retained for re-enable).`);
}

// ---------------------------------------------------------------------------
// refresh
// ---------------------------------------------------------------------------

async function cmdRefresh(args: string[]): Promise<void> {
	const targetName = args[0];
	const projectRoot = process.cwd();

	// Collect plugins to refresh
	const installed = listInstalledPlugins(projectRoot);
	const toRefresh = targetName
		? installed.filter((p) => p.name === targetName)
		: installed;

	if (toRefresh.length === 0) {
		console.log(targetName ? `Plugin not found: ${targetName}` : "No plugins installed.");
		return;
	}

	for (const p of toRefresh) {
		// Only refresh enabled plugins (unless a specific name was requested)
		if (!targetName) {
			const projectJsonPath = path.join(projectRoot, ".orqa", "project.json");
			if (fs.existsSync(projectJsonPath)) {
				try {
					const data = readProjectJson(projectRoot);
					const plugins = data["plugins"] as Record<string, { enabled?: boolean }> | undefined;
					if (plugins && plugins[p.name]?.enabled === false) {
						console.log(`Skipping disabled plugin: ${p.name}`);
						continue;
					}
				} catch {
					// project.json unreadable — proceed
				}
			}
		}

		console.log(`Refreshing ${p.name}...`);

		const pluginDir = p.path;
		const pluginManifest = readManifest(pluginDir);

		// Install deps and build
		installPluginDeps(pluginDir, pluginManifest);
		buildPlugin(pluginDir, pluginManifest);

		// Re-sync content (uses three-way diff internally)
		const existingManifest = readContentManifest(projectRoot);
		const copyResult = copyPluginContent(pluginDir, projectRoot, pluginManifest, existingManifest);

		// Merge: skipped files retain their existing hashes
		const mergedFiles: Record<string, FileHashEntry> = { ...copyResult.copied };
		const existingEntry = existingManifest.plugins[p.name];
		if (existingEntry) {
			for (const skippedFile of copyResult.skipped) {
				const existing = existingEntry.files[skippedFile.path];
				if (existing) {
					mergedFiles[skippedFile.path] = existing;
				}
			}
		}

		// Update manifest
		const contentManifest = readContentManifest(projectRoot);
		contentManifest.plugins[p.name] = {
			version: pluginManifest.version,
			installed_at: new Date().toISOString(),
			files: mergedFiles,
		};
		writeContentManifest(projectRoot, contentManifest);

		const copiedCount = Object.keys(copyResult.copied).length;
		if (copiedCount > 0) {
			console.log(`  Re-synced ${copiedCount} content file(s)`);
		} else {
			console.log(`  No content to sync.`);
		}

		if (copyResult.skipped.length > 0) {
			console.log(`  Skipped ${copyResult.skipped.length} user-modified file(s)`);
		}

		// Ensure plugin is registered in project.json (fixes fresh-clone gap)
		const shortPath = path.relative(projectRoot, pluginDir);
		updateProjectJsonPlugin(projectRoot, p.name, {
			installed: true,
			enabled: true,
			path: shortPath,
		});
	}

	// Process aggregated files from all plugins
	try {
		processAggregatedFiles(projectRoot);
	} catch {
		// Non-fatal
	}

	// Resolve workflows from plugin contributions
	try {
		runWorkflowResolution(projectRoot);
	} catch {
		// Non-fatal
	}

	// Compose schema from all installed plugin manifests
	try {
		writeComposedSchema(projectRoot);
	} catch {
		// Non-fatal
	}

	console.log("Refresh complete.");
}

// ---------------------------------------------------------------------------
// diff
// ---------------------------------------------------------------------------

async function cmdDiff(args: string[]): Promise<void> {
	const targetName = args[0];
	const useJson = args.includes("--json");
	const projectRoot = process.cwd();

	const installed = listInstalledPlugins(projectRoot);
	const toDiff = targetName
		? installed.filter((p) => p.name === targetName)
		: installed;

	if (toDiff.length === 0) {
		console.log(targetName ? `Plugin not found: ${targetName}` : "No plugins installed.");
		return;
	}

	const results = [];

	for (const p of toDiff) {
		const pluginManifest = readManifest(p.path);
		const result = diffPluginContent(p.path, projectRoot, pluginManifest);
		results.push(result);
	}

	if (useJson) {
		console.log(JSON.stringify(results, null, 2));
		return;
	}

	// Human-readable output
	let totalModified = 0;
	let totalMissing = 0;
	let totalIdentical = 0;
	let totalOrphaned = 0;

	for (const result of results) {
		console.log(`  ${result.pluginName}:`);

		const allFiles = [
			...result.identical.map((f) => ({ file: f, status: "identical" })),
			...result.modified.map((f) => ({ file: f, status: "MODIFIED" })),
			...result.missing.map((f) => ({ file: f, status: "MISSING" })),
			...result.orphaned.map((f) => ({ file: f, status: "ORPHANED" })),
		];

		if (allFiles.length === 0) {
			console.log("    (no content)");
		} else {
			for (const { file, status } of allFiles) {
				const filename = path.basename(file);
				console.log(`    ${filename}: ${status}`);
			}
		}

		console.log();

		totalIdentical += result.identical.length;
		totalModified += result.modified.length;
		totalMissing += result.missing.length;
		totalOrphaned += result.orphaned.length;
	}

	const parts: string[] = [];
	if (totalModified > 0) parts.push(`${totalModified} modified`);
	if (totalMissing > 0) parts.push(`${totalMissing} missing`);
	if (totalOrphaned > 0) parts.push(`${totalOrphaned} orphaned`);
	if (totalIdentical > 0) parts.push(`${totalIdentical} identical`);

	console.log(`  ${parts.join(", ")}`);
}

// ---------------------------------------------------------------------------
// registry
// ---------------------------------------------------------------------------

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
			`Failed to fetch registry: ${err instanceof Error ? err.message : String(err)}`,
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

// ---------------------------------------------------------------------------
// create
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// status
// ---------------------------------------------------------------------------

async function cmdStatus(args: string[]): Promise<void> {
	const targetName = args[0];
	const useJson = args.includes("--json");
	const projectRoot = process.cwd();

	const installed = listInstalledPlugins(projectRoot);
	const toCheck = targetName
		? installed.filter((p) => p.name === targetName)
		: installed;

	if (toCheck.length === 0) {
		console.log(targetName ? `Plugin not found: ${targetName}` : "No plugins installed.");
		return;
	}

	const contentManifest = readContentManifest(projectRoot);
	const allResults: Array<{ plugin: string; files: ThreeWayFileStatus[] }> = [];

	for (const p of toCheck) {
		const pluginManifest = readManifest(p.path);
		const entry = contentManifest.plugins[p.name];
		if (!entry) continue;

		const fileStatuses: ThreeWayFileStatus[] = [];

		for (const [relPath, hashEntry] of Object.entries(entry.files)) {
			const sourceFile = findSourceFile(p.path, pluginManifest, relPath);
			const sourceHash = sourceFile && fs.existsSync(sourceFile)
				? computeFileHash(sourceFile)
				: "";

			const state = computeThreeWayState(relPath, projectRoot, hashEntry, sourceHash);
			fileStatuses.push({ path: relPath, state });
		}

		allResults.push({ plugin: p.name, files: fileStatuses });
	}

	if (useJson) {
		console.log(JSON.stringify(allResults, null, 2));
		return;
	}

	for (const result of allResults) {
		console.log(`\n${result.plugin}:`);
		for (const f of result.files) {
			const icon = f.state === "clean" ? " " :
				f.state === "plugin-updated" ? "P" :
				f.state === "user-modified" ? "U" :
				f.state === "conflict" ? "C" :
				f.state === "missing" ? "!" : "?";
			console.log(`  [${icon}] ${path.basename(f.path)}: ${f.state}`);
		}
	}
}

// ---------------------------------------------------------------------------
// template-validate
// ---------------------------------------------------------------------------

async function cmdTemplateValidate(args: string[]): Promise<void> {
	const templateDir = args[0];
	if (!templateDir) {
		console.error("Usage: orqa plugin template-validate <template-dir>");
		process.exit(1);
	}

	const absDir = path.resolve(templateDir);
	if (!fs.existsSync(absDir)) {
		console.error(`Directory not found: ${absDir}`);
		process.exit(1);
	}

	const manifestPath = path.join(absDir, "orqa-plugin.json");
	if (!fs.existsSync(manifestPath)) {
		console.error(`No orqa-plugin.json found in ${absDir}`);
		process.exit(1);
	}

	const errors: string[] = [];

	try {
		const raw = fs.readFileSync(manifestPath, "utf-8");
		const manifest = JSON.parse(raw) as PluginManifest;

		if (!manifest.name) errors.push("Missing required field: name");
		if (!manifest.version) errors.push("Missing required field: version");
		if (!manifest.provides) errors.push("Missing required field: provides");

		// Check content mappings point to existing directories
		if (manifest.content) {
			for (const [label, mapping] of Object.entries(manifest.content)) {
				const sourceDir = path.join(absDir, mapping.source);
				if (!fs.existsSync(sourceDir)) {
					errors.push(`Content "${label}": source dir "${mapping.source}" does not exist`);
				}
			}
		}
	} catch (err) {
		errors.push(`Failed to parse orqa-plugin.json: ${err instanceof Error ? err.message : String(err)}`);
	}

	if (errors.length === 0) {
		console.log("Template validation passed.");
	} else {
		console.error(`Template validation failed (${errors.length} error(s)):`);
		for (const e of errors) {
			console.error(`  - ${e}`);
		}
		process.exit(1);
	}
}
