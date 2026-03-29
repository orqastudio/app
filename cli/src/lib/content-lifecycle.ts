/**
 * Plugin content lifecycle — install, remove, diff, and refresh plugin content.
 *
 * Plugins declare content mappings in `orqa-plugin.json`:
 *   { "content": { "rules": { "source": "rules", "target": ".orqa/learning/rules" } } }
 *
 * When installed, plugin content is copied from plugin source dirs to `.orqa/` target
 * dirs under the project root. Ownership is tracked in `.orqa/manifest.json`.
 */

import * as fs from "node:fs";
import * as path from "node:path";
import { createHash } from "node:crypto";
import { execSync } from "node:child_process";
import type { PluginManifest, PluginContentMapping } from "@orqastudio/types";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export interface FileHashEntry {
	sourceHash: string;
	installedHash: string;
}

export type ThreeWayState = "clean" | "plugin-updated" | "user-modified" | "conflict" | "missing";

export interface ThreeWayFileStatus {
	path: string;
	state: ThreeWayState;
}

export interface CopyResult {
	copied: Record<string, FileHashEntry>;
	skipped: ThreeWayFileStatus[];
}

export interface ContentManifest {
	plugins: Record<string, ContentManifestEntry>;
}

export interface ContentManifestEntry {
	version: string;
	installed_at: string;
	/** Relative paths from project root, using forward slashes → hash entries. */
	files: Record<string, FileHashEntry>;
}

export interface ContentDiffResult {
	pluginName: string;
	/** Files whose content is identical between plugin source and .orqa/ copy. */
	identical: string[];
	/** Files in .orqa/ that differ from the plugin source. */
	modified: string[];
	/** Files in the manifest but deleted from .orqa/. */
	missing: string[];
	/** Files found in the plugin's target dirs that are not in the manifest. */
	orphaned: string[];
	/** Three-way state for each tracked file. */
	threeWay: ThreeWayFileStatus[];
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const MANIFEST_FILENAME = ".orqa/manifest.json";

// ---------------------------------------------------------------------------
// Manifest I/O
// ---------------------------------------------------------------------------

/**
 * Read `.orqa/manifest.json` from the project root.
 * Returns an empty manifest if the file does not exist.
 * @param projectRoot - Absolute path to the project root.
 * @returns The parsed content manifest, or an empty manifest if not found.
 */
export function readContentManifest(projectRoot: string): ContentManifest {
	const manifestPath = path.join(projectRoot, MANIFEST_FILENAME);

	if (!fs.existsSync(manifestPath)) {
		return { plugins: {} };
	}

	const raw = fs.readFileSync(manifestPath, "utf-8");
	return JSON.parse(raw) as ContentManifest;
}

/**
 * Write `.orqa/manifest.json` to the project root with pretty-printed JSON.
 * @param projectRoot - Absolute path to the project root.
 * @param manifest - The content manifest to write.
 */
export function writeContentManifest(projectRoot: string, manifest: ContentManifest): void {
	const manifestPath = path.join(projectRoot, MANIFEST_FILENAME);
	const dir = path.dirname(manifestPath);

	if (!fs.existsSync(dir)) {
		fs.mkdirSync(dir, { recursive: true });
	}

	fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + "\n", "utf-8");
}

// ---------------------------------------------------------------------------
// Content Copy & Removal
// ---------------------------------------------------------------------------

/**
 * Copy files from the plugin's content source dirs to the project's target dirs.
 *
 * Supports three-way diff: when `currentManifest` is provided, files that have
 * been modified by the user (user-modified or conflict) are skipped.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param projectRoot - Absolute path to the project root.
 * @param manifest - The plugin's `orqa-plugin.json` manifest.
 * @param currentManifest - Optional existing content manifest for three-way state.
 * @returns CopyResult with copied files (with hashes) and skipped files.
 */
