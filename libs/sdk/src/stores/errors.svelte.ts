/**
 * Global error store for surfacing backend, sidecar, and frontend errors.
 */
import { listen } from "@tauri-apps/api/event";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { logger } from "../logger.js";

const log = logger("errors");

export interface AppError {
	id: number;
	source: string;
	message: string;
	level: string;
	timestamp: number;
}

const MAX_ERRORS = 50;
const TOAST_DURATION_MS = 8000;

export class ErrorStoreImpl {
	errors = $state<AppError[]>([]);
	private nextId = 0;
	private unlistenAppError: UnlistenFn | null = null;
	private initialized = false;
	private browserHandlersInstalled = false;

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

	dismiss(id: number) {
		this.errors = this.errors.filter((e) => e.id !== id);
	}

	dismissAll() {
		this.errors = [];
	}

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

/**
 * @deprecated Use `getStores().errorStore.initBrowserHandlers()` instead.
 * Kept for backward compatibility during migration.
 */
export function initBrowserHandlers(): void {
	// no-op — callers should use the instance method
	log.warn("initBrowserHandlers() is deprecated — use getStores().errorStore.initBrowserHandlers() instead");
}
