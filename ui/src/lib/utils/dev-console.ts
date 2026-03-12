/**
 * Dev-mode console forwarding.
 *
 * Monkey-patches `console.log`, `console.warn`, and `console.error` to POST
 * messages to the OrqaDev dashboard at `http://localhost:3001/log`.
 *
 * Gated by `import.meta.env.DEV` — stripped from production builds entirely.
 * Call `initDevConsole()` once at app startup.
 */

const DEV_LOG_URL = "http://localhost:3001/log";

/** Guard: true once patched. */
let patched = false;

function formatArgs(args: unknown[]): string {
	return args
		.map((a) => {
			if (typeof a === "string") return a;
			try {
				return JSON.stringify(a);
			} catch {
				return String(a);
			}
		})
		.join(" ");
}

function postLog(level: string, message: string) {
	// Fire-and-forget — never block console output
	try {
		const body = JSON.stringify({ level, message });
		// Use sendBeacon for reliability, fallback to fetch
		if (navigator.sendBeacon) {
			const blob = new Blob([body], { type: "application/json" });
			navigator.sendBeacon(DEV_LOG_URL, blob);
		} else {
			void fetch(DEV_LOG_URL, {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body,
				keepalive: true,
			}).catch(() => {
				// Dev dashboard not running — ignore
			});
		}
	} catch {
		// Never fail console output
	}
}

/**
 * Install dev console forwarding.
 *
 * Only call this when `import.meta.env.DEV` is true.
 * The function is a no-op if called multiple times.
 */
export function initDevConsole() {
	if (!import.meta.env.DEV) return;

	// Already patched — bail
	if (patched) return;
	patched = true;

	const savedLog = console.log.bind(console);
	const savedWarn = console.warn.bind(console);
	const savedError = console.error.bind(console);

	console.log = (...args: unknown[]) => {
		savedLog(...args);
		postLog("log", formatArgs(args));
	};

	console.warn = (...args: unknown[]) => {
		savedWarn(...args);
		postLog("warn", formatArgs(args));
	};

	console.error = (...args: unknown[]) => {
		savedError(...args);
		postLog("error", formatArgs(args));
	};
}
