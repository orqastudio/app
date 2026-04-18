/**
 * Plugin list command — print the plugin installation ledger from SurrealDB.
 *
 * orqa plugin list
 *
 * Queries GET /plugins/installed on the daemon and prints a table showing:
 * plugin name, manifest version, install date, file count (all targets), drift status.
 *
 * Replaces: cat .orqa/manifest.json
 */

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface PluginInstallationRecord {
	plugin_name: string;
	manifest_version: string;
	manifest_hash: string;
	installed_at: string;
	updated_at: string;
	version: number;
	files: Array<{
		path: string;
		source_hash: string;
		installed_hash: string;
		target: string;
		artifact_id?: string;
	}>;
}

// ---------------------------------------------------------------------------
// Daemon communication
// ---------------------------------------------------------------------------

/**
 * Resolve the daemon base URL from the environment or use the default.
 * @returns The daemon's HTTP base URL.
 */
function daemonBaseUrl(): string {
	const portBase = process.env["ORQA_PORT_BASE"];
	const port = portBase ? parseInt(portBase, 10) : 10100;
	return `http://127.0.0.1:${port}`;
}

/**
 * Fetch all plugin_installation records from the daemon.
 * @returns The list of plugin installation records.
 */
async function fetchInstalledPlugins(): Promise<PluginInstallationRecord[]> {
	const url = `${daemonBaseUrl()}/plugins/installed`;
	let response: Response;
	try {
		response = await fetch(url);
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
		throw new Error(`Daemon returned HTTP ${response.status}${detail}`);
	}

	return (await response.json()) as PluginInstallationRecord[];
}

// ---------------------------------------------------------------------------
// Drift detection
// ---------------------------------------------------------------------------

/**
 * Compute drift status for a plugin record.
 * Drift = any file has mismatched source_hash vs installed_hash.
 * @param record - The plugin installation record to check.
 * @returns Either "ok" or a short drift description.
 */
function driftStatus(record: PluginInstallationRecord): string {
	const drifted = record.files.filter((f) => f.source_hash !== f.installed_hash);
	if (drifted.length === 0) {
		return "ok";
	}
	return `drifted (${drifted.length} file${drifted.length === 1 ? "" : "s"})`;
}

// ---------------------------------------------------------------------------
// Output formatting
// ---------------------------------------------------------------------------

/**
 * Format an ISO datetime string to a short date.
 * @param iso - ISO 8601 datetime string.
 * @returns The YYYY-MM-DD date portion, or the original input on parse failure.
 */
function formatDate(iso: string): string {
	try {
		const d = new Date(iso);
		return d.toISOString().split("T")[0] ?? iso;
	} catch {
		return iso;
	}
}

/**
 * Print the plugin installation table.
 * @param records - The plugin installation records to render.
 */
function printTable(records: PluginInstallationRecord[]): void {
	if (records.length === 0) {
		console.log("No plugin installation records found.");
		console.log("Run 'orqa install' or 'orqa plugin install <path>' to install plugins.");
		return;
	}

	const headers = ["Plugin", "Version", "Installed", "Files", "Drift"];
	const rows = records.map((r) => [
		r.plugin_name,
		r.manifest_version,
		formatDate(r.installed_at),
		String(r.files.length),
		driftStatus(r),
	]);

	// Compute column widths.
	const widths = headers.map((h, i) =>
		Math.max(h.length, ...rows.map((row) => (row[i] ?? "").length)),
	);

	const separator = widths.map((w) => "-".repeat(w)).join("  ");
	const fmt = (row: string[]) => row.map((cell, i) => cell.padEnd(widths[i] ?? 0)).join("  ");

	console.log(fmt(headers));
	console.log(separator);
	for (const row of rows) {
		console.log(fmt(row));
	}
	console.log(`\n${records.length} plugin${records.length === 1 ? "" : "s"} installed.`);
}

// ---------------------------------------------------------------------------
// Command entry point
// ---------------------------------------------------------------------------

const USAGE = `
Usage: orqa plugin list

Print the plugin installation ledger from SurrealDB.

Shows plugin name, manifest version, install date, file count, and drift status
for every installed plugin. Requires the daemon to be running.

Options:
  --help, -h   Show this help message
`.trim();

/**
 * Run the plugin list command.
 * @param args - CLI arguments after "plugin list".
 */
export async function runPluginListCommand(args: string[]): Promise<void> {
	if (args.includes("--help") || args.includes("-h")) {
		console.log(USAGE);
		return;
	}

	let records: PluginInstallationRecord[];
	try {
		records = await fetchInstalledPlugins();
	} catch (err) {
		console.error(`Error: ${err instanceof Error ? err.message : String(err)}`);
		process.exit(1);
	}

	printTable(records);
}
