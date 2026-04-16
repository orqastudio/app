/**
 * Storage migration command — migrate .orqa/ markdown artifacts into SurrealDB.
 *
 * orqa migrate storage ingest [--project-root <path>]
 *
 * Calls the daemon to classify every .md file under .orqa/ as user-authored
 * (inserted) or plugin-derived (skipped). Pauses the file watcher before
 * ingestion and resumes it after, on both success and error paths.
 *
 * Idempotent: re-running against a populated SurrealDB writes zero new records
 * because the daemon uses content-hash dedup.
 */

import { resolve } from "node:path";
import { getRoot } from "../lib/root.js";

const USAGE = `
Usage: orqa migrate storage ingest [options]

Migrate user-authored .orqa/ markdown artifacts into SurrealDB.

Pauses the file watcher before ingest and resumes it after (on both success
and error paths). Safe to re-run — idempotent via content-hash dedup.

Options:
  --project-root <path>   Override the project root to scan (default: CWD root)
  --help, -h              Show this help message

Examples:
  orqa migrate storage ingest
  orqa migrate storage ingest --project-root /path/to/project
`.trim();

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface MigrationCounts {
	scanned: number;
	inserted: number;
	skipped: number;
	flagged: number;
	errors: number;
}

interface IngestResponse {
	migration_id: string;
	counts: MigrationCounts;
	report_path: string;
	flagged_files: string[];
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
 * POST /watcher/pause — stop the daemon file watcher from emitting sync events.
 * Idempotent: calling when already paused is safe.
 * @throws if the daemon is unreachable or returns a non-OK status.
 */
async function pauseWatcher(): Promise<void> {
	const url = `${daemonBaseUrl()}/watcher/pause`;
	let response: Response;
	try {
		response = await fetch(url, { method: "POST" });
	} catch (err) {
		const msg = err instanceof Error ? err.message : String(err);
		throw new Error(
			`Could not reach daemon at ${url}: ${msg}\n` +
				"Ensure the daemon is running with: orqa daemon start",
		);
	}
	if (!response.ok) {
		throw new Error(`Failed to pause watcher: HTTP ${response.status}`);
	}
}

/**
 * POST /watcher/resume — resume the daemon file watcher.
 * Idempotent: calling when already running is safe.
 * Best-effort: logs a warning on failure rather than throwing, because this
 * is called from a finally block where the primary error must not be shadowed.
 */
async function resumeWatcher(): Promise<void> {
	const url = `${daemonBaseUrl()}/watcher/resume`;
	try {
		const response = await fetch(url, { method: "POST" });
		if (!response.ok) {
			console.warn(`Warning: watcher resume returned HTTP ${response.status} — manual resume may be needed`);
		}
	} catch (err) {
		const msg = err instanceof Error ? err.message : String(err);
		console.warn(`Warning: could not resume watcher: ${msg}`);
		console.warn("Run: curl -X POST http://127.0.0.1:10100/watcher/resume");
	}
}

/**
 * POST /admin/migrate/storage/ingest — trigger the storage ingest on the daemon.
 * Classifies and inserts user-authored artifacts into SurrealDB.
 * @param projectRoot - Optional override for the project root to scan.
 * @returns The IngestResponse from the daemon.
 */
async function postStorageIngest(projectRoot: string | null): Promise<IngestResponse> {
	const url = `${daemonBaseUrl()}/admin/migrate/storage/ingest`;
	const body = projectRoot ? JSON.stringify({ project_root: projectRoot }) : "{}";

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

	return (await response.json()) as IngestResponse;
}

// ---------------------------------------------------------------------------
// Output formatting
// ---------------------------------------------------------------------------

/**
 * Print the ingest response summary to stdout.
 * @param resp - The IngestResponse from the daemon.
 */
function printSummary(resp: IngestResponse): void {
	const { counts } = resp;
	console.log(`\nStorage ingest complete (migration ID: ${resp.migration_id})\n`);
	console.log(
		`  Scanned:  ${counts.scanned}  |  ` +
			`Inserted: ${counts.inserted}  |  ` +
			`Skipped: ${counts.skipped}  |  ` +
			`Flagged: ${counts.flagged}  |  ` +
			`Errors: ${counts.errors}`,
	);
	console.log(`\nReport written to: ${resp.report_path}`);

	if (resp.flagged_files.length > 0) {
		console.log(`\nFlagged files (${resp.flagged_files.length}) — manual resolution required:`);
		for (const f of resp.flagged_files) {
			console.log(`  ${f}`);
		}
	}
}

// ---------------------------------------------------------------------------
// Command entry point
// ---------------------------------------------------------------------------

/**
 * Dispatch the storage ingest subcommand.
 *
 * Pauses the file watcher before calling the daemon ingest route, then resumes
 * it in a finally block so the watcher is always restored — even on error.
 * @param args - CLI arguments after "migrate storage ingest".
 */
export async function runMigrateStorageIngestCommand(args: string[]): Promise<void> {
	if (args.includes("--help") || args.includes("-h")) {
		console.log(USAGE);
		return;
	}

	// Parse optional --project-root override.
	let projectRoot: string | null = null;
	const rootIdx = args.indexOf("--project-root");
	if (rootIdx !== -1 && rootIdx + 1 < args.length) {
		projectRoot = resolve(args[rootIdx + 1]);
	} else {
		projectRoot = resolve(getRoot());
	}

	console.log("Pausing file watcher...");
	await pauseWatcher();
	console.log("  Watcher paused.");

	try {
		console.log(`\nStarting storage ingest for: ${projectRoot}`);
		const resp = await postStorageIngest(projectRoot);
		printSummary(resp);

		if (resp.counts.errors > 0) {
			console.error(`\nWarning: ${resp.counts.errors} artifact(s) failed to ingest.`);
			process.exitCode = 1;
		}
	} finally {
		console.log("\nResuming file watcher...");
		await resumeWatcher();
		console.log("  Watcher resumed.");
	}
}