export function copyPluginContent(
	pluginDir: string,
	projectRoot: string,
	manifest: PluginManifest,
	currentManifest?: ContentManifest,
): CopyResult {
	if (!manifest.content || Object.keys(manifest.content).length === 0) {
		return { copied: {}, skipped: [] };
	}

	const copied: Record<string, FileHashEntry> = {};
	const skipped: ThreeWayFileStatus[] = [];
	const existingEntry = currentManifest?.plugins[manifest.name];

	for (const [, mapping] of Object.entries(manifest.content)) {
		// Handle "extends" strategy
		if (mapping.strategy === "extends") {
			setupConfigExtends(pluginDir, projectRoot, mapping);
			continue;
		}

		const sourceDir = path.join(pluginDir, mapping.source);
		const targetDir = path.join(projectRoot, mapping.target);

		if (!fs.existsSync(sourceDir)) {
			continue;
		}

		if (!fs.existsSync(targetDir)) {
			fs.mkdirSync(targetDir, { recursive: true });
		}

		const entries = fs.readdirSync(sourceDir, { withFileTypes: true });

		for (const entry of entries) {
			if (!entry.isFile()) {
				continue;
			}

			const srcFile = path.join(sourceDir, entry.name);
			const dstFile = path.join(targetDir, entry.name);
			const relativePath = path.join(mapping.target, entry.name).replace(/\\/g, "/");

			// Compute source hash before copy
			const sourceHash = computeFileHash(srcFile);

			// Three-way state check when we have an existing manifest
			if (existingEntry && existingEntry.files[relativePath]) {
				const state = computeThreeWayState(
					relativePath,
					projectRoot,
					existingEntry.files[relativePath],
					sourceHash,
				);
				if (state === "user-modified" || state === "conflict") {
					skipped.push({ path: relativePath, state });
					continue;
				}
			}

			fs.copyFileSync(srcFile, dstFile);

			// Compute installed hash after copy
			const installedHash = computeFileHash(dstFile);

			copied[relativePath] = { sourceHash, installedHash };
		}
	}

	return { copied, skipped };
}

/**
 * Remove all content files belonging to a plugin and update the manifest.
 * @param pluginName - The plugin's `name` field from its manifest.
 * @param projectRoot - Absolute path to the project root.
 */
export function removePluginContent(pluginName: string, projectRoot: string): void {
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

	const updated: ContentManifest = {
		plugins: { ...contentManifest.plugins },
	};
	delete updated.plugins[pluginName];

	writeContentManifest(projectRoot, updated);
}

// ---------------------------------------------------------------------------
// Dependencies & Build
// ---------------------------------------------------------------------------

/**
 * Install plugin dependencies.
 *
 * - If `pluginManifest.dependencies.npm` is non-empty, runs `npm install` in pluginDir.
 * - If `pluginManifest.dependencies.system` is non-empty, checks each binary exists.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param pluginManifest - The plugin's manifest.
 * @throws {Error} If any required system binary is not found on PATH.
 */
export function installPluginDeps(pluginDir: string, pluginManifest: PluginManifest): void {
	const deps = pluginManifest.dependencies;

	if (!deps) {
		return;
	}

	if (Array.isArray(deps.npm) && deps.npm.length > 0) {
		const hasNodeModules = fs.existsSync(path.join(pluginDir, "node_modules"));
		const hasPackageJson = fs.existsSync(path.join(pluginDir, "package.json"));

		// Skip npm install for workspace members — their deps are managed by the
		// root package.json. Running npm install inside a workspace member breaks
		// npm's arborist ("Cannot read properties of null").
		const isWorkspaceMember = isInsideWorkspace(pluginDir);

		if (!hasNodeModules && hasPackageJson && !isWorkspaceMember) {
			execSync("npm install", { cwd: pluginDir, stdio: "inherit" });
		}
	}

	if (Array.isArray(deps.system) && deps.system.length > 0) {
		const missing: string[] = [];

		for (const req of deps.system) {
			if (!isBinaryAvailable(req.binary)) {
				missing.push(req.binary);
			}
		}

		if (missing.length > 0) {
			throw new Error(
				`Plugin "${pluginManifest.name}" requires system binaries that were not found: ${missing.join(", ")}`,
			);
		}
	}
}

