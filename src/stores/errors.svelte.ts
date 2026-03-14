/**
 * Global error store for surfacing backend, sidecar, and frontend errors.
 *
 * Core error accumulation (addError, dismiss, etc.) works without browser handlers.
 * Call `initBrowserHandlers()` separately to set up window.onerror and
 * window.onunhandledrejection — opt-in for browser environments.
 *
 * Tauri event listening (`app-error`) is set up in `initialize()` and requires
 * `@tauri-apps/api/event` to be available.
 */
import { listen } from "@tauri-apps/api/event";
import type { UnlistenFn } from "@tauri-apps/api/event";

export interface AppError {
	id: number;
	source: string;
	message: string;
	level: string;
	timestamp: number;
}

const MAX_ERRORS = 50;
const TOAST_DURATION_MS = 8000;

let nextId = 0;
let errors = $state<AppError[]>([]);
let unlistenAppError: UnlistenFn | null = null;
let initialized = false;
let browserHandlersInstalled = false;

function addError(source: string, message: string, level: string = "error") {
	const error: AppError = {
		id: nextId++,
		source,
		message,
		level,
		timestamp: Date.now(),
	};

	errors = [error, ...errors].slice(0, MAX_ERRORS);

	// Auto-dismiss after duration
	setTimeout(() => {
		dismiss(error.id);
	}, TOAST_DURATION_MS);
}

function dismiss(id: number) {
	errors = errors.filter((e) => e.id !== id);
}

function dismissAll() {
	errors = [];
}

async function initialize() {
	if (initialized) return;
	initialized = true;

	// Listen for Tauri app-error events from the Rust backend
	unlistenAppError = await listen<{
		source: string;
		message: string;
		level: string;
	}>("app-error", (event) => {
		addError(event.payload.source, event.payload.message, event.payload.level);
	});
}

/**
 * Set up browser-specific global error handlers.
 *
 * Captures `window.onerror` and `window.onunhandledrejection` to feed errors
 * into the store. This is opt-in — call it only in browser environments where
 * you want global error capture.
 */
function initBrowserHandlers(): void {
	if (browserHandlersInstalled) return;
	if (typeof window === "undefined") return;
	browserHandlersInstalled = true;

	window.onerror = (_message, source, lineno, colno, error) => {
		const msg = error?.message ?? String(_message);
		const location = source ? ` (${source}:${lineno}:${colno})` : "";
		addError("frontend", `${msg}${location}`);
	};

	window.onunhandledrejection = (event: PromiseRejectionEvent) => {
		const msg =
			event.reason instanceof Error
				? event.reason.message
				: String(event.reason);
		addError("frontend", `Unhandled rejection: ${msg}`);
	};
}

function destroy() {
	unlistenAppError?.();
	unlistenAppError = null;
	if (typeof window !== "undefined" && browserHandlersInstalled) {
		window.onerror = null;
		window.onunhandledrejection = null;
	}
	browserHandlersInstalled = false;
	initialized = false;
}

export const errorStore = {
	get errors() {
		return errors;
	},
	addError,
	dismiss,
	dismissAll,
	initialize,
	destroy,
};

export { initBrowserHandlers };
