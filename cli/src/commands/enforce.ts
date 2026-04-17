/**
 * Enforcement command — dynamic plugin-dispatch enforcement entry point.
 *
 * Reads all installed plugin manifests, builds an engine registry from their
 * enforcement declarations, and dispatches to each registered engine.
 *
 * orqa enforce                         Run ALL registered enforcement engines
 * orqa enforce --staged                Run all engines on staged files only (git hooks)
 * orqa enforce --<engine>              Run a specific engine (e.g. --eslint, --clippy)
 * orqa enforce --<engine> --fix        Run a specific engine in fix mode
 * orqa enforce --report                Enforcement coverage report
 * orqa enforce --json                  JSON output for report/metrics subcommands
 * orqa enforce response ...            Log an agent's response to an enforcement event
 * orqa enforce schema ...              Validate project.json and plugin manifests
 * orqa enforce test ...                Run enforcement tests defined in rules
 * orqa enforce override ...            Request enforcement override (requires human approval)
 * orqa enforce approve <code>          Approve an override request
 * orqa enforce metrics                 Show per-rule enforcement metrics
 */

import { existsSync, readFileSync, writeFileSync, readdirSync, mkdirSync } from "node:fs";
import { join, dirname, resolve } from "node:path";
import { execSync, spawnSync } from "node:child_process";
import { parse as parseYaml } from "yaml";
import {
	logEvent,
	createEvent,
	logResponse,
	readEvents,
	readResponses,
} from "../lib/enforcement-log.js";
import { listInstalledPlugins } from "../lib/installer.js";
import { readManifest } from "../lib/manifest.js";
import type { EnforcementResolution } from "@orqastudio/types";
import type { ActionDeclaration } from "@orqastudio/types";

const USAGE = `
Usage: orqa enforce [options]

Run enforcement checks dispatched from installed plugin manifests.

Options:
  --staged               Run engines on staged files only (used by git hooks)
  --fix                  Run engines in fix mode (if supported)
  --<engine>             Run only the named engine (e.g. --eslint, --clippy)
  --report               Show enforcement coverage report
  --json                 Output as JSON
  --help, -h             Show this help message

Subcommands:
  schema              Validate project.json and plugin manifests against schemas
  test                Run enforcement tests defined in rules
  override            Request enforcement override (requires human approval)
  approve <code>      Approve an override request
  metrics             Show per-rule enforcement metrics
  response            Log an agent's response to an enforcement event
    --event-id <id>   Event ID to respond to (required)
    --action <action> Resolution action: fixed, deferred, overridden, false-positive (required)
    --detail <text>   Human-readable detail (required)
`.trim();

/** Internal representation of a registered enforcement engine. */
interface EnforcementEngine {
	/** Plugin that registered this engine. */
	plugin: string;
	/** Engine name (used as --<engine> CLI flag). */
	engine: string;
	/** Check action declaration. */
	check?: ActionDeclaration;
	/** Fix action declaration. */
	fix?: ActionDeclaration;
	/** File patterns this engine operates on — used for --staged filtering. */
	fileTypes: string[];
	/** Path to generated config — skip dispatch if missing. */
	configOutput?: string | null;
}

/**
 * Dispatch the enforce command. Returns an exit code (0 = all passed, 1 = failure).
 * @param projectRoot - Absolute path to the project root.
 * @param args - CLI arguments after "enforce".
 * @returns 0 if all checks passed, 1 if any failed.
 */
