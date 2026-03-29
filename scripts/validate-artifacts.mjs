#!/usr/bin/env node
/**
 * Temporary artifact validation script for the migration period.
 *
 * Validates governance artifacts against targets/schema.composed.json.
 * This script is a stopgap — it gets replaced by the engine's enforcement
 * crate once the Rust engine extraction is complete.
 *
 * Usage:
 *   node scripts/validate-artifacts.mjs                    # Validate all .orqa/ artifacts
 *   node scripts/validate-artifacts.mjs --staged           # Validate only staged .md files in .orqa/
 *   node scripts/validate-artifacts.mjs --file <path>      # Validate a single file
 *   node scripts/validate-artifacts.mjs --hook --file <p>  # Called from pre-commit hook (respects ORQA_SKIP_SCHEMA_VALIDATION)
 *   node scripts/validate-artifacts.mjs --summary          # Print summary counts only
 *
 * Exit codes:
 *   0 — all valid (or skipped via env var)
 *   1 — validation errors found
 *   2 — script error (missing schema, bad args)
 */

import { readFileSync, readdirSync, statSync, existsSync } from "fs";
import { join, relative, extname, basename, dirname } from "path";
import { execSync } from "child_process";

// ── Config ──────────────────────────────────────────────────────────────────

const PROJECT_ROOT = findProjectRoot();
const SCHEMA_PATH = join(PROJECT_ROOT, "targets", "schema.composed.json");
const ORQA_DIR = join(PROJECT_ROOT, ".orqa");

// ── Skip check for hook mode ────────────────────────────────────────────────

const args = process.argv.slice(2);
const isHookMode = args.includes("--hook");
const isStagedMode = args.includes("--staged");
const isSummaryMode = args.includes("--summary");
const fileArg = args.includes("--file") ? args[args.indexOf("--file") + 1] : null;

if (isHookMode && process.env.ORQA_SKIP_SCHEMA_VALIDATION === "true") {
	process.exit(0);
}

// ── Load schema ─────────────────────────────────────────────────────────────

if (!existsSync(SCHEMA_PATH)) {
	console.error(`ERROR: Target schema not found at ${SCHEMA_PATH}`);
	console.error("Run the schema composition step first (Phase 1, Step 1).");
	process.exit(2);
}

let schema;
try {
	schema = JSON.parse(readFileSync(SCHEMA_PATH, "utf-8"));
} catch (e) {
	console.error(`ERROR: Failed to parse schema: ${e.message}`);
	process.exit(2);
}

const artifactTypes = schema.artifactTypes || {};
const relationships = schema.relationships || {};

// ── Collect files to validate ───────────────────────────────────────────────

let filesToValidate = [];

if (fileArg) {
	if (fileArg === "skip") {
		process.exit(0);
	}
	if (existsSync(fileArg) && extname(fileArg) === ".md") {
		filesToValidate = [fileArg];
	} else {
		process.exit(0); // Not a markdown file, nothing to validate
	}
} else if (isStagedMode) {
	try {
		const staged = execSync("git diff --cached --name-only --diff-filter=ACMR", {
			encoding: "utf-8",
		})
			.trim()
			.split("\n")
			.filter((f) => f.endsWith(".md") && f.startsWith(".orqa/"));
		filesToValidate = staged.map((f) => join(PROJECT_ROOT, f));
	} catch {
		filesToValidate = [];
	}
} else {
	filesToValidate = collectMarkdownFiles(ORQA_DIR);
}

if (filesToValidate.length === 0) {
	if (!isSummaryMode) console.log("No artifacts to validate.");
	process.exit(0);
}

// ── Parse frontmatter ───────────────────────────────────────────────────────

/**
 * Parses YAML frontmatter from markdown content.
 * Handles flat key-value pairs, booleans, nulls, and the relationships array
 * (list of objects with target/type/rationale keys). Returns null if no
 * frontmatter block is present.
 */