/**
 * Run the plugin's build command, if declared.
 * @param pluginDir - Absolute path to the plugin directory (cwd for the command).
 * @param pluginManifest - The plugin's manifest.
 */
export function buildPlugin(pluginDir: string, pluginManifest: PluginManifest): void {
	if (!pluginManifest.build) {
		return;
	}

	execSync(pluginManifest.build, { cwd: pluginDir, stdio: "inherit" });
}

// ---------------------------------------------------------------------------
// Lifecycle Hooks
// ---------------------------------------------------------------------------

/**
 * Run a plugin lifecycle hook command (`install` or `uninstall`), if declared.
 * @param pluginDir - Absolute path to the plugin directory (cwd for the command).
 * @param pluginManifest - The plugin's manifest.
 * @param hook - Which hook to run.
 */
export function runLifecycleHook(
	pluginDir: string,
	pluginManifest: PluginManifest,
	hook: "install" | "uninstall",
): void {
	const command = pluginManifest.lifecycle?.[hook];

	if (!command) {
		return;
	}

	execSync(command, { cwd: pluginDir, stdio: "inherit" });
}

// ---------------------------------------------------------------------------
// Diff
// ---------------------------------------------------------------------------

/**
 * Compare the plugin's source content against the installed copies in `.orqa/`.
 *
 * For each file tracked in the content manifest:
 * - If it no longer exists in `.orqa/`: listed as `missing`.
 * - If its content matches the plugin source: listed as `identical`.
 * - If its content differs: listed as `modified`.
 *
 * Files found in the plugin's target dirs that are NOT in the manifest are `orphaned`.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param projectRoot - Absolute path to the project root.
 * @param pluginManifest - The plugin's manifest.
 * @returns Diff result categorizing files as identical, modified, missing, or orphaned.
 */
export function diffPluginContent(
	pluginDir: string,
	projectRoot: string,
	pluginManifest: PluginManifest,
): ContentDiffResult {
	const result: ContentDiffResult = {
		pluginName: pluginManifest.name,
		identical: [],
		modified: [],
		missing: [],
		orphaned: [],
		threeWay: [],
	};

	const contentManifest = readContentManifest(projectRoot);
	const entry = contentManifest.plugins[pluginManifest.name];
	const trackedFiles = new Set(Object.keys(entry?.files ?? {}));

	// Categorise each tracked file
	for (const relPath of trackedFiles) {
		const installedPath = path.join(projectRoot, relPath);

		if (!fs.existsSync(installedPath)) {
			result.missing.push(relPath);
			result.threeWay.push({ path: relPath, state: "missing" });
			continue;
		}

		// Find the corresponding source file in the plugin
		const sourceFile = findSourceFile(pluginDir, pluginManifest, relPath);

		if (!sourceFile || !fs.existsSync(sourceFile)) {
			// Source no longer exists — treat as modified (stale install)
			result.modified.push(relPath);
			continue;
		}

		const installedContent = fs.readFileSync(installedPath);
		const sourceContent = fs.readFileSync(sourceFile);

		if (installedContent.equals(sourceContent)) {
			result.identical.push(relPath);
			result.threeWay.push({ path: relPath, state: "clean" });
		} else {
			result.modified.push(relPath);
			// Compute detailed three-way state if we have hashes
			if (entry?.files[relPath]) {
				const sourceHash = computeFileHash(sourceFile);
				const state = computeThreeWayState(
					relPath,
					projectRoot,
					entry.files[relPath],
					sourceHash,
				);
				result.threeWay.push({ path: relPath, state });
			} else {
				result.threeWay.push({ path: relPath, state: "plugin-updated" });
			}
		}
	}

	// Find orphaned files in target dirs — files not tracked by ANY plugin
	if (pluginManifest.content) {
		// Build set of ALL tracked files across ALL plugins
		const allTrackedFiles = new Set<string>();
		for (const [, pluginEntry] of Object.entries(contentManifest.plugins)) {
			for (const f of Object.keys(pluginEntry.files)) {
				allTrackedFiles.add(f);
			}
		}

		for (const [, mapping] of Object.entries(pluginManifest.content)) {
			const targetDir = path.join(projectRoot, mapping.target);

			if (!fs.existsSync(targetDir)) {
				continue;
			}

			const entries = fs.readdirSync(targetDir, { withFileTypes: true });

			for (const dirEntry of entries) {
				if (!dirEntry.isFile()) {
					continue;
				}

				const relPath = path.join(mapping.target, dirEntry.name).replace(/\\/g, "/");

				// Only orphaned if not tracked by ANY plugin
				if (!allTrackedFiles.has(relPath)) {
					result.orphaned.push(relPath);
				}
			}
		}
	}

	return result;
}