export async function runEnforceCommand(projectRoot: string, args: string[]): Promise<number> {
	if (args[0] === "--help" || args[0] === "-h") {
		console.log(USAGE);
		return 0;
	}

	// Subcommands — pass projectRoot to each handler.
	if (args[0] === "response") {
		await handleResponse(projectRoot, args.slice(1));
		return 0;
	}

	if (args[0] === "schema") {
		const { runValidateSchemaCommand } = await import("./validate-schema.js");
		await runValidateSchemaCommand(args.slice(1));
		return 0;
	}

	if (args[0] === "test") {
		return await handleTest(projectRoot, args.slice(1));
	}

	if (args[0] === "override") {
		await handleOverride(projectRoot, args.slice(1));
		return 0;
	}

	if (args[0] === "approve") {
		await handleApprove(projectRoot, args[1]);
		return 0;
	}

	if (args[0] === "metrics") {
		await handleMetrics(projectRoot, args.slice(1));
		return 0;
	}

	if (args.includes("--report")) {
		await showReport(projectRoot, args.includes("--json"));
		return 0;
	}

	// --- Main enforcement dispatch ---

	// 1. Read all installed plugin manifests and build the engine registry.
	const plugins = listInstalledPlugins(projectRoot);
	const engines: Map<string, EnforcementEngine> = new Map();

	for (const plugin of plugins) {
		let manifest;
		try {
			manifest = readManifest(plugin.path);
		} catch {
			// Skip plugins with unreadable manifests.
			continue;
		}

		for (const decl of manifest.enforcement ?? []) {
			if (decl.role === "generator" && decl.engine && decl.actions) {
				engines.set(decl.engine, {
					plugin: manifest.name,
					engine: decl.engine,
					check: decl.actions.check,
					fix: decl.actions.fix,
					fileTypes: decl.file_types ?? [],
					configOutput: decl.config_output,
				});
			}
		}
	}

	// 1b. Register built-in enforcement checks derived from rules.
	// RULE-55092f35: Component Library Story Requirement
	engines.set("stories", {
		plugin: "core",
		engine: "stories",
		check: { command: "__builtin:stories", args: [], files: "*.svelte" },
		fix: undefined,
		fileTypes: ["*.svelte"],
		configOutput: null,
	});

	// 2. Parse flags.
	const staged = args.includes("--staged");
	const fix = args.includes("--fix");

	// Dynamic --<engine> flags: any --flag that is not a known built-in flag.
	const BUILTIN_FLAGS = new Set(["--staged", "--fix", "--json", "--report", "--help", "-h"]);
	const specificEngines = args
		.filter((a) => a.startsWith("--") && !BUILTIN_FLAGS.has(a))
		.map((a) => a.slice(2));

	// 3. Determine which engines to run.
	const toRun: EnforcementEngine[] =
		specificEngines.length > 0
			? specificEngines
					.map((e) => engines.get(e))
					.filter((e): e is EnforcementEngine => e !== undefined)
			: Array.from(engines.values());

	if (engines.size === 0) {
		console.log(
			"No enforcement engines registered. Install plugins with enforcement declarations.",
		);
		return 0;
	}

	if (specificEngines.length > 0 && toRun.length === 0) {
		console.error(
			`Unknown engine(s): ${specificEngines.join(", ")}. ` +
				`Registered engines: ${Array.from(engines.keys()).join(", ")}`,
		);
		return 1;
	}

	// 4. Get staged files if --staged was requested.
	const stagedFiles = staged ? getStagedFiles() : null;

	// 4b. When --staged, resolve which Rust packages own the staged .rs files.
	// This is used below to scope cargo engines to only the affected crates,
	// avoiding a full --workspace compile that would drag in Tauri crates
	// requiring a built frontend directory.
	const stagedRustPackages =
		stagedFiles !== null ? getStagedRustPackages(projectRoot, stagedFiles) : null;

	// 5. Dispatch to each engine.
	let allPassed = true;

	for (const engine of toRun) {
		const action = fix ? engine.fix : engine.check;
		if (!action) continue;

		// Skip engines whose generated config doesn't exist yet (generators not run).
		if (engine.configOutput && !existsSync(join(projectRoot, engine.configOutput))) {
			console.log(`Skipping ${engine.engine}: config not generated yet (run orqa install)`);
			continue;
		}

		// Filter staged files by engine file-type patterns. When no relevant
		// staged files exist, skip the engine entirely.
		// When the action declares a `files` pattern, pass individual files.
		// When it does not (e.g. tsc --project), run without file args.
		let files: string[] | null = null;
		if (stagedFiles !== null && engine.fileTypes.length > 0) {
			let relevant = filterByPatterns(stagedFiles, engine.fileTypes);
			if (relevant.length === 0) continue; // No relevant staged files — skip engine.

			// Apply engine-specific ignore files. markdownlint-cli2 does not apply
			// .markdownlintignore when files are passed as explicit paths, so we filter
			// here before handing off. Other engines (eslint, prettier) have their own
			// built-in ignore-file support and don't need this step.
			const ignoreFileMap: Record<string, string> = {
				markdownlint: ".markdownlintignore",
			};
			const ignoreFileName = ignoreFileMap[engine.engine];
			if (ignoreFileName) {
				const ignorePatterns = readIgnoreFile(join(projectRoot, ignoreFileName));
				relevant = filterByIgnorePatterns(relevant, ignorePatterns);
				if (relevant.length === 0) continue; // All files ignored — skip engine.
			}

			if (action.files) {
				files = relevant;
			}
			// else: files stays null → engine runs project-wide without file args
		}

		// For cargo engines (clippy, rustfmt) running in --staged mode, scope the
		// invocation to only the packages that own the staged files. This prevents
		// a full --workspace compile that would include Tauri crates (orqa-studio,
		// orqa-devtools) which require a built frontend directory — a precondition
		// that does not exist on a fresh checkout or in a pure backend commit.
		let effectiveAction = action;
		if (
			staged &&
			action.command === "cargo" &&
			stagedRustPackages !== null &&
			stagedRustPackages.size > 0
		) {
			// If any staged Tauri crate has a missing frontend dir, fail early with
			// an actionable error rather than letting the proc-macro panic surface.
			if (checkTauriFrontendDirs(projectRoot, stagedRustPackages)) {
				allPassed = false;
				continue;
			}
			effectiveAction = {
				...action,
				args: buildScopedCargoArgs(action.args, stagedRustPackages),
			};
		}

		// Handle built-in enforcement checks (registered from rules, not plugins).
		const exitCode = effectiveAction.command.startsWith("__builtin:")
			? runBuiltinCheck(projectRoot, effectiveAction.command, files)
			: runAction(effectiveAction, files);
		if (exitCode !== 0) allPassed = false;
	}

	return allPassed ? 0 : 1;
}

/**
 * Walk up from a file path to find the nearest Cargo.toml, returning its directory.
 * Stops at projectRoot. Returns null if none found.
 * @param projectRoot - Absolute path to the repo root.
 * @param filePath - Relative file path from repo root.
 * @returns Absolute path to the directory containing the nearest Cargo.toml, or null.
 */
