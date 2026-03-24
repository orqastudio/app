/**
 * Dev-mode logging setup.
 *
 * Connects the SDK's centralized logger to the dev controller dashboard.
 * In dev mode, all log entries are forwarded to http://localhost:10401/log.
 *
 * Call `initDevConsole()` once at app startup.
 */

import { subscribeToLogs, setLogLevel } from "@orqastudio/sdk";
import type { LogEntry } from "@orqastudio/sdk";

const DEV_LOG_URL = "http://localhost:10401/log";

function forwardEntry(entry: LogEntry) {
	try {
		const body = JSON.stringify({
			level: entry.level,
			source: entry.source,
			message: `[${entry.source}] ${entry.message}`,
		});
		if (navigator.sendBeacon) {
			const blob = new Blob([body], { type: "application/json" });
			navigator.sendBeacon(DEV_LOG_URL, blob);
		} else {
			void fetch(DEV_LOG_URL, {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body,
				keepalive: true,
			}).catch(() => {});
		}
	} catch {
		// Never fail
	}
}

/**
 * Install dev console forwarding.
 * Only call this when `import.meta.env.DEV` is true.
 */
export function initDevConsole() {
	if (!import.meta.env.DEV) return;

	// Set log level to debug in dev mode
	setLogLevel("debug");

	// Forward all SDK log entries to the dev dashboard
	subscribeToLogs(forwardEntry);
}