function parseFrontmatter(content) {
	const match = content.match(/^---\r?\n([\s\S]*?)\r?\n---/);
	if (!match) return null;

	const yaml = match[1];
	const fields = {};
	const lines = yaml.split("\n");

	let i = 0;
	while (i < lines.length) {
		const line = lines[i];
		const kvMatch = line.match(/^(\w[\w-]*)\s*:\s*(.*)$/);
		if (kvMatch) {
			const key = kvMatch[1];
			let value = kvMatch[2].trim();

			// Detect a block sequence (value is empty, next lines are list items)
			if (value === "") {
				const items = [];
				i++;
				while (i < lines.length && lines[i].match(/^\s+-\s/)) {
					// Parse the first property of the list item
					const firstPropMatch = lines[i].match(/^\s+-\s+(\w[\w-]*)\s*:\s*(.*)$/);
					if (firstPropMatch) {
						const obj = {};
						const propKey = firstPropMatch[1];
						let propValue = firstPropMatch[2].trim();
						if (
							(propValue.startsWith('"') && propValue.endsWith('"')) ||
							(propValue.startsWith("'") && propValue.endsWith("'"))
						) {
							propValue = propValue.slice(1, -1);
						}
						obj[propKey] = propValue;

						// Collect subsequent indented properties belonging to this object
						i++;
						while (i < lines.length && lines[i].match(/^\s{4,}(\w[\w-]*)\s*:/)) {
							const nestedMatch = lines[i].match(/^\s+(\w[\w-]*)\s*:\s*(.*)$/);
							if (nestedMatch) {
								const nk = nestedMatch[1];
								let nv = nestedMatch[2].trim();
								if (
									(nv.startsWith('"') && nv.endsWith('"')) ||
									(nv.startsWith("'") && nv.endsWith("'"))
								) {
									nv = nv.slice(1, -1);
								}
								obj[nk] = nv;
							}
							i++;
						}
						items.push(obj);
					} else {
						// Plain list item (scalar)
						const scalarMatch = lines[i].match(/^\s+-\s+(.*)/);
						if (scalarMatch) items.push(scalarMatch[1].trim());
						i++;
					}
				}
				fields[key] = items;
				continue; // i already advanced inside the loop above
			}

			// Remove quotes from scalar values
			if (
				(value.startsWith('"') && value.endsWith('"')) ||
				(value.startsWith("'") && value.endsWith("'"))
			) {
				value = value.slice(1, -1);
			}

			// Handle booleans and nulls
			if (value === "true") value = true;
			else if (value === "false") value = false;
			else if (value === "null" || value === "") value = null;

			fields[key] = value;
		}
		i++;
	}

	return fields;
}

// ── Validation ──────────────────────────────────────────────────────────────

const errors = [];
const warnings = [];
const counts = { total: 0, valid: 0, invalid: 0, skipped: 0 };

// Two-pass relationship validation: collect all known IDs and all relationship
// targets, then cross-reference after the main loop.
const knownIds = new Set();
const allRelationshipTargets = [];

// Track which files already have errors so the broken-link pass doesn't double-count.
const invalidFiles = new Set();

for (const filePath of filesToValidate) {
	counts.total++;
	const relPath = relative(PROJECT_ROOT, filePath).replace(/\\/g, "/");

	let content;
	try {
		content = readFileSync(filePath, "utf-8");
	} catch {
		errors.push({ file: relPath, error: "Could not read file" });
		counts.invalid++;
		invalidFiles.add(relPath);
		continue;
	}

	const fm = parseFrontmatter(content);
	if (!fm) {
		// Skip files without frontmatter (e.g., non-artifact markdown)
		counts.skipped++;
		continue;
	}

	// Collect this artifact's ID for the cross-reference pass.
	if (fm.id) knownIds.add(fm.id);

	// Collect relationship targets for the cross-reference pass.
	if (Array.isArray(fm.relationships)) {
		for (const rel of fm.relationships) {
			if (rel && typeof rel === "object" && rel.target) {
				allRelationshipTargets.push({ file: relPath, target: rel.target });
			}
		}
	}

	const fileErrors = validateArtifact(fm, relPath, content);
	if (fileErrors.length > 0) {
		errors.push(...fileErrors.map((e) => ({ file: relPath, ...e })));
		counts.invalid++;
		invalidFiles.add(relPath);
	} else {
		counts.valid++;
	}
}

