// Process tracker — synthesizes ProcessInfo entries from process manager events
// in the log stream. The daemon health endpoint only reports its own subprocesses
// (LSP, MCP). Other processes (search, app, storybook) are managed by the CLI
// process manager and emit structured JSON events that flow through the
// dev_controller into the log stream. This store watches those events and
// maintains a reactive map of process status.

import { SvelteSet } from "svelte/reactivity";
import { type LogEvent } from "./log-store.svelte.js";
import type { ProcessInfo, ProcessStatus } from "../components/processes/ProcessCard.svelte";

// Process state tracked from PM events.
interface TrackedProcess {
	name: string;
	source: string;
	status: ProcessStatus;
	startedAt: number | null;
}

// Internal map of nodeId → tracked state.
const tracked = $state<Map<string, TrackedProcess>>(new Map());

/**
 * Process a log event and update the tracked process map if it's a PM event.
 * Called from the log store's event ingestion path.
 * @param event - The log event to inspect.
 */
export function trackProcessEvent(event: LogEvent): void {
	// PM events have categories like "process:daemon", "process:search", etc.
	if (!event.category.startsWith("process:")) return;

	const nodeId = event.category.slice("process:".length);
	if (!nodeId) return;

	// Extract status from the message pattern.
	const msg = event.message.toLowerCase();
	let status: ProcessStatus = "unknown";
	if (msg.includes("running") || msg.includes("ready") || msg.includes("loaded")) {
		status = "running";
	} else if (msg.includes("starting") || msg.includes("building")) {
		status = "running";
	} else if (msg.includes("stopped") || msg.includes("stopped")) {
		status = "stopped";
	} else if (msg.includes("failed") || msg.includes("crashed")) {
		status = "crashed";
	} else if (msg.includes("built")) {
		status = "running";
	}

	const existing = tracked.get(nodeId);
	if (existing) {
		existing.status = status;
		if (status === "running" && !existing.startedAt) {
			existing.startedAt = event.timestamp;
		}
	} else {
		// Derive a human-readable name from the nodeId.
		const name = nodeId.replace(/-/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
		tracked.set(nodeId, {
			name,
			source: nodeId,
			status,
			startedAt: status === "running" ? event.timestamp : null,
		});
	}
}

/**
 * Get process info entries from the PM tracker. These supplement the daemon
 * health entries. Excludes daemon/lsp/mcp since those are reported by the
 * daemon health endpoint directly.
 * @returns Array of ProcessInfo for CLI-managed processes.
 */
export function getTrackedProcesses(): ProcessInfo[] {
	const daemonManaged = new SvelteSet(["daemon", "lsp", "mcp"]);
	const now = Date.now();

	return [...tracked.values()]
		.filter((p) => !daemonManaged.has(p.source))
		.map((p) => ({
			name: p.name,
			source: p.source,
			status: p.status,
			pid: null,
			uptime_seconds: p.startedAt ? Math.floor((now - p.startedAt) / 1000) : null,
			memory_bytes: null,
			binary_path: null,
		}));
}