// ---------------------------------------------------------------------------
// Refresh
// ---------------------------------------------------------------------------

/**
 * Re-install a plugin's dependencies, rebuild, re-copy content, and update the manifest.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param projectRoot - Absolute path to the project root.
 * @param pluginManifest - The plugin's manifest.
 * @returns CopyResult with copied and skipped files.
 */
export function refreshPluginContent(
	pluginDir: string,
	projectRoot: string,
	pluginManifest: PluginManifest,
): CopyResult {
	installPluginDeps(pluginDir, pluginManifest);
	buildPlugin(pluginDir, pluginManifest);

	// Read existing manifest for three-way state
	const existingManifest = readContentManifest(projectRoot);
	const copyResult = copyPluginContent(pluginDir, projectRoot, pluginManifest, existingManifest);

	// Merge: skipped files retain their existing hashes
	const mergedFiles: Record<string, FileHashEntry> = { ...copyResult.copied };
	const existingEntry = existingManifest.plugins[pluginManifest.name];
	if (existingEntry) {
		for (const skipped of copyResult.skipped) {
			const existing = existingEntry.files[skipped.path];
			if (existing) {
				mergedFiles[skipped.path] = existing;
			}
		}
	}

	// Update the content manifest
	const contentManifest = readContentManifest(projectRoot);
	contentManifest.plugins[pluginManifest.name] = {
		version: pluginManifest.version,
		installed_at: new Date().toISOString(),
		files: mergedFiles,
	};
	writeContentManifest(projectRoot, contentManifest);

	return copyResult;
}

// ---------------------------------------------------------------------------
// Three-way state helpers
// ---------------------------------------------------------------------------

/**
 * Compute the three-way state of a file by comparing:
 * - The installed file on disk vs the installedHash at last install
 * - The current source hash vs the sourceHash at last install
 *
 * States:
 * - "clean": neither user nor plugin changed it
 * - "plugin-updated": plugin has a newer version, user hasn't touched it
 * - "user-modified": user changed it, plugin hasn't updated
 * - "conflict": both user and plugin changed it
 * - "missing": file doesn't exist on disk
 * @param relPath - Relative path from project root to the file.
 * @param projectRoot - Absolute path to the project root.
 * @param lastEntry - The hash entry from the last install.
 * @param currentSourceHash - Current hash of the file in the plugin source.
 * @returns The three-way state of the file.
 */
export function computeThreeWayState(
	relPath: string,
	projectRoot: string,
	lastEntry: FileHashEntry,
	currentSourceHash: string,
): ThreeWayState {
	const absPath = path.join(projectRoot, relPath);

	if (!fs.existsSync(absPath)) {
		return "missing";
	}

	const currentInstalledHash = computeFileHash(absPath);
	const userChanged = currentInstalledHash !== lastEntry.installedHash;
	const pluginChanged = currentSourceHash !== lastEntry.sourceHash;

	if (!userChanged && !pluginChanged) return "clean";
	if (pluginChanged && !userChanged) return "plugin-updated";
	if (userChanged && !pluginChanged) return "user-modified";
	return "conflict";
}