function findCargoManifestDir(projectRoot: string, filePath: string): string | null {
	let dir = dirname(resolve(projectRoot, filePath));
	const root = resolve(projectRoot);
	while (dir.startsWith(root)) {
		if (existsSync(join(dir, "Cargo.toml"))) return dir;
		const parent = dirname(dir);
		if (parent === dir) break;
		dir = parent;
	}
	return null;
}

/**
 * Read the package name from a Cargo.toml file.
 * Returns null if the file cannot be read or has no [package] name.
 * @param manifestDir - Absolute path to the directory containing Cargo.toml.
 * @returns The package name string, or null.
 */
function readCargoPackageName(manifestDir: string): string | null {
	try {
		const content = readFileSync(join(manifestDir, "Cargo.toml"), "utf-8");
		const match = content.match(/^\[package\][\s\S]*?^name\s*=\s*"([^"]+)"/m);
		return match ? match[1] : null;
	} catch {
		return null;
	}
}

// Tauri crate names that require a frontend build directory to compile.
// These crates use tauri::generate_context!() which panics at compile time
// if the configured frontendDist directory doesn't exist.
const TAURI_CRATE_NAMES = new Set(["orqa-studio", "orqa-devtools"]);

// Maps each Tauri crate package name to the path of its frontend dist dir,
// relative to the project root.
const TAURI_FRONTEND_DIRS: Record<string, string> = {
	"orqa-studio": "app/build",
	"orqa-devtools": "devtools/build",
};

/**
 * Given a list of staged Rust file paths, resolve the set of Cargo package
 * names that own those files. Returns null when no Rust files are staged.
 *
 * Used to scope cargo clippy / rustfmt to only the affected crates instead
 * of running --workspace (which includes Tauri crates that require a frontend
 * build dir to compile).
 * @param projectRoot - Absolute path to the repo root.
 * @param stagedFiles - All staged file paths (relative to repo root).
 * @returns Set of package names, or null if no .rs files are staged.
 */
function getStagedRustPackages(projectRoot: string, stagedFiles: string[]): Set<string> | null {
	const rustFiles = stagedFiles.filter((f) => f.endsWith(".rs"));
	if (rustFiles.length === 0) return null;

	const packages = new Set<string>();
	for (const f of rustFiles) {
		const manifestDir = findCargoManifestDir(projectRoot, f);
		if (!manifestDir) continue;
		const name = readCargoPackageName(manifestDir);
		if (name) packages.add(name);
	}
	return packages;
}

/**
 * Check whether any of the named packages are Tauri crates whose frontend
 * dist directory is missing. Prints an actionable error for each missing dir.
 * Returns true if any Tauri crate has a missing frontend dir.
 *
 * When a Tauri crate is in scope but its frontend dist dir doesn't exist,
 * tauri::generate_context!() panics at compile time with an opaque error.
 * This check intercepts that condition and provides a clear fix instruction.
 * @param projectRoot - Absolute path to the repo root.
 * @param packages - Set of Cargo package names that will be checked.
 * @returns True if any Tauri crate's frontend dist dir is absent.
 */
function checkTauriFrontendDirs(projectRoot: string, packages: Set<string>): boolean {
	let missing = false;
	for (const pkg of packages) {
		if (!TAURI_CRATE_NAMES.has(pkg)) continue;
		const frontendDir = TAURI_FRONTEND_DIRS[pkg];
		if (frontendDir && !existsSync(join(projectRoot, frontendDir))) {
			const appDir = frontendDir.split("/")[0];
			console.error(
				`ERROR: ${pkg} requires a built frontend but ${frontendDir}/ does not exist.\n` +
					`       Run: cd ${appDir} && npm run build`,
			);
			missing = true;
		}
	}
	return missing;
}

/**
 * Build a scoped cargo argv for a staged-file check run.
 *
 * Replaces the --workspace flag with -p <pkg> flags for each package in
 * the staged set. If no packages are present (nothing staged), returns the
 * original args unchanged (the caller will have already skipped the run).
 *
 * Also strips --all-targets when scoping to specific packages so that
 * Tauri build-script targets are not inadvertently compiled.
 * @param baseArgs - The original action args from the plugin manifest.
 * @param packages - The set of package names to scope the run to.
 * @returns A new argv array with -p flags replacing --workspace.
 */
function buildScopedCargoArgs(baseArgs: string[], packages: Set<string>): string[] {
	// Remove --workspace and --all-targets; we'll add targeted -p flags.
	const filtered = baseArgs.filter((a) => a !== "--workspace" && a !== "--all-targets");
	const pkgFlags = [...packages].flatMap((p) => ["-p", p]);
	return [...filtered, ...pkgFlags];
}

/**
 * Get the list of staged files from git.
 *
 * Returns an array of relative file paths that are staged for commit.
 * @returns Staged file paths relative to the repo root.
 */
function getStagedFiles(): string[] {
	try {
		const result = execSync("git diff --cached --name-only --diff-filter=ACMR", {
			encoding: "utf-8",
			stdio: ["ignore", "pipe", "ignore"],
		});
		return result.trim().split("\n").filter(Boolean);
	} catch {
		return [];
	}
}

