/**
 * Global error store for surfacing backend, sidecar, and frontend errors.
 *
 * Listens to:
 * - `app-error` Tauri events (Rust errors, sidecar errors)
 * - `window.onerror` and `window.onunhandledrejection` (frontend errors)
 *
 * Exposes a reactive list of recent errors for the ErrorToast component.
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

	// Capture uncaught errors (all modes)
	window.onerror = (_message, source, lineno, colno, error) => {
		const msg = error?.message ?? String(_message);
		const location = source ? ` (${source}:${lineno}:${colno})` : "";
		addError("frontend", `${msg}${location}`);
	};

	// Capture unhandled promise rejections (all modes)
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
	window.onerror = null;
	window.onunhandledrejection = null;
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
