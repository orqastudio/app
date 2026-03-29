/**
 * Migrate command — apply status migrations from workflow definitions.
 *
 * orqa migrate [options]
 *
 * Reads migration mappings from .orqa/workflows/*.resolved.yaml and updates
 * artifact frontmatter `status` fields accordingly. Only changes status values
 * that differ between old and new (i.e., actual renames, not identity mappings).
 */

import { type Dirent, readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { parse as parseYaml } from "yaml";
import { getRoot } from "../lib/root.js";
import {
	parseFrontmatterFromContent,
	writeFrontmatter,
} from "../lib/frontmatter.js";

const USAGE = `
Usage: orqa migrate [options]

Apply status migrations from workflow definitions to artifact frontmatter.

Reads migration mappings from .orqa/workflows/*.resolved.yaml and updates
each artifact's status field when it matches a migration "from" value.

Options:
  --dry-run            Preview changes without writing files
  --type <type>        Migrate only artifacts of this type (e.g., task, epic)
  --verbose            Show identity mappings (status unchanged) in output
  --help, -h           Show this help message

Examples:
  orqa migrate --dry-run          Preview all migrations
  orqa migrate                    Apply all migrations
  orqa migrate --type task        Migrate only task artifacts
`.trim();

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface MigrationMapping {
	artifactType: string;
	workflowName: string;
	/** old status -> new state */
	mappings: Record<string, string>;
}

interface MigrationChange {
	file: string;
	artifactId: string;
	artifactType: string;
	oldStatus: string;
	newStatus: string;
}

// ---------------------------------------------------------------------------
// Workflow scanning
// ---------------------------------------------------------------------------

/**
 * Read all resolved workflow files and extract migration mappings.
 * Only returns mappings where at least one entry actually changes the status
 * (from !== to).
 * @param projectRoot - Absolute path to the project root.
 * @returns Array of migration mappings from resolved workflow files.
 */
function loadMigrationMappings(projectRoot: string): MigrationMapping[] {
	const workflowDir = join(projectRoot, ".orqa", "workflows");
	let entries: Dirent[];
	try {
		entries = readdirSync(workflowDir, { withFileTypes: true, encoding: "utf-8" }) as Dirent[];
	} catch {
		return [];
	}

	const mappings: MigrationMapping[] = [];

	for (const entry of entries) {
		if (!entry.name.endsWith(".resolved.yaml")) continue;

		const filePath = join(workflowDir, entry.name);
		let content: string;
		try {
			content = readFileSync(filePath, "utf-8");
		} catch {
			continue;
		}

		let parsed: Record<string, unknown>;
		try {
			parsed = parseYaml(content) as Record<string, unknown>;
		} catch {
			continue;
		}

		const artifactType = parsed.artifact_type;
		const workflowName = parsed.name;
		const migration = parsed.migration;

		if (
			typeof artifactType !== "string" ||
			typeof workflowName !== "string" ||
			!migration ||
			typeof migration !== "object"
		) {
			continue;
		}

		const migrationMap = migration as Record<string, string>;

		// Only include if there are actual renames (from !== to)
		const hasRenames = Object.entries(migrationMap).some(
			([from, to]) => from !== to,
		);
		if (!hasRenames) continue;

		mappings.push({
			artifactType: artifactType,
			workflowName: workflowName as string,
			mappings: migrationMap,
		});
	}

	return mappings;
}

// ---------------------------------------------------------------------------
// Artifact scanning
// ---------------------------------------------------------------------------

/**
 * Recursively walk directories collecting .md files.
 * @param dir - Directory to walk.
 * @param results - Accumulator for collected paths.
 * @returns Array of absolute paths to .md files.
 */
function walkFiles(dir: string, results: string[] = []): string[] {
	let entries: Dirent[];
	try {
		entries = readdirSync(dir, { withFileTypes: true, encoding: "utf-8" }) as Dirent[];
	} catch {
		return results;
	}
	for (const entry of entries) {
		if (
			entry.name.startsWith(".") ||
			entry.name === "node_modules" ||
			entry.name === "dist" ||
			entry.name === "target" ||
			entry.name === "workflows"
		) {
			continue;
		}
		const full = join(dir, entry.name);
		if (entry.isDirectory()) walkFiles(full, results);
		else if (entry.name.endsWith(".md")) results.push(full);
	}
	return results;
}

/**
 * Find all artifacts in .orqa/ directories and return them grouped by type.
 * @param projectRoot - Absolute path to the project root.
 * @returns Map of artifact type to list of artifact summaries.
 */
function scanArtifacts(
	projectRoot: string,
): Map<string, Array<{ file: string; id: string; status: string }>> {
	const scanDirs = [
		join(projectRoot, ".orqa"),
		join(projectRoot, "app", ".orqa"),
	];

	const allFiles = scanDirs.flatMap((d) => walkFiles(d));
	const byType = new Map<
		string,
		Array<{ file: string; id: string; status: string }>
	>();

	for (const file of allFiles) {
		let content: string;
		try {
			content = readFileSync(file, "utf-8");
		} catch {
			continue;
		}

		const parsed = parseFrontmatterFromContent(content);
		if (!parsed) continue;

		const [fm] = parsed;
		const type = typeof fm.type === "string" ? fm.type.trim() : "";
		const id = typeof fm.id === "string" ? fm.id.trim() : "";
		const status = typeof fm.status === "string" ? fm.status.trim() : "";

		if (!type || !id || !status) continue;

		const list = byType.get(type) ?? [];
		list.push({ file, id, status });
		byType.set(type, list);
	}

	return byType;
}

// ---------------------------------------------------------------------------
// Migration logic
// ---------------------------------------------------------------------------

/**
 * Compute the set of changes that would be applied.
 * @param projectRoot - Absolute path to the project root.
 * @param mappings - Migration mappings from resolved workflows.
 * @param typeFilter - Limit changes to this artifact type, or null for all.
 * @returns Array of pending migration changes.
 */
function computeChanges(
	projectRoot: string,
	mappings: MigrationMapping[],
	typeFilter: string | null,
): MigrationChange[] {
	const artifacts = scanArtifacts(projectRoot);
	const changes: MigrationChange[] = [];

	for (const mapping of mappings) {
		if (typeFilter && mapping.artifactType !== typeFilter) continue;

		const artifactsOfType = artifacts.get(mapping.artifactType);
		if (!artifactsOfType) continue;

		for (const artifact of artifactsOfType) {
			const newStatus = mapping.mappings[artifact.status];
			if (newStatus === undefined) continue;
			// Skip identity mappings — status is already correct
			if (newStatus === artifact.status) continue;

			changes.push({
				file: artifact.file,
				artifactId: artifact.id,
				artifactType: mapping.artifactType,
				oldStatus: artifact.status,
				newStatus,
			});
		}
	}

	return changes;
}

/**
 * Apply changes to disk — rewrite frontmatter status fields.
 * @param changes - List of changes to apply.
 * @returns Number of changes successfully applied.
 */
function applyChanges(changes: MigrationChange[]): number {
	let applied = 0;

	for (const change of changes) {
		let content: string;
		try {
			content = readFileSync(change.file, "utf-8");
		} catch {
			console.error(`  SKIP: cannot read ${change.file}`);
			continue;
		}

		const parsed = parseFrontmatterFromContent(content);
		if (!parsed) {
			console.error(`  SKIP: cannot parse frontmatter in ${change.file}`);
			continue;
		}

		const [fm, body] = parsed;
		fm.status = change.newStatus;
		writeFrontmatter(change.file, fm, body);
		applied++;
	}

	return applied;
}

// ---------------------------------------------------------------------------
// Command entry point
// ---------------------------------------------------------------------------

/**
 * Dispatch the migrate command: apply workflow-driven status migrations across the graph.
 * @param args - CLI arguments after "migrate".
 */
export async function runMigrateCommand(args: string[]): Promise<void> {
	if (args.includes("--help") || args.includes("-h")) {
		console.log(USAGE);
		return;
	}

	const dryRun = args.includes("--dry-run");
	const verbose = args.includes("--verbose");

	// Parse --type flag
	let typeFilter: string | null = null;
	const typeIdx = args.indexOf("--type");
	if (typeIdx !== -1 && typeIdx + 1 < args.length) {
		typeFilter = args[typeIdx + 1];
	}

	const projectRoot = resolve(getRoot());

	// Step 1: Load migration mappings from resolved workflows
	const mappings = loadMigrationMappings(projectRoot);

	if (mappings.length === 0) {
		console.log("No migration mappings found in .orqa/workflows/*.resolved.yaml");
		console.log("(Only workflows with status renames are included.)");
		return;
	}

	console.log("Migration mappings loaded from resolved workflows:\n");
	for (const m of mappings) {
		const renames = Object.entries(m.mappings)
			.filter(([from, to]) => from !== to)
			.map(([from, to]) => `${from} -> ${to}`);
		console.log(`  ${m.workflowName} (${m.artifactType}):`);
		for (const r of renames) {
			console.log(`    ${r}`);
		}
		if (verbose) {
			const identity = Object.entries(m.mappings)
				.filter(([from, to]) => from === to)
				.map(([from]) => from);
			if (identity.length > 0) {
				console.log(`    (unchanged: ${identity.join(", ")})`);
			}
		}
	}
	console.log();

	// Step 2: Compute changes
	const changes = computeChanges(projectRoot, mappings, typeFilter);

	if (changes.length === 0) {
		console.log("No artifacts need status migration.");
		if (typeFilter) {
			console.log(`(Filtered to type: ${typeFilter})`);
		}
		return;
	}

	// Step 3: Report
	console.log(
		`Found ${changes.length} artifact(s) to migrate${typeFilter ? ` (type: ${typeFilter})` : ""}:\n`,
	);

	for (const change of changes) {
		const rel = change.file
			.replace(projectRoot + "/", "")
			.replace(projectRoot + "\\", "");
		console.log(
			`  ${change.artifactId} (${change.artifactType}): ${change.oldStatus} -> ${change.newStatus}`,
		);
		console.log(`    ${rel}`);
	}
	console.log();

	// Step 4: Apply (or preview)
	if (dryRun) {
		console.log("Dry run — no files were modified.");
		return;
	}

	const applied = applyChanges(changes);
	console.log(`Migrated ${applied}/${changes.length} artifact(s).`);

	if (applied < changes.length) {
		console.error(
			`Warning: ${changes.length - applied} artifact(s) could not be updated.`,
		);
		process.exit(1);
	}
}