// ── Relationship target existence check (cross-reference pass) ───────────────
//
// Now that all artifact IDs are known, verify that every relationship target
// refers to an ID that actually exists. Broken links indicate either a typo or
// a deleted artifact whose referencing file was not updated.

for (const { file, target } of allRelationshipTargets) {
	if (!knownIds.has(target)) {
		errors.push({
			file,
			error: `Broken relationship: target "${target}" does not exist as a known artifact`,
		});
		// Only count the file as invalid once — skip if already counted above.
		if (!invalidFiles.has(file)) {
			counts.invalid++;
			// counts.valid was already incremented for this file; undo that.
			counts.valid--;
			invalidFiles.add(file);
		}
	}
}

function validateArtifact(fm, relPath, content) {
	const errs = [];

	// 1. Must have id
	if (!fm.id) {
		errs.push({ error: "Missing required field: id" });
		return errs; // Can't validate further without id
	}

	// 2. Must have type
	if (!fm.type) {
		errs.push({ error: "Missing required field: type" });
	}

	// 3. ID format: PREFIX-[0-9a-f]{8}
	const idMatch = fm.id.match(/^([A-Z]+)-([0-9a-f]{8})$/);
	if (!idMatch) {
		errs.push({
			error: `Invalid ID format: "${fm.id}" — expected PREFIX-[0-9a-f]{8}`,
		});
	}

	// 4. ID must match filename
	const expectedFilename = `${fm.id}.md`;
	const actualFilename = basename(relPath);
	if (actualFilename !== expectedFilename) {
		errs.push({
			error: `ID/filename mismatch: id="${fm.id}" but file is "${actualFilename}"`,
		});
	}

	// 5. Type must have a schema definition
	if (fm.type && artifactTypes[fm.type]) {
		const typeDef = artifactTypes[fm.type];

		// 5a. ID must match type's id_pattern when available, or fall back to
		// strict id_prefix comparison. id_pattern allows both legacy and current
		// prefixes (e.g., IDEA-* and DISC-IDEA-* for discovery-idea).
		if (idMatch) {
			const idPatternStr = typeDef.id_pattern;
			if (idPatternStr) {
				const idPattern = new RegExp(idPatternStr);
				if (!idPattern.test(fm.id)) {
					errs.push({
						error: `ID format mismatch: type "${fm.type}" expects pattern "${idPatternStr}" but got "${fm.id}"`,
					});
				}
			} else if (typeDef.id_prefix && idMatch[1] !== typeDef.id_prefix) {
				errs.push({
					error: `ID prefix mismatch: type "${fm.type}" expects prefix "${typeDef.id_prefix}" but got "${idMatch[1]}"`,
				});
			}
		}

		// 5b. Type-location consistency: the artifact must live under its type's
		// expected directory. relPath is normalised to forward slashes above.
		const defaultPath = typeDef.default_path;
		if (defaultPath) {
			const artifactDir = dirname(relPath) + "/";
			const expectedDir = defaultPath.endsWith("/") ? defaultPath : defaultPath + "/";
			// Strip a leading "./" from expectedDir so both sides are comparable.
			const normalizedExpected = expectedDir.replace(/^\.\//, "");
			const isKnowledge = fm.type === "knowledge";
			const inAKnowledgeSubdir = /[/]knowledge[/]$/.test(artifactDir);
			if (!artifactDir.startsWith(normalizedExpected) && !(isKnowledge && inAKnowledgeSubdir)) {
				errs.push({
					error: `Type-location mismatch: "${fm.type}" expects path "${expectedDir}" but artifact is at "${relPath}"`,
				});
			}
		}

		// 5c. Required fields
		if (typeDef.fields && typeDef.fields.required) {
			for (const [field, fieldDef] of Object.entries(typeDef.fields.required)) {
				if (field === "id") continue; // Already checked
				if (fm[field] === undefined || fm[field] === null) {
					errs.push({ error: `Missing required field: ${field}` });
				}
			}
		}

		// 5d. Status must be valid
		if (fm.status && typeDef.statuses && typeDef.statuses.length > 0) {
			if (!typeDef.statuses.includes(fm.status)) {
				errs.push({
					error: `Invalid status "${fm.status}" for type "${fm.type}" — valid: [${typeDef.statuses.join(", ")}]`,
				});
			}
		}
	} else if (fm.type) {
		warnings.push({
			file: relPath,
			warning: `Unknown artifact type: "${fm.type}" — no schema definition found`,
		});
	}

	// 6. Should use 'title' not 'name' for display name
	if (fm.name && !fm.title) {
		errs.push({
			error: `Uses "name" field instead of "title" — standardize to "title"`,
		});
	}

	// 7. Knowledge size constraint (approximate)
	if (fm.type === "knowledge") {
		// Rough token estimate: ~4 chars per token
		const approxTokens = Math.ceil(content.length / 4);
		if (approxTokens < 500) {
			// Token count is a guideline, not a structural requirement — warn but don't block.
			warnings.push({
				file: relPath,
				warning: `Knowledge artifact is ~${approxTokens} tokens — below target (minimum: 500 tokens)`,
			});
		}
		if (approxTokens > 2000) {
			warnings.push({
				file: relPath,
				warning: `Knowledge artifact is ~${approxTokens} tokens (target: 500-2000)`,
			});
		}
	}

	return errs;
}

// ── Output ──────────────────────────────────────────────────────────────────

if (isSummaryMode) {
	console.log(
		`Artifacts: ${counts.total} total, ${counts.valid} valid, ${counts.invalid} invalid, ${counts.skipped} skipped`,
	);
	if (errors.length > 0) {
		console.log(`Errors: ${errors.length}`);
	}
	if (warnings.length > 0) {
		console.log(`Warnings: ${warnings.length}`);
	}
} else {
	if (errors.length > 0) {
		console.error(`\n=== Artifact Validation: ${errors.length} error(s) ===\n`);
		for (const err of errors) {
			console.error(`  ${err.file}: ${err.error}`);
		}
	}

	if (warnings.length > 0) {
		console.warn(`\n=== Warnings: ${warnings.length} ===\n`);
		for (const w of warnings) {
			console.warn(`  ${w.file}: ${w.warning}`);
		}
	}

	if (errors.length === 0) {
		console.log(
			`\nArtifact validation passed: ${counts.valid} valid, ${counts.skipped} skipped.`,
		);
	}
}

process.exit(errors.length > 0 ? 1 : 0);

// ── Helpers ─────────────────────────────────────────────────────────────────

function findProjectRoot() {
	let dir = process.cwd();
	while (dir !== "/" && dir !== "C:\\") {
		if (existsSync(join(dir, ".orqa"))) return dir;
		const parent = join(dir, "..");
		if (parent === dir) break;
		dir = parent;
	}
	return process.cwd();
}

function collectMarkdownFiles(dir) {
	const files = [];
	if (!existsSync(dir)) return files;

	for (const entry of readdirSync(dir, { withFileTypes: true })) {
		const fullPath = join(dir, entry.name);
		if (entry.isDirectory()) {
			// Skip workflows (resolved YAML, not artifacts) and search index
			if (entry.name === "workflows" || entry.name === "node_modules") continue;
			files.push(...collectMarkdownFiles(fullPath));
		} else if (entry.isFile() && extname(entry.name) === ".md") {
			files.push(fullPath);
		}
	}
	return files;
}
