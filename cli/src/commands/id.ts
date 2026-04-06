/**
 * ID management command — generate, check, and migrate artifact IDs.
 *
 * orqa id generate <type>        Generate a new hex ID
 * orqa id check                  Scan graph for duplicate IDs
 * orqa id migrate <old> <new>    Rename an ID across the entire graph
 */

import { createHash, randomBytes } from "node:crypto";
import { readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { parseFrontmatterFromContent, writeFrontmatter } from "../lib/frontmatter.js";

const USAGE = `
Usage: orqa id <subcommand> [options]

Subcommands:
  generate <TYPE> <TITLE>  Generate a deterministic hex ID from MD5(title)
                           e.g. orqa id generate TASK "Fix broken tests"
  check                    Scan the graph for duplicate IDs
  migrate <old> <new>      Rename an artifact ID across the entire graph (updates all references)

Options:
  --fix                 With 'check': prompt to regenerate duplicate IDs
  -y                    With 'check --fix': auto-regenerate without prompting (for CI/tooling)
  --help, -h            Show this help message
`.trim();

/**
 * Generate a deterministic 8-char hex ID from MD5 of the title.
 * ID = PREFIX-{first 8 hex chars of MD5(title)}
 * @param prefix - Artifact type prefix (e.g. "TASK", "EPIC").
 * @param title - The artifact title used to derive the ID.
 * @returns Generated ID string.
 */
function generateIdFromTitle(prefix: string, title: string): string {
	const hex = createHash("md5").update(title).digest("hex").substring(0, 8);
	return `${prefix.toUpperCase()}-${hex}`;
}

/**
 * Generate a random 8-char hex ID (legacy, used only for dedup fix).
 * @param prefix - Artifact type prefix (e.g. "TASK").
 * @returns Random ID string.
 */
function generateRandomId(prefix: string): string {
	const hex = randomBytes(4).toString("hex");
	return `${prefix.toUpperCase()}-${hex}`;
}

/**
 * Walk all markdown files in scan directories.
 * @param dir - Directory to walk recursively.
 * @param results - Accumulator for collected file paths.
 * @returns Array of absolute paths to .md files.
 */
function walkFiles(dir: string, results: string[] = []): string[] {
	let entries;
	try {
		entries = readdirSync(dir, { withFileTypes: true });
	} catch {
		return results;
	}
	for (const entry of entries) {
		if (
			entry.name.startsWith(".") ||
			entry.name === "node_modules" ||
			entry.name === "dist" ||
			entry.name === "target"
		)
			continue;
		const full = join(dir, entry.name);
		if (entry.isDirectory()) walkFiles(full, results);
		else if (entry.name.endsWith(".md")) results.push(full);
	}
	return results;
}

/**
 * Scan for duplicate IDs in the graph.
 * @param projectRoot - Absolute path to the project root.
 * @param autoFix - Whether to auto-regenerate duplicate IDs.
 */
function checkDuplicates(projectRoot: string, autoFix: boolean): void {
	// Build ID → paths map by scanning files directly
	const idToFiles = new Map<string, string[]>();
	const scanDirs = [
		join(projectRoot, ".orqa"),
		join(projectRoot, "app", ".orqa"),
		join(projectRoot, "plugins"),
		join(projectRoot, "connectors"),
		join(projectRoot, "sidecars"),
	];

	const allFiles = scanDirs.flatMap((d) => walkFiles(d));

	for (const file of allFiles) {
		const content = readFileSync(file, "utf-8");
		const parsed = parseFrontmatterFromContent(content);
		if (!parsed) continue;
		const [fm] = parsed;
		const id = typeof fm.id === "string" ? fm.id.trim() : "";
		if (!id) continue;

		const list = idToFiles.get(id) ?? [];
		list.push(file);
		idToFiles.set(id, list);
	}

	// Find duplicates
	const duplicates = [...idToFiles.entries()].filter(([, files]) => files.length > 1);

	if (duplicates.length === 0) {
		console.log("No duplicate IDs found.");
		return;
	}

	console.log(`Found ${duplicates.length} duplicate ID(s):\n`);

	let fixed = 0;

	for (const [id, files] of duplicates) {
		console.log(`  ${id} (${files.length} files):`);
		for (const file of files) {
			const rel = file.replace(projectRoot + "/", "").replace(projectRoot + "\\", "");
			console.log(`    - ${rel}`);
		}

		if (autoFix) {
			// Keep the first file's ID, regenerate for the rest
			const prefix = id.split("-")[0];
			for (let i = 1; i < files.length; i++) {
				const file = files[i];
				const content = readFileSync(file, "utf-8");
				const parsed = parseFrontmatterFromContent(content);
				if (!parsed) continue;

				const [fm, body] = parsed;
				const newId = generateRandomId(prefix);
				const oldId = fm.id as string;
				fm.id = newId;

				writeFrontmatter(file, fm, body);

				// Update all references to the old ID in all files
				const refsUpdated = updateReferences(allFiles, oldId, newId);

				const rel = file.replace(projectRoot + "/", "").replace(projectRoot + "\\", "");
				console.log(`    FIXED: ${rel} → ${newId} (${refsUpdated} references updated)`);
				fixed++;
			}
		}

		console.log();
	}

	if (autoFix && fixed > 0) {
		console.log(`Regenerated ${fixed} duplicate ID(s).`);
	} else if (!autoFix && duplicates.length > 0) {
		console.log("Run with --fix to auto-regenerate duplicate IDs.");
		process.exit(1);
	}
}

/**
 * Migrate a single ID across the entire graph.
 * Updates the artifact's own frontmatter and all relationship references.
 * @param projectRoot - Absolute path to the project root.
 * @param oldId - The existing artifact ID to rename.
 * @param newId - The new artifact ID.
 */
function migrateId(projectRoot: string, oldId: string, newId: string): void {
	const scanDirs = [
		join(projectRoot, ".orqa"),
		join(projectRoot, "app", ".orqa"),
		join(projectRoot, "plugins"),
		join(projectRoot, "connectors"),
		join(projectRoot, "sidecars"),
	];

	const allFiles = scanDirs.flatMap((d) => walkFiles(d));

	// Find the source artifact
	let sourceFile: string | null = null;
	for (const file of allFiles) {
		const content = readFileSync(file, "utf-8");
		const parsed = parseFrontmatterFromContent(content);
		if (!parsed) continue;
		const [fm] = parsed;
		if (fm.id === oldId) {
			sourceFile = file;
			break;
		}
	}

	if (!sourceFile) {
		console.error(`Artifact with ID "${oldId}" not found.`);
		process.exit(1);
	}

	// Update the source artifact's ID
	const content = readFileSync(sourceFile, "utf-8");
	const parsed = parseFrontmatterFromContent(content);
	if (!parsed) {
		console.error(`Failed to parse frontmatter in ${sourceFile}`);
		process.exit(1);
	}

	const [fm, body] = parsed;
	fm.id = newId;
	writeFrontmatter(sourceFile, fm, body);

	const rel = sourceFile.replace(projectRoot + "/", "").replace(projectRoot + "\\", "");
	console.log(`Updated: ${rel} (${oldId} → ${newId})`);

	// Update all references
	const refsUpdated = updateReferences(allFiles, oldId, newId);
	console.log(`Updated ${refsUpdated} reference(s) across the graph.`);
}

/**
 * Update all relationship target references from oldId to newId.
 * Uses YAML parser for frontmatter, preserves body content.
 * @param allFiles - All .md file paths to scan.
 * @param oldId - The old ID to replace.
 * @param newId - The replacement ID.
 * @returns Number of references updated.
 */
function updateReferences(allFiles: string[], oldId: string, newId: string): number {
	let count = 0;

	for (const file of allFiles) {
		const content = readFileSync(file, "utf-8");

		// Quick check: skip files that don't contain the old ID at all
		if (!content.includes(oldId)) continue;

		const parsed = parseFrontmatterFromContent(content);
		if (!parsed) continue;

		const [fm, body] = parsed;
		let modified = false;

		// Update relationship targets
		if (Array.isArray(fm.relationships)) {
			for (const rel of fm.relationships as Array<Record<string, unknown>>) {
				if (rel.target === oldId) {
					rel.target = newId;
					modified = true;
					count++;
				}
			}
		}

		// Update body text references (outside frontmatter)
		const updatedBody = body.replaceAll(oldId, newId);
		if (updatedBody !== body) {
			modified = true;
			count++;
		}

		if (modified) {
			writeFrontmatter(file, fm, updatedBody);
		}
	}

	return count;
}

/**
 * Dispatch the id command: generate, check, or migrate artifact IDs.
 * @param args - CLI arguments after "id".
 */
export async function runIdCommand(args: string[]): Promise<void> {
	if (args.length === 0 || args.includes("--help") || args.includes("-h")) {
		console.log(USAGE);
		return;
	}

	const subcommand = args[0];
	const subArgs = args.slice(1);

	switch (subcommand) {
		case "generate": {
			const positional = subArgs.filter((a) => !a.startsWith("--"));
			if (positional.length < 2) {
				console.error("Usage: orqa id generate <TYPE> <TITLE>");
				console.error('Example: orqa id generate TASK "Fix broken tests"');
				process.exit(1);
			}
			const [typePrefix, ...titleParts] = positional;
			const title = titleParts.join(" ");
			console.log(generateIdFromTitle(typePrefix, title));
			break;
		}

		case "check": {
			const autoFix = subArgs.includes("--fix") || subArgs.includes("-y");
			const targetPath = subArgs.find((a) => !a.startsWith("--") && a !== "-y") ?? process.cwd();
			checkDuplicates(resolve(targetPath), autoFix);
			break;
		}

		case "migrate": {
			const ids = subArgs.filter((a) => !a.startsWith("--"));
			if (ids.length < 2) {
				console.error("Usage: orqa id migrate <old-id> <new-id>");
				process.exit(1);
			}
			const [oldId, newId] = ids;
			const targetPath = resolve(
				subArgs.find((a) => a.startsWith("--path="))?.replace("--path=", "") ?? process.cwd(),
			);
			migrateId(targetPath, oldId, newId);
			break;
		}

		default:
			console.error(`Unknown subcommand: ${subcommand}`);
			console.log(USAGE);
			process.exit(1);
	}
}
