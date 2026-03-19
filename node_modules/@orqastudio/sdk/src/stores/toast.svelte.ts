/**
 * Toast notification store.
 *
 * Provides a reactive list of toast notifications with auto-dismiss support.
 * Toasts are typed by severity: success, error, warning, info.
 */

import { SvelteMap } from "svelte/reactivity";

export type ToastType = "success" | "error" | "warning" | "info";

export interface Toast {
	id: string;
	message: string;
	type: ToastType;
	duration: number;
}

const DEFAULT_DURATION_MS = 4000;
const MAX_TOASTS = 10;

export class ToastStore {
	toasts = $state<Toast[]>([]);
	private nextId = 0;
	private dismissTimers = new SvelteMap<string, ReturnType<typeof setTimeout>>();

	private generateId(): string {
		return `toast-${this.nextId++}-${Date.now()}`;
	}

	add(message: string, type: ToastType, duration: number = DEFAULT_DURATION_MS): string {
		const id = this.generateId();
		const entry: Toast = { id, message, type, duration };

		this.toasts = [entry, ...this.toasts].slice(0, MAX_TOASTS);

		if (duration > 0) {
			const timer = setTimeout(() => {
				this.dismiss(id);
			}, duration);
			this.dismissTimers.set(id, timer);
		}

		return id;
	}

	dismiss(id: string): void {
		const timer = this.dismissTimers.get(id);
		if (timer !== undefined) {
			clearTimeout(timer);
			this.dismissTimers.delete(id);
		}
		this.toasts = this.toasts.filter((t) => t.id !== id);
	}

	dismissAll(): void {
		for (const timer of this.dismissTimers.values()) {
			clearTimeout(timer);
		}
		this.dismissTimers.clear();
		this.toasts = [];
	}
}

/** Create convenience functions bound to a ToastStore instance. */
export function createToastConvenience(store: ToastStore) {
	return {
		success(message: string, duration?: number): string {
			return store.add(message, "success", duration);
		},
		error(message: string, duration?: number): string {
			return store.add(message, "error", duration);
		},
		warning(message: string, duration?: number): string {
			return store.add(message, "warning", duration);
		},
		info(message: string, duration?: number): string {
			return store.add(message, "info", duration);
		},
		dismiss(id: string): void {
			store.dismiss(id);
		},
		dismissAll(): void {
			store.dismissAll();
		},
	};
}