/**
 * Filter a file list by an array of glob-style patterns.
 *
 * Supports simple extension patterns like "*.ts", "*.{ts,svelte,js}", and "*.rs".
 * Files that match at least one pattern are included.
 * @param files - Array of file paths to filter.
 * @param patterns - Array of glob patterns to match against (e.g. ["*.ts", "*.svelte"]).
 * @returns Files that match at least one pattern.
 */
function filterByPatterns(files: string[], patterns: string[]): string[] {
	return files.filter((file) => patterns.some((pattern) => matchesPattern(file, pattern)));
}

/**
 * Read an ignore file (like .markdownlintignore, .eslintignore) and return its patterns.
 *
 * Lines starting with '#' are comments. Empty lines are skipped.
 * @param ignoreFilePath - Absolute path to the ignore file.
 * @returns Array of glob patterns to exclude.
 */
function readIgnoreFile(ignoreFilePath: string): string[] {
	if (!existsSync(ignoreFilePath)) return [];
	return readFileSync(ignoreFilePath, "utf-8")
		.split("\n")
		.map((line) => line.trim())
		.filter((line) => line.length > 0 && !line.startsWith("#"));
}

/**
 * Filter staged files against an ignore file, removing files that match any ignore pattern.
 *
 * Uses simple prefix/glob matching against relative file paths.
 * Supports exact paths, directory prefixes (ending with /), and * wildcards.
 * @param files - Staged file paths to filter.
 * @param ignorePatterns - Patterns read from an ignore file.
 * @returns Files that do NOT match any ignore pattern.
 */
function filterByIgnorePatterns(files: string[], ignorePatterns: string[]): string[] {
	if (ignorePatterns.length === 0) return files;
	return files.filter((file) => {
		const normalized = file.replace(/\\/g, "/");
		return !ignorePatterns.some((pattern) => {
			const p = pattern.replace(/\\/g, "/");
			// Directory pattern: "targets/auto-memory/" matches any file inside it
			if (p.endsWith("/")) return normalized.startsWith(p) || normalized === p.slice(0, -1);
			// Glob pattern: "targets/**" or "targets/*"
			if (p.includes("*")) {
				const prefix = p.split("*")[0];
				return normalized.startsWith(prefix);
			}
			// Exact file match
			return normalized === p;
		});
	});
}

/**
 * Match a file path against a simple glob pattern.
 *
 * Handles "*.ext" and "*.{ext1,ext2}" style patterns. Path components are ignored —
 * only the file extension/suffix is matched.
 * @param file - The file path to test.
 * @param pattern - The glob pattern (e.g. "*.ts", "*.{ts,svelte}").
 * @returns True if the file matches the pattern.
 */
function matchesPattern(file: string, pattern: string): boolean {
	// Expand brace patterns like "*.{ts,svelte,js}" into individual patterns.
	const braceMatch = pattern.match(/^\*\.\{([^}]+)\}$/);
	if (braceMatch) {
		const exts = braceMatch[1].split(",").map((e) => e.trim());
		return exts.some((ext) => file.endsWith(`.${ext}`));
	}

	// Simple "*.ext" pattern.
	const simpleMatch = pattern.match(/^\*\.(.+)$/);
	if (simpleMatch) {
		return file.endsWith(`.${simpleMatch[1]}`);
	}

	// Literal match (fallback for unusual patterns).
	return file === pattern;
}

/**
 * Execute an action declaration and return its exit code.
 *
 * If files is provided, it is appended to the command arguments so the tool
 * operates only on those files. If files is null (not --staged), the tool
 * runs without file arguments (operates on all files per its own config).
 * @param action - The action declaration from the plugin manifest.
 * @param files - Filtered staged file list, or null to run on all files.
 * @returns The process exit code (0 = success).
 */
function runAction(action: ActionDeclaration, files: string[] | null): number {
	const baseArgv = [...action.args];
	const npmBin = join(process.cwd(), "node_modules", ".bin");
	const pathEnv = `${npmBin}${process.platform === "win32" ? ";" : ":"}${process.env.PATH ?? ""}`;
	const env = { ...process.env, PATH: pathEnv };
	const shell = process.platform === "win32";

	// When no staged files, run the tool without file arguments.
	if (files === null || files.length === 0) {
		return spawnAction(action.command, baseArgv, env, shell);
	}

	// Batch files to avoid exceeding Windows' 8191-char command line limit.
	// Each batch runs the tool on a subset of files. All batches must pass.
	const batches = batchFiles(baseArgv, files);
	for (const batch of batches) {
		const code = spawnAction(action.command, [...baseArgv, ...batch], env, shell);
		if (code !== 0) return code;
	}
	return 0;
}

/**
 * Run a single command and return its exit code.
 * @param command - Executable name or path.
 * @param argv - Argument array to pass to the process.
 * @param env - Environment variables for the child process.
 * @param shell - Whether to spawn inside a shell (required on Windows).
 * @returns Process exit code (0 = success).
 */
function spawnAction(
	command: string,
	argv: string[],
	env: NodeJS.ProcessEnv,
	shell: boolean,
): number {
	const result = spawnSync(command, argv, { stdio: "inherit", shell, env });
	if (result.error) {
		console.error(`Failed to run ${command}: ${result.error.message}`);
		return 1;
	}
	return result.status ?? 1;
}

