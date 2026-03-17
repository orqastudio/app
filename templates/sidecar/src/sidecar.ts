/**
 * Sidecar entry point — long-running NDJSON process.
 *
 * Reads SidecarRequest from stdin (one JSON per line),
 * writes SidecarResponse to stdout (one JSON per line).
 *
 * Replace the handler implementations with your provider logic.
 */

import * as readline from "node:readline";

interface SidecarRequest {
	type: string;
	[key: string]: unknown;
}

interface SidecarResponse {
	type: string;
	[key: string]: unknown;
}

function send(response: SidecarResponse): void {
	process.stdout.write(JSON.stringify(response) + "\n");
}

async function handleRequest(request: SidecarRequest): Promise<void> {
	switch (request.type) {
		case "health_check":
			send({ type: "health_ok", version: "0.1.0" });
			break;
		default:
			send({ type: "error", message: `Unknown request type: ${request.type}` });
	}
}

// Main loop — read NDJSON from stdin
const rl = readline.createInterface({ input: process.stdin });

rl.on("line", async (line: string) => {
	try {
		const request = JSON.parse(line) as SidecarRequest;
		await handleRequest(request);
	} catch (err) {
		send({ type: "error", message: `Parse error: ${err}` });
	}
});

process.on("SIGTERM", () => process.exit(0));
process.on("SIGINT", () => process.exit(0));