/**
 * Compute the SHA-256 hash of a file.
 * @param filePath - Absolute path to the file to hash.
 * @returns Hex-encoded SHA-256 hash of the file contents.
 */
export function computeFileHash(filePath: string): string {
	const content = fs.readFileSync(filePath);
	return createHash("sha256").update(content).digest("hex");
}

/**
 * Check if a directory is inside an npm workspace (root package.json has "workspaces").
 * @param dir - Absolute path to the directory to check.
 * @returns True if the directory is inside an npm workspace.
 */
function isInsideWorkspace(dir: string): boolean {
	let current = path.dirname(dir);
	while (current !== path.dirname(current)) {
		const pkgPath = path.join(current, "package.json");
		if (fs.existsSync(pkgPath)) {
			try {
				const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf-8"));
				if (Array.isArray(pkg.workspaces) && pkg.workspaces.length > 0) {
					return true;
				}
			} catch { /* ignore */ }
		}
		current = path.dirname(current);
	}
	return false;
}

// ---------------------------------------------------------------------------
// Symlink and aggregated file processing
// ---------------------------------------------------------------------------

/**
 * Process symlink declarations from a plugin manifest.
 * Creates symlinks in the project directory as declared.
 * @param projectRoot - Absolute path to the project root.
 * @param pluginManifest - The plugin's manifest containing symlink declarations.
 */
export function processPluginSymlinks(
	projectRoot: string,
	pluginManifest: PluginManifest,
): void {
	const symlinks = pluginManifest.provides?.symlinks;
	if (!symlinks || symlinks.length === 0) return;

	for (const decl of symlinks) {
		const source = path.join(projectRoot, decl.source);
		const target = path.join(projectRoot, decl.target);

		if (!fs.existsSync(source)) continue;

		// Ensure target parent directory exists
		const targetParent = path.dirname(target);
		if (!fs.existsSync(targetParent)) {
			fs.mkdirSync(targetParent, { recursive: true });
		}

		// Remove existing target if it's a symlink or file
		try {
			const stat = fs.lstatSync(target);
			if (stat.isSymbolicLink() || stat.isFile()) {
				fs.unlinkSync(target);
			}
		} catch {
			// Target doesn't exist — fine
		}

		// Create symlink
		try {
			const isDir = fs.statSync(source).isDirectory();
			fs.symlinkSync(source, target, isDir ? "dir" : "file");
		} catch {
			// Symlink creation may fail on some systems — non-fatal
		}
	}
}

/**
 * Process aggregated file declarations from all installed plugins.
 * Collects values from plugin manifests and writes them to a single output file.
 * @param projectRoot - Absolute path to the project root.
 */