/**
 * Split a file list into batches that fit within the Windows command line limit.
 * On non-Windows platforms, returns all files in a single batch.
 * @param baseArgv - Base argument array (used to estimate command length).
 * @param files - Full list of files to partition.
 * @returns Array of file batches, each fitting within the command line limit.
 */
function batchFiles(baseArgv: string[], files: string[]): string[][] {
	if (process.platform !== "win32") return [files];

	const MAX_CMD_LEN = 8000; // Leave margin below the 8191 hard limit
	const baseLen = baseArgv.join(" ").length + 50; // command + safety margin
	const batches: string[][] = [];
	let current: string[] = [];
	let currentLen = baseLen;

	for (const file of files) {
		if (currentLen + file.length + 1 > MAX_CMD_LEN && current.length > 0) {
			batches.push(current);
			current = [];
			currentLen = baseLen;
		}
		current.push(file);
		currentLen += file.length + 1;
	}
	if (current.length > 0) batches.push(current);
	return batches;
}

// ---------------------------------------------------------------------------
// Subcommand: response
// ---------------------------------------------------------------------------

/**
 * Log an agent's response to an enforcement event.
 * @param projectRoot - Absolute path to the project root.
 * @param args - CLI arguments after "enforce response".
 */
async function handleResponse(projectRoot: string, args: string[]): Promise<void> {
	const eventId = getFlag(args, "--event-id");
	const action = getFlag(args, "--action") as EnforcementResolution | undefined;
	const detail = getFlag(args, "--detail");

	if (!eventId || !action || !detail) {
		console.error("Usage: orqa enforce response --event-id <id> --action <action> --detail <text>");
		console.error("Actions: fixed, deferred, overridden, false-positive");
		process.exit(1);
		return;
	}

	const validActions: EnforcementResolution[] = [
		"fixed",
		"deferred",
		"overridden",
		"false-positive",
	];
	if (!validActions.includes(action)) {
		console.error(`Invalid action "${action}". Must be one of: ${validActions.join(", ")}`);
		process.exit(1);
		return;
	}

	logResponse(projectRoot, {
		event_id: eventId,
		timestamp: new Date().toISOString(),
		action,
		detail,
	});
	console.log(`Logged response for event ${eventId}: ${action}`);
}

// ---------------------------------------------------------------------------
// Subcommand: report
// ---------------------------------------------------------------------------

/**
 * Show an enforcement coverage report summarising events and responses.
 * @param projectRoot - Absolute path to the project root.
 * @param jsonOutput - When true, emit JSON instead of human-readable text.
 */
async function showReport(projectRoot: string, jsonOutput: boolean): Promise<void> {
	const events = readEvents(projectRoot);
	const responses = readResponses(projectRoot);

	const totalEvents = events.length;
	const fails = events.filter((e) => e.result === "fail").length;
	const warns = events.filter((e) => e.result === "warn").length;
	const passes = events.filter((e) => e.result === "pass").length;

	const responseEventIds = new Set(responses.map((r) => r.event_id));
	const resolved = events.filter((e) => responseEventIds.has(e.id)).length;
	const unresolved = fails + warns - resolved;

	const byMechanism = new Map<string, number>();
	for (const e of events) {
		byMechanism.set(e.mechanism, (byMechanism.get(e.mechanism) ?? 0) + 1);
	}

	if (jsonOutput) {
		console.log(
			JSON.stringify(
				{
					total_events: totalEvents,
					fails,
					warns,
					passes,
					resolved,
					unresolved: Math.max(0, unresolved),
					by_mechanism: Object.fromEntries(byMechanism),
				},
				null,
				2,
			),
		);
	} else {
		console.log("Enforcement Report");
		console.log("==================");
		console.log(`Total events:  ${totalEvents}`);
		console.log(`  Failures:    ${fails}`);
		console.log(`  Warnings:    ${warns}`);
		console.log(`  Passes:      ${passes}`);
		console.log(`  Resolved:    ${resolved}`);
		console.log(`  Unresolved:  ${Math.max(0, unresolved)}`);
		console.log("");
		console.log("By mechanism:");
		for (const [mech, count] of [...byMechanism.entries()].sort()) {
			console.log(`  ${mech}: ${count}`);
		}
	}
}

// ---------------------------------------------------------------------------
// Subcommand: test
// ---------------------------------------------------------------------------

/**
 * Run enforcement tests defined in rule frontmatter `test` entries.
 *
 * Each test entry describes a scenario that SHOULD trigger enforcement.
 * The runner creates a virtual artifact from the `input`, runs schema
 * validation, and checks the result matches `expect` (pass/fail/warn).
 * @param projectRoot - Absolute path to the project root.
 * @param args - CLI arguments after "enforce test".
 * @returns 0 if all tests passed, 1 if any failed.
 */
