/**
 * Import command — import a directory of markdown artifact files into SurrealDB.
 *
 * orqa import --path <dir> [--on-conflict=upsert|merge] [--no-base-action=take-theirs|keep-ours|review-each|fail]
 *
 * Sends a POST /artifacts/import request to the running daemon with the
 * supplied directory path and conflict policy. The daemon walks all .md files
 * in the directory, parses each one, and imports it into SurrealDB.
 *
 * Default conflict policy: upsert (overwrite existing records, bump version).
 * Can be overridden via --on-conflict flag or project.json `import.onConflict`.
 *
 * Default no-base action (merge policy only): fail in non-interactive mode.
 */

import { readFileSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { getRoot } from "../lib/root.js";

const USAGE = `
Usage: orqa import --path <dir> [options]

Import a directory of markdown artifact files into SurrealDB.

Options:
  --path <dir>                     Directory of .md files to import (required)
  --on-conflict=upsert|merge       Conflict policy (default: upsert, or from project.json)
  --no-base-action=<action>        No-base policy for merge (default: fail)
                                     take-theirs  Accept incoming file unconditionally
                                     keep-ours    Keep current DB state
                                     review-each  Surface as CONFLICT for manual review
                                     fail         Abort if any record lacks a base
  --help, -h                       Show this help message

Examples:
  orqa import --path /path/to/artifacts
  orqa import --path ./exported --on-conflict=merge
  orqa import --path ./exported --on-conflict=merge --no-base-action=take-theirs
`.trim();

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

type ConflictPolicy = "upsert" | "merge";
type NoBaseAction = "take-theirs" | "keep-ours" | "review-each" | "fail";

interface ArtifactImportStatus {
	id: string;
	path: string;
	outcome: "CREATED" | "UPDATED" | "SKIPPED" | "MERGED" | "CONFLICT";
	reason?: string;
}

interface ImportResponse {
	migration_id: string;
	total: number;
	results: ArtifactImportStatus[];
	created: number;
	updated: number;
	skipped: number;
	merged: number;
	conflicts: number;
	base_snapshot_warning?: string;
}

// ---------------------------------------------------------------------------
// Project config reading
// ---------------------------------------------------------------------------

/**
 * Read the `import.onConflict` setting from project.json if it exists.
 * Returns null if the project.json is absent, unreadable, or the field is missing.
 * @param projectRoot - Absolute path to the project root.
 * @returns The conflict policy from config, or null.
 */
function readConfigConflictPolicy(projectRoot: string): ConflictPolicy | null {
	const configPath = join(projectRoot, ".orqa", "project.json");
	if (!existsSync(configPath)) return null;

	let content: string;
	try {
		content = readFileSync(configPath, "utf-8");
	} catch {
		return null;
	}

	let config: Record<string, unknown>;
	try {
		config = JSON.parse(content) as Record<string, unknown>;
	} catch {
		return null;
	}

	const importSection = config["import"];
	if (!importSection || typeof importSection !== "object") return null;

	const onConflict = (importSection as Record<string, unknown>)["onConflict"];
	if (onConflict === "upsert" || onConflict === "merge") return onConflict;
	return null;
}

// ---------------------------------------------------------------------------
// Daemon communication
// ---------------------------------------------------------------------------

/**
 * Resolve the daemon base URL from the environment or use the default.
 * @returns The daemon base URL (e.g. "http://127.0.0.1:10100").
 */
function daemonBaseUrl(): string {
	const portBase = process.env["ORQA_PORT_BASE"];
	const port = portBase ? parseInt(portBase, 10) : 10100;
	return `http://127.0.0.1:${port}`;
}

/**
 * POST /artifacts/import to the running daemon.
 * @param importPath - Absolute path to the directory of .md files.
 * @param onConflict - Conflict resolution policy.
 * @param noBaseAction - Action to take when no merge base is available.
 * @returns The ImportResponse from the daemon.
 */
async function postImport(
	importPath: string,
	onConflict: ConflictPolicy,
	noBaseAction: NoBaseAction,
): Promise<ImportResponse> {
	const url = `${daemonBaseUrl()}/artifacts/import`;
	const body = JSON.stringify({
		path: importPath,
		on_conflict: onConflict,
		no_base_action: noBaseAction,
	});

	let response: Response;
	try {
		response = await fetch(url, {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body,
		});
	} catch (err) {
		const msg = err instanceof Error ? err.message : String(err);
		throw new Error(
			`Could not reach daemon at ${url}: ${msg}\n` +
				"Ensure the daemon is running with: orqa daemon start",
		);
	}

	if (!response.ok) {
		let detail = "";
		try {
			const json = (await response.json()) as Record<string, unknown>;
			detail = typeof json["error"] === "string" ? `: ${json["error"]}` : "";
		} catch {
			// ignore parse errors
		}
		throw new Error(`Daemon returned ${response.status}${detail}`);
	}

	return (await response.json()) as ImportResponse;
}

// ---------------------------------------------------------------------------
// Output formatting
// ---------------------------------------------------------------------------

/**
 * Print the import response summary to stdout.
 * @param resp - The ImportResponse from the daemon.
 */
function printSummary(resp: ImportResponse): void {
	console.log(`\nImport complete (run ID: ${resp.migration_id})\n`);
	console.log(
		`  Total:    ${resp.total}  |  ` +
			`Created: ${resp.created}  |  ` +
			`Updated: ${resp.updated}  |  ` +
			`Merged: ${resp.merged}  |  ` +
			`Skipped: ${resp.skipped}  |  ` +
			`Conflicts: ${resp.conflicts}`,
	);

	if (resp.base_snapshot_warning) {
		console.log(`\n  Warning: ${resp.base_snapshot_warning}`);
	}

	if (resp.results.length > 0) {
		console.log("\nPer-artifact results:\n");
		for (const item of resp.results) {
			const label = item.outcome.padEnd(9);
			const reason = item.reason ? `  (${item.reason})` : "";
			console.log(`  ${label}  ${item.id}${reason}`);
		}
	}

	if (resp.conflicts > 0) {
		console.log(
			`\n  ${resp.conflicts} conflict(s) written to ` +
				`.state/import-conflicts/${resp.migration_id}/`,
		);
	}
}

// ---------------------------------------------------------------------------
// Command entry point
// ---------------------------------------------------------------------------

/**
 * Dispatch the import command: import a directory of artifacts into SurrealDB.
 * @param args - CLI arguments after "import".
 */
export async function runImportCommand(args: string[]): Promise<void> {
	if (args.includes("--help") || args.includes("-h")) {
		console.log(USAGE);
		return;
	}

	// Parse --path
	const pathIdx = args.indexOf("--path");
	if (pathIdx === -1 || pathIdx + 1 >= args.length) {
		console.error("Error: --path <dir> is required.\n");
		console.error(USAGE);
		process.exit(1);
	}
	const rawPath = args[pathIdx + 1];
	const importPath = resolve(rawPath);

	// Parse --on-conflict (flag overrides config)
	let onConflict: ConflictPolicy | null = null;
	const conflictIdx = args.findIndex((a) => a.startsWith("--on-conflict="));
	if (conflictIdx !== -1) {
		const val = args[conflictIdx].split("=")[1];
		if (val === "upsert" || val === "merge") {
			onConflict = val;
		} else {
			console.error(`Error: invalid --on-conflict value "${val}". Use upsert or merge.\n`);
			process.exit(1);
		}
	}

	// Fall back to project.json config, then default to upsert.
	if (onConflict === null) {
		const projectRoot = resolve(getRoot());
		onConflict = readConfigConflictPolicy(projectRoot) ?? "upsert";
	}

	// Parse --no-base-action.
	// Default is context-sensitive: interactive sessions default to review-each
	// (surface conflicts for human review); non-interactive (CI/pipe) defaults
	// to fail so automation does not silently discard data.
	let noBaseAction: NoBaseAction = process.stdout.isTTY ? "review-each" : "fail";
	const noBaseIdx = args.findIndex((a) => a.startsWith("--no-base-action="));
	if (noBaseIdx !== -1) {
		const val = args[noBaseIdx].split("=")[1];
		const valid: NoBaseAction[] = ["take-theirs", "keep-ours", "review-each", "fail"];
		if (valid.includes(val as NoBaseAction)) {
			noBaseAction = val as NoBaseAction;
		} else {
			console.error(
				`Error: invalid --no-base-action value "${val}". ` +
					`Use: take-theirs, keep-ours, review-each, fail\n`,
			);
			process.exit(1);
		}
	}

	console.log(
		`Importing from: ${importPath}\n` +
			`  on-conflict:    ${onConflict}\n` +
			`  no-base-action: ${noBaseAction}`,
	);

	let resp: ImportResponse;
	try {
		resp = await postImport(importPath, onConflict, noBaseAction);
	} catch (err) {
		console.error(`\nImport failed: ${err instanceof Error ? err.message : String(err)}`);
		process.exit(1);
	}

	printSummary(resp);

	// Exit non-zero if there were unresolvable conflicts.
	if (resp.conflicts > 0) {
		process.exit(1);
	}
}