export function processAggregatedFiles(
	projectRoot: string,
): void {
	// Scan all plugin manifests for aggregatedFiles declarations
	const scanDirs = [
		path.join(projectRoot, "plugins"),
		path.join(projectRoot, "connectors"),
		path.join(projectRoot, "sidecars"),
	];

	// Collect all aggregatedFile declarations first
	interface AggregatedDecl {
		target: string;
		collect: string;
		relativeToPlugin?: boolean;
		wrapper?: string;
		stripFields?: string[];
		pluginDir: string;
	}

	const declarations: AggregatedDecl[] = [];

	for (const scanDir of scanDirs) {
		if (!fs.existsSync(scanDir)) continue;

		for (const entry of fs.readdirSync(scanDir, { withFileTypes: true })) {
			if (!entry.isDirectory() || entry.name.startsWith(".")) continue;

			const pluginDir = path.join(scanDir, entry.name);
			const manifestPath = path.join(pluginDir, "orqa-plugin.json");
			if (!fs.existsSync(manifestPath)) continue;

			try {
				const raw = fs.readFileSync(manifestPath, "utf-8");
				const manifest = JSON.parse(raw) as PluginManifest;
				const aggFiles = manifest.provides?.aggregatedFiles;
				if (!aggFiles || aggFiles.length === 0) continue;

				for (const aggFile of aggFiles) {
					declarations.push({ ...aggFile, pluginDir });
				}
			} catch {
				// Skip invalid manifests
			}
		}
	}

	// Group declarations by target file
	const byTarget = new Map<string, AggregatedDecl[]>();
	for (const decl of declarations) {
		const list = byTarget.get(decl.target) ?? [];
		list.push(decl);
		byTarget.set(decl.target, list);
	}

	// Process each target file
	for (const [target, decls] of byTarget) {
		const collected: Record<string, unknown> = {};

		for (const decl of decls) {
			// Re-scan all plugin manifests to collect the specified field
			for (const scanDir of scanDirs) {
				if (!fs.existsSync(scanDir)) continue;

				for (const entry of fs.readdirSync(scanDir, { withFileTypes: true })) {
					if (!entry.isDirectory() || entry.name.startsWith(".")) continue;

					const pluginDir = path.join(scanDir, entry.name);
					const manifestPath = path.join(pluginDir, "orqa-plugin.json");
					if (!fs.existsSync(manifestPath)) continue;

					try {
						const raw = fs.readFileSync(manifestPath, "utf-8");
						const manifest = JSON.parse(raw) as Record<string, unknown>;
						const value = getNestedField(manifest, decl.collect);

						if (value && typeof value === "object" && !Array.isArray(value)) {
							for (const [key, val] of Object.entries(value as Record<string, unknown>)) {
								if (key in collected) continue; // First declaration wins

								let processedVal = val;

								// Strip specified fields
								if (decl.stripFields && processedVal && typeof processedVal === "object") {
									const copy = { ...(processedVal as Record<string, unknown>) };
									for (const field of decl.stripFields) {
										delete copy[field];
									}
									processedVal = copy;
								}

								collected[key] = processedVal;
							}
						}
					} catch {
						// Skip invalid manifests
					}
				}
			}
		}

		// Write the output file
		const outputPath = path.join(projectRoot, target);
		const outputDir = path.dirname(outputPath);
		if (!fs.existsSync(outputDir)) {
			fs.mkdirSync(outputDir, { recursive: true });
		}

		// Apply wrapper if specified
		const firstDecl = decls[0];
		const output = firstDecl.wrapper
			? { [firstDecl.wrapper]: collected }
			: collected;

		const newContent = JSON.stringify(output, null, 2);
		const existingContent = fs.existsSync(outputPath)
			? fs.readFileSync(outputPath, "utf-8")
			: "";

		if (newContent !== existingContent) {
			fs.writeFileSync(outputPath, newContent);
		}
	}
}

// ---------------------------------------------------------------------------
// Config extends
// ---------------------------------------------------------------------------

/**
 * Set up config extension for a content mapping with strategy "extends".
 * Creates the target config file that extends the source.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param projectRoot - Absolute path to the project root.
 * @param mapping - The content mapping declaring the extension.
 */