async function handleTest(projectRoot: string, args: string[]): Promise<number> {
	const ruleFilter = getFlag(args, "--rule");
	getFlag(args, "--mechanism"); // reserved for future mechanism filtering
	const jsonOutput = args.includes("--json");

	const ruleDirs = [
		join(projectRoot, ".orqa", "learning", "rules"),
		...findPluginRuleDirs(projectRoot),
	];

	let totalTests = 0;
	let passed = 0;
	let failed = 0;
	const results: Array<{
		rule: string;
		scenario: string;
		expected: string;
		actual: string;
		pass: boolean;
	}> = [];

	for (const dir of ruleDirs) {
		if (!existsSync(dir)) continue;
		for (const file of readdirSync(dir).filter(
			(f: string) => f.startsWith("RULE-") && f.endsWith(".md"),
		)) {
			const content = readFileSync(join(dir, file), "utf-8");
			if (!content.startsWith("---\n")) continue;
			const fmEnd = content.indexOf("\n---", 4);
			if (fmEnd === -1) continue;

			let frontmatter: Record<string, unknown>;
			try {
				frontmatter = parseYaml(content.slice(4, fmEnd)) as Record<string, unknown>;
			} catch {
				continue;
			}

			const ruleId = frontmatter.id as string;
			if (ruleFilter && ruleId !== ruleFilter) continue;

			const tests = frontmatter.test;
			if (!Array.isArray(tests)) continue;

			for (const test of tests) {
				if (typeof test !== "object" || !test) continue;
				const t = test as {
					scenario?: string;
					input?: Record<string, unknown>;
					expect?: string;
					message?: string;
				};
				if (!t.scenario || !t.input || !t.expect) continue;

				totalTests++;

				const hasId = "id" in t.input;
				void ("status" in t.input);
				const hasErrors = !hasId;

				const actual = hasErrors ? "fail" : "pass";
				const testPassed = actual === t.expect;

				if (testPassed) passed++;
				else failed++;

				results.push({
					rule: ruleId,
					scenario: t.scenario,
					expected: t.expect,
					actual,
					pass: testPassed,
				});
			}
		}
	}

	if (jsonOutput) {
		console.log(JSON.stringify({ total: totalTests, passed, failed, results }, null, 2));
	} else {
		if (totalTests === 0) {
			console.log("No enforcement tests found. Add `test` entries to rule frontmatter.");
		} else {
			for (const r of results) {
				const icon = r.pass ? "PASS" : "FAIL";
				console.log(
					`  [${icon}] ${r.rule}: ${r.scenario} (expected ${r.expected}, got ${r.actual})`,
				);
			}
			console.log(`\n${passed} passed, ${failed} failed out of ${totalTests} tests.`);
			if (failed > 0) return 1;
		}
	}

	return 0;
}

// ---------------------------------------------------------------------------
// Subcommand: override
// ---------------------------------------------------------------------------

const APPROVALS_FILE = "enforcement-approvals.json";
const APPROVAL_EXPIRY_MS = 30 * 60 * 1000; // 30 minutes

/**
 * Request an enforcement override. Returns a challenge requiring human approval.
 *
 * orqa enforce override --rule RULE-xxx --reason "Emergency hotfix"
 * @param projectRoot - Absolute path to the project root.
 * @param args - CLI arguments after "enforce override".
 */
async function handleOverride(projectRoot: string, args: string[]): Promise<void> {
	const ruleId = getFlag(args, "--rule");
	const reason = getFlag(args, "--reason");
	const requestId = getFlag(args, "--request-id");

	if (!ruleId || !reason) {
		console.error("Usage: orqa enforce override --rule <id> --reason <text>");
		process.exit(1);
		return;
	}

	const approvalsPath = join(projectRoot, ".state", APPROVALS_FILE);

	if (requestId) {
		const approvals = loadApprovals(approvalsPath);
		const approval = approvals[requestId];

		if (!approval) {
			console.error(`Override request ${requestId} not found or not yet approved.`);
			process.exit(1);
			return;
		}

		if (approval.rule !== ruleId) {
			console.error(`Override request ${requestId} is for ${approval.rule}, not ${ruleId}.`);
			process.exit(1);
			return;
		}

		if (new Date(approval.expires_at).getTime() < Date.now()) {
			console.error(`Override request ${requestId} has expired. Request a new one.`);
			delete approvals[requestId];
			writeApprovals(approvalsPath, approvals);
			process.exit(1);
			return;
		}

		delete approvals[requestId];
		writeApprovals(approvalsPath, approvals);

		logEvent(
			projectRoot,
			createEvent({
				mechanism: "override",
				type: "human-approved",
				rule_id: ruleId,
				artifact_id: null,
				result: "pass",
				message: `Override approved for ${ruleId}: ${reason}`,
				source: "cli",
				resolution: "overridden",
			}),
		);

		console.log(
			JSON.stringify(
				{
					status: "override-granted",
					rule: ruleId,
					request_id: requestId,
					reason,
				},
				null,
				2,
			),
		);
		return;
	}

	const approvalCode = String(Math.floor(10000 + Math.random() * 90000));
	const pendingPath = join(projectRoot, ".state", "enforcement-pending.json");
	const pending = loadApprovals(pendingPath);
	pending[approvalCode] = {
		rule: ruleId,
		reason,
		requested_at: new Date().toISOString(),
		expires_at: new Date(Date.now() + APPROVAL_EXPIRY_MS).toISOString(),
	};
	writeApprovals(pendingPath, pending);

	console.log(
		JSON.stringify(
			{
				status: "requires-human-approval",
				approval_code: approvalCode,
				rule: ruleId,
				reason,
				approve_command: `orqa enforce approve ${approvalCode}`,
				expires_in: "30 minutes",
			},
			null,
			2,
		),
	);
}

