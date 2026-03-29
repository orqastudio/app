/**
 * Global error store for surfacing backend, sidecar, and frontend errors.
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

/** Reactive store for surfacing backend, sidecar, and frontend errors as toasts. */
export class ErrorStoreImpl {
	/** Active errors, newest first, capped at MAX_ERRORS. */
	errors = $state<AppError[]>([]);
	private nextId = 0;
	private unlistenAppError: UnlistenFn | null = null;
	private initialized = false;
	private browserHandlersInstalled = false;

	/**
	 * Add an error to the store and auto-dismiss after TOAST_DURATION_MS.
	 * @param source - Origin of the error (e.g. "frontend", "backend").
	 * @param message - Human-readable error description.
	 * @param level - Severity level, defaults to "error".
	 */
	addError(source: string, message: string, level: string = "error") {
		const error: AppError = {
			id: this.nextId++,
			source,
			message,
			level,
			timestamp: Date.now(),
		};

		this.errors = [error, ...this.errors].slice(0, MAX_ERRORS);

		setTimeout(() => {
			this.dismiss(error.id);
		}, TOAST_DURATION_MS);
	}

	/**
	 * Dismiss a single error by its ID.
	 * @param id - The numeric ID assigned when the error was added.
	 */
	dismiss(id: number) {
		this.errors = this.errors.filter((e) => e.id !== id);
	}

	/** Dismiss all active errors. */
	dismissAll() {
		this.errors = [];
	}

	/**
	 * Subscribe to the Tauri "app-error" event from the backend.
	 * Safe to call multiple times — subsequent calls are no-ops.
	 */
	async initialize() {
		if (this.initialized) return;
		this.initialized = true;

		this.unlistenAppError = await listen<{
			source: string;
			message: string;
			level: string;
		}>("app-error", (event) => {
			this.addError(event.payload.source, event.payload.message, event.payload.level);
		});
	}

	/**
	 * Set up browser-specific global error handlers.
	 * Captures `window.onerror` and `window.onunhandledrejection`.
	 */
	initBrowserHandlers(): void {
		if (this.browserHandlersInstalled) return;
		if (typeof window === "undefined") return;
		this.browserHandlersInstalled = true;

		window.onerror = (_message, source, lineno, colno, error) => {
			const msg = error?.message ?? String(_message);
			const location = source ? ` (${source}:${lineno}:${colno})` : "";
			this.addError("frontend", `${msg}${location}`);
		};

		window.onunhandledrejection = (event: PromiseRejectionEvent) => {
			const msg =
				event.reason instanceof Error
					? event.reason.message
					: String(event.reason);
			this.addError("frontend", `Unhandled rejection: ${msg}`);
		};
	}

	/** Tear down event listeners and reset browser global error handlers. */
	destroy() {
		this.unlistenAppError?.();
		this.unlistenAppError = null;
		if (typeof window !== "undefined" && this.browserHandlersInstalled) {
			window.onerror = null;
			window.onunhandledrejection = null;
		}
		this.browserHandlersInstalled = false;
		this.initialized = false;
	}
}