export function setupConfigExtends(
	pluginDir: string,
	projectRoot: string,
	mapping: PluginContentMapping,
): void {
	const mechanism = mapping.mechanism ?? "json-extends";

	if (mechanism === "json-extends") {
		setupJsonExtends(pluginDir, projectRoot, mapping);
	} else if (mechanism === "js-import") {
		setupJsImportExtends(pluginDir, projectRoot, mapping);
	}
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/**
 * Check whether a binary is available on the system PATH.
 * Tries `which` (Unix) and `where` (Windows) — handles both.
 * @param binary - The binary name to check for.
 * @returns True if the binary is available on PATH.
 */
function isBinaryAvailable(binary: string): boolean {
	for (const checker of [`which ${binary}`, `where ${binary}`]) {
		try {
			execSync(checker, { stdio: ["pipe", "pipe", "pipe"] });
			return true;
		} catch {
			// Not found via this checker — try the next
		}
	}
	return false;
}

/**
 * Given a relative path from the project root (e.g. `.orqa/learning/rules/RULE-abc.md`),
 * find the corresponding source file in the plugin directory by matching target mappings.
 *
 * Returns the absolute path to the source file, or null if no mapping covers this path.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param pluginManifest - The plugin's manifest containing content mappings.
 * @param relPath - Relative path from the project root to the installed file.
 * @returns Absolute path to the source file, or null if no mapping covers this path.
 */
export function findSourceFile(
	pluginDir: string,
	pluginManifest: PluginManifest,
	relPath: string,
): string | null {
	if (!pluginManifest.content) {
		return null;
	}

	// Normalise to forward slashes for comparison
	const normRelPath = relPath.replace(/\\/g, "/");

	for (const [, mapping] of Object.entries(pluginManifest.content)) {
		const normTarget = mapping.target.replace(/\\/g, "/").replace(/\/$/, "");

		if (!normRelPath.startsWith(`${normTarget}/`)) {
			continue;
		}

		const filename = normRelPath.slice(normTarget.length + 1);
		return path.join(pluginDir, mapping.source, filename);
	}

	return null;
}

/**
 * Set up JSON extends — creates a target file with an "extends" field pointing
 * to the source.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param projectRoot - Absolute path to the project root.
 * @param mapping - The content mapping declaring the JSON extension.
 */
function setupJsonExtends(
	pluginDir: string,
	projectRoot: string,
	mapping: PluginContentMapping,
): void {
	const targetPath = path.join(projectRoot, mapping.target);
	const sourcePath = path.join(pluginDir, mapping.source);

	if (!fs.existsSync(sourcePath)) return;

	// Compute relative path from target to source
	const relSource = path.relative(path.dirname(targetPath), sourcePath).replace(/\\/g, "/");

	if (fs.existsSync(targetPath)) {
		// Only update if the file doesn't already extend the source
		try {
			const existing = JSON.parse(fs.readFileSync(targetPath, "utf-8"));
			if (existing.extends === relSource) return;
		} catch {
			// File isn't valid JSON — will be overwritten
		}
	}

	const targetDir = path.dirname(targetPath);
	if (!fs.existsSync(targetDir)) {
		fs.mkdirSync(targetDir, { recursive: true });
	}

	fs.writeFileSync(targetPath, JSON.stringify({ extends: relSource }, null, 2) + "\n", "utf-8");
}

/**
 * Set up JS import extends — creates a target JS file that re-exports from
 * the source.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param projectRoot - Absolute path to the project root.
 * @param mapping - The content mapping declaring the JS import extension.
 */
function setupJsImportExtends(
	pluginDir: string,
	projectRoot: string,
	mapping: PluginContentMapping,
): void {
	const targetPath = path.join(projectRoot, mapping.target);
	const sourcePath = path.join(pluginDir, mapping.source);

	if (!fs.existsSync(sourcePath)) return;

	// Compute relative path from target to source
	const relSource = path.relative(path.dirname(targetPath), sourcePath).replace(/\\/g, "/");

	const content = `export { default } from "${relSource}";\n`;

	if (fs.existsSync(targetPath)) {
		const existing = fs.readFileSync(targetPath, "utf-8");
		if (existing === content) return;
	}

	const targetDir = path.dirname(targetPath);
	if (!fs.existsSync(targetDir)) {
		fs.mkdirSync(targetDir, { recursive: true });
	}

	fs.writeFileSync(targetPath, content, "utf-8");
}

/**
 * Get a nested field value from an object using dot-path notation.
 * e.g. getNestedField(obj, "provides.mcpServers") → obj.provides.mcpServers
 * @param obj - The object to traverse.
 * @param dotPath - Dot-separated path to the desired field.
 * @returns The value at the specified path, or undefined if not found.
 */
function getNestedField(obj: Record<string, unknown>, dotPath: string): unknown {
	const parts = dotPath.split(".");
	let current: unknown = obj;
	for (const part of parts) {
		if (current === null || current === undefined || typeof current !== "object") {
			return undefined;
		}
		current = (current as Record<string, unknown>)[part];
	}
	return current;
}