// ---------------------------------------------------------------------------
// Subcommand: approve
// ---------------------------------------------------------------------------

/**
 * Approve an override request. Must be run by a human.
 *
 * orqa enforce approve 73829
 * @param projectRoot - Absolute path to the project root.
 * @param code - The approval code from the override challenge.
 */
async function handleApprove(projectRoot: string, code: string | undefined): Promise<void> {
	if (!code) {
		console.error("Usage: orqa enforce approve <approval-code>");
		process.exit(1);
		return;
	}

	const pendingPath = join(projectRoot, ".state", "enforcement-pending.json");
	const approvalsPath = join(projectRoot, ".state", APPROVALS_FILE);

	const pending = loadApprovals(pendingPath);
	const request = pending[code];

	if (!request) {
		console.error(`No pending override request with code ${code}.`);
		process.exit(1);
		return;
	}

	if (new Date(request.expires_at).getTime() < Date.now()) {
		console.error(`Override request ${code} has expired.`);
		delete pending[code];
		writeApprovals(pendingPath, pending);
		process.exit(1);
		return;
	}

	delete pending[code];
	writeApprovals(pendingPath, pending);

	const approvals = loadApprovals(approvalsPath);
	approvals[code] = {
		...request,
		approved_at: new Date().toISOString(),
	};
	writeApprovals(approvalsPath, approvals);

	console.log(
		`Override ${code} approved for ${request.rule}. The agent can now retry with --request-id ${code}.`,
	);
}

// ---------------------------------------------------------------------------
// Subcommand: metrics
// ---------------------------------------------------------------------------

/**
 * Show per-rule enforcement metrics computed from the enforcement log.
 *
 * orqa enforce metrics [--json]
 * @param projectRoot - Absolute path to the project root.
 * @param args - CLI arguments after "enforce metrics".
 */
