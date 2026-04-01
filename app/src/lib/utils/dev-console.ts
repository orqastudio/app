/**
 * Dev-mode logging setup.
 *
 * Sets the log level to debug in dev mode and intercepts browser console
 * methods to forward entries to the dev dashboard at localhost:10130/log.
 * Dashboard forwarding for SDK logger entries is handled by the SDK's
 * built-in forwardToDashboard() in emit().
 *
 * Call `initDevConsole()` once at app startup.
 */

import { setLogLevel } from "@orqastudio/sdk";

/**
 * Initialize dev console settings.
 * Only has an effect when `import.meta.env.DEV` is true.
 */
export function initDevConsole() {
	if (!import.meta.env.DEV) return;

	// Set log level to debug in dev mode
	setLogLevel("debug");

	// Intercept console methods and forward to the dev dashboard.
	// The original method is always called first so normal browser output is unaffected.
	const methods = ["log", "warn", "error", "debug"] as const;
	for (const method of methods) {
		const original = console[method].bind(console);
		console[method] = (...args: unknown[]) => {
			// Always invoke the original so browser devtools output is preserved.
			original(...args);

			// Forward to the dev dashboard using sendBeacon to avoid CORS issues.
			const message = args
				.map((a) => (typeof a === "string" ? a : JSON.stringify(a)))
				.join(" ");
			const level = method === "log" ? "info" : method;
			const payload = JSON.stringify({ level, source: "console", message });
			navigator.sendBeacon(
				"http://localhost:10130/log",
				new Blob([payload], { type: "text/plain" }),
			);
		};
	}
}
