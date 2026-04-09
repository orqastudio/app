/**
 * Port consistency check: `orqa check ports`.
 *
 * Validates that static config files (tauri.conf.json, docker-compose.yml)
 * match the canonical port values defined in infrastructure/ports.json.
 * These files cannot import at runtime so they must hardcode values — this
 * command catches drift between them and ports.json.
 *
 * Checks:
 *   - app/src-tauri/tauri.conf.json devUrl port matches vite service port
 *   - devtools/src-tauri/tauri.conf.json devUrl port matches devtools service port
 *   - infrastructure/orqastudio-git/docker-compose.yml port mappings match
 *     forgejo_http and forgejo_ssh service ports
 */

import * as fs from "node:fs";
import * as path from "node:path";
import { getRoot } from "../lib/root.js";

/** One port mismatch found during validation. */
interface Mismatch {
	file: string;
	field: string;
	expected: number;
	actual: number;
}

/**
 * Extract the port number from a localhost URL string.
 * Returns null if the URL is not parseable or has no explicit port.
 * @param url - URL string, e.g. "http://localhost:10420".
 * @returns Port number or null.
 */
function extractPort(url: string): number | null {
	try {
		const parsed = new URL(url);
		const port = parseInt(parsed.port, 10);
		return Number.isNaN(port) ? null : port;
	} catch {
		return null;
	}
}

/**
 * Check tauri.conf.json devUrl port against the expected service port.
 * @param root - Repository root path.
 * @param tauriConfRelPath - Relative path to tauri.conf.json from root.
 * @param serviceName - Service name for the expected port label.
 * @param expectedPort - Expected port number from ports.json.
 * @returns Array of mismatches found (empty if all match).
 */
function checkTauriConf(
	root: string,
	tauriConfRelPath: string,
	serviceName: string,
	expectedPort: number,
): Mismatch[] {
	const fullPath = path.join(root, tauriConfRelPath);
	if (!fs.existsSync(fullPath)) return [];

	let conf: { build?: { devUrl?: string } };
	try {
		conf = JSON.parse(fs.readFileSync(fullPath, "utf-8")) as typeof conf;
	} catch {
		return [];
	}

	const devUrl = conf.build?.devUrl;
	if (!devUrl) return [];

	const actual = extractPort(devUrl);
	if (actual === null) return [];

	if (actual !== expectedPort) {
		return [
			{
				file: tauriConfRelPath,
				field: "build.devUrl",
				expected: expectedPort,
				actual,
			},
		];
	}
	return [];
}

/**
 * Check docker-compose.yml port mappings against the expected service ports.
 * Looks for "HOST:CONTAINER" port mappings matching expected host ports.
 * @param root - Repository root path.
 * @param composeRelPath - Relative path to docker-compose.yml from root.
 * @param expectedPorts - Map of expected host ports to service label.
 * @returns Array of mismatches found (empty if all match).
 */
function checkDockerCompose(
	root: string,
	composeRelPath: string,
	expectedPorts: Array<{ port: number; label: string }>,
): Mismatch[] {
	const fullPath = path.join(root, composeRelPath);
	if (!fs.existsSync(fullPath)) return [];

	const content = fs.readFileSync(fullPath, "utf-8");
	const mismatches: Mismatch[] = [];

	for (const { port, label } of expectedPorts) {
		// Match "PORT:ANYTHING" patterns in the ports section.
		// docker-compose format: "- \"HOST:CONTAINER\"" or "- HOST:CONTAINER".
		const pattern = new RegExp(`["']?(\\d+):\\d+["']?`);
		const lines = content.split("\n");
		let found = false;

		for (const line of lines) {
			const match = line.match(pattern);
			if (match) {
				const hostPort = parseInt(match[1], 10);
				if (hostPort === port) {
					found = true;
					break;
				}
			}
		}

		if (!found) {
			mismatches.push({
				file: composeRelPath,
				field: `ports[${label}]`,
				expected: port,
				actual: 0,
			});
		}
	}

	return mismatches;
}

/**
 * Run the ports consistency check.
 *
 * Reads infrastructure/ports.json and validates all static config files against it.
 * Reports mismatches with clear field-level detail. Exits 0 on success, 1 on failure.
 * @returns Exit code: 0 = all match, 1 = mismatches found.
 */
export async function runCheckPortsCommand(): Promise<number> {
	const root = getRoot();
	const portsJsonPath = path.join(root, "infrastructure/ports.json");

	if (!fs.existsSync(portsJsonPath)) {
		console.error("ERROR: infrastructure/ports.json not found.");
		console.error("  Expected path:", portsJsonPath);
		return 1;
	}

	let portsJson: {
		base: number;
		services: Record<string, { offset: number | null; port: number; description: string }>;
	};
	try {
		portsJson = JSON.parse(fs.readFileSync(portsJsonPath, "utf-8")) as typeof portsJson;
	} catch {
		console.error("ERROR: Failed to parse infrastructure/ports.json.");
		return 1;
	}

	// Resolve the canonical ports from ports.json directly (no ORQA_PORT_BASE — static files
	// always use the default ports, not overrides).
	const vitePort = portsJson.services["vite"]?.port;
	const devtoolsPort = portsJson.services["devtools"]?.port;
	const forgejoHttpPort = portsJson.services["forgejo_http"]?.port;
	const forgejoSshPort = portsJson.services["forgejo_ssh"]?.port;

	if (!vitePort || !devtoolsPort || !forgejoHttpPort || !forgejoSshPort) {
		console.error("ERROR: ports.json is missing required service entries.");
		return 1;
	}

	console.log("Checking port consistency against infrastructure/ports.json...\n");

	const allMismatches: Mismatch[] = [
		// App Tauri config: devUrl must be http://localhost:<vite port>
		...checkTauriConf(root, "app/src-tauri/tauri.conf.json", "vite", vitePort),
		// DevTools Tauri config: devUrl must be http://localhost:<devtools port>
		...checkTauriConf(root, "devtools/src-tauri/tauri.conf.json", "devtools", devtoolsPort),
		// Docker Compose: must expose forgejo_http and forgejo_ssh host ports
		...checkDockerCompose(root, "infrastructure/orqastudio-git/docker-compose.yml", [
			{ port: forgejoHttpPort, label: "forgejo_http" },
			{ port: forgejoSshPort, label: "forgejo_ssh" },
		]),
	];

	if (allMismatches.length === 0) {
		console.log("All static config files match ports.json.");
		return 0;
	}

	console.error(`Found ${allMismatches.length} port mismatch(es):\n`);
	for (const m of allMismatches) {
		if (m.actual === 0) {
			console.error(`  MISSING  ${m.file} → ${m.field} (expected port ${m.expected} not found)`);
		} else {
			console.error(`  MISMATCH ${m.file} → ${m.field}: got ${m.actual}, expected ${m.expected}`);
		}
	}
	console.error(
		"\nUpdate the listed files to use the canonical ports from infrastructure/ports.json.",
	);
	return 1;
}