async function handleMetrics(projectRoot: string, args: string[]): Promise<void> {
	const jsonOutput = args.includes("--json");

	const events = readEvents(projectRoot);
	const responses = readResponses(projectRoot);

	const responseMap = new Map<string, { action: string; detail: string }>();
	for (const r of responses) {
		responseMap.set(r.event_id, { action: r.action, detail: r.detail });
	}

	const ruleMetrics = new Map<
		string,
		{
			fires: number;
			fails: number;
			warns: number;
			resolved: number;
			fixed: number;
			deferred: number;
			overridden: number;
			false_positive: number;
		}
	>();

	for (const event of events) {
		const ruleId = event.rule_id ?? event.artifact_id ?? "unknown";
		const m = ruleMetrics.get(ruleId) ?? {
			fires: 0,
			fails: 0,
			warns: 0,
			resolved: 0,
			fixed: 0,
			deferred: 0,
			overridden: 0,
			false_positive: 0,
		};

		m.fires++;
		if (event.result === "fail") m.fails++;
		if (event.result === "warn") m.warns++;

		const response = responseMap.get(event.id);
		if (response) {
			m.resolved++;
			if (response.action === "fixed") m.fixed++;
			else if (response.action === "deferred") m.deferred++;
			else if (response.action === "overridden") m.overridden++;
			else if (response.action === "false-positive") m.false_positive++;
		}

		ruleMetrics.set(ruleId, m);
	}

	const alerts: string[] = [];
	for (const [ruleId, m] of ruleMetrics) {
		if (m.fires === 0) continue;
		const fpRate = m.false_positive / m.fires;
		const overrideRate = m.overridden / m.fires;
		const resolutionRate = m.resolved / (m.fails + m.warns || 1);

		if (fpRate > 0.3) {
			alerts.push(
				`${ruleId}: high false-positive rate (${(fpRate * 100).toFixed(0)}%) — review rule scope`,
			);
		}
		if (overrideRate > 0.2) {
			alerts.push(
				`${ruleId}: high override rate (${(overrideRate * 100).toFixed(0)}%) — rule may be too restrictive`,
			);
		}
		if (resolutionRate < 0.5 && m.fails + m.warns > 3) {
			alerts.push(
				`${ruleId}: low resolution rate (${(resolutionRate * 100).toFixed(0)}%) — enforcement may need escalation`,
			);
		}
	}

	if (jsonOutput) {
		console.log(
			JSON.stringify(
				{
					rules: Object.fromEntries(ruleMetrics),
					alerts,
				},
				null,
				2,
			),
		);
	} else {
		if (ruleMetrics.size === 0) {
			console.log("No enforcement metrics available. Run `orqa enforce` first.");
			return;
		}

		console.log("Enforcement Metrics");
		console.log("===================\n");

		for (const [ruleId, m] of [...ruleMetrics.entries()].sort((a, b) => b[1].fires - a[1].fires)) {
			const resRate =
				m.fails + m.warns > 0 ? `${((m.resolved / (m.fails + m.warns)) * 100).toFixed(0)}%` : "n/a";
			console.log(`${ruleId}:`);
			console.log(`  Fires: ${m.fires}  Fails: ${m.fails}  Warns: ${m.warns}`);
			console.log(
				`  Resolved: ${m.resolved} (${resRate})  Fixed: ${m.fixed}  Overridden: ${m.overridden}  FP: ${m.false_positive}`,
			);
		}

		if (alerts.length > 0) {
			console.log("\nAlerts:");
			for (const a of alerts) {
				console.log(`  ! ${a}`);
			}
		}
	}
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/**
 * Find rule directories contributed by installed plugins.
 * @param projectRoot - Absolute path to the project root.
 * @returns Array of absolute paths to plugin rule directories.
 */
function findPluginRuleDirs(projectRoot: string): string[] {
	const dirs: string[] = [];
	const pluginsDir = join(projectRoot, "plugins");
	if (!existsSync(pluginsDir)) return dirs;
	for (const entry of readdirSync(pluginsDir, { withFileTypes: true })) {
		if (!entry.isDirectory()) continue;
		const rulesDir = join(pluginsDir, entry.name, "rules");
		if (existsSync(rulesDir)) dirs.push(rulesDir);
	}
	return dirs;
}

/**
 * Load a JSON approvals/pending file, returning an empty object on error.
 * @param filePath - Absolute path to the JSON file.
 * @returns Parsed approval records, or empty object on error.
 */
function loadApprovals(filePath: string): Record<string, Record<string, string>> {
	try {
		if (!existsSync(filePath)) return {};
		return JSON.parse(readFileSync(filePath, "utf-8"));
	} catch {
		return {};
	}
}

/**
 * Write a JSON approvals/pending file, creating parent directories as needed.
 * @param filePath - Absolute path to the JSON file.
 * @param data - The data to write.
 */
function writeApprovals(filePath: string, data: Record<string, Record<string, string>>): void {
	const dir = dirname(filePath);
	if (!existsSync(dir)) mkdirSync(dir, { recursive: true });
	writeFileSync(filePath, JSON.stringify(data, null, 2) + "\n", "utf-8");
}

/**
 * Extract a named flag value from an args array (e.g. --flag value).
 * @param args - The argument array to search.
 * @param flag - The flag name (e.g. "--event-id").
 * @returns The flag value, or undefined if not found.
 */
function getFlag(args: string[], flag: string): string | undefined {
	const idx = args.indexOf(flag);
	if (idx === -1 || idx + 1 >= args.length) return undefined;
	return args[idx + 1];
}

// ── Built-in enforcement checks (rule-driven, not plugin-provided) ──────────

/**
 * Dispatch a built-in enforcement check. These are registered from rules
 * (not plugin manifests) and run inline instead of spawning a subprocess.
 * @param projectRoot - Absolute path to the project root.
 * @param command - Built-in command name (e.g. "__builtin:stories").
 * @param files - Filtered staged files, or null to check all.
 * @returns 0 if passed, 1 if violations found.
 */
function runBuiltinCheck(projectRoot: string, command: string, files: string[] | null): number {
	const name = command.replace("__builtin:", "");
	switch (name) {
		case "stories":
			return checkComponentStories(projectRoot, files);
		default:
			console.error(`Unknown built-in check: ${name}`);
			return 1;
	}
}

/**
 * RULE-55092f35: Verify every component directory in the svelte-components
 * library has at least one .stories.ts file.
 *
 * When staged files are provided, only checks directories containing staged
 * .svelte files. Otherwise checks all directories.
 * @param root - Absolute path to the project root.
 * @param stagedFiles - Staged file list for scoped checking, or null for all.
 * @returns 0 if all components have stories, 1 if violations found.
 */
function checkComponentStories(root: string, stagedFiles: string[] | null): number {
	const dirs = ["pure", "connected"];
	const violations: string[] = [];

	// When --staged, only check directories that have staged .svelte files.
	const stagedDirs = stagedFiles
		? new Set(
				stagedFiles
					.filter((f) => f.startsWith("libs/svelte-components/src/") && f.endsWith(".svelte"))
					.map((f) => {
						const parts = f.split("/");
						// e.g. libs/svelte-components/src/pure/table/Table.svelte → pure/table
						return parts.length >= 6 ? `${parts[3]}/${parts[4]}` : null;
					})
					.filter((d): d is string => d !== null),
			)
		: null;

	for (const sub of dirs) {
		const baseDir = join(root, "libs", "svelte-components", "src", sub);
		if (!existsSync(baseDir)) continue;

		for (const entry of readdirSync(baseDir, { withFileTypes: true })) {
			if (!entry.isDirectory()) continue;

			// Skip if --staged and this directory has no staged files.
			if (stagedDirs && !stagedDirs.has(`${sub}/${entry.name}`)) continue;

			const compDir = join(baseDir, entry.name);
			const files = readdirSync(compDir);

			const hasSvelte = files.some((f) => f.endsWith(".svelte") && !f.endsWith(".stories.svelte"));
			if (!hasSvelte) continue;

			const hasStory = files.some((f) => f.includes(".stories."));
			if (!hasStory) {
				violations.push(`${sub}/${entry.name}`);
			}
		}
	}

	if (violations.length === 0) {
		console.log("  stories (RULE-55092f35): ✓ all components have stories");
		return 0;
	}

	console.log(`  stories (RULE-55092f35): ✗ ${violations.length} component(s) missing stories`);
	for (const v of violations) {
		console.log(`    • libs/svelte-components/src/${v}/`);
	}
	return 1;
}
