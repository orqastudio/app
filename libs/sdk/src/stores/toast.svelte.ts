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

/**
 * Reactive store that manages the list of active toast notifications.
 */
export class ToastStore {
	toasts = $state<Toast[]>([]);
	private nextId = 0;
	private dismissTimers = new SvelteMap<string, ReturnType<typeof setTimeout>>();

	private generateId(): string {
		return `toast-${this.nextId++}-${Date.now()}`;
	}

	/**
	 * Adds a new toast notification and schedules auto-dismiss if duration is positive.
	 * @param message - The text content to display in the toast.
	 * @param type - The severity type controlling the toast's visual appearance.
	 * @param duration - How long in milliseconds before the toast is auto-dismissed.
	 * @returns The generated ID for the new toast.
	 */
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

	/**
	 * Dismisses a specific toast by ID, cancelling its auto-dismiss timer.
	 * @param id - The ID of the toast to remove.
	 */
	dismiss(id: string): void {
		const timer = this.dismissTimers.get(id);
		if (timer !== undefined) {
			clearTimeout(timer);
			this.dismissTimers.delete(id);
		}
		this.toasts = this.toasts.filter((t) => t.id !== id);
	}

	/**
	 * Dismisses all active toasts and cancels all pending auto-dismiss timers.
	 */
	dismissAll(): void {
		for (const timer of this.dismissTimers.values()) {
			clearTimeout(timer);
		}
		this.dismissTimers.clear();
		this.toasts = [];
	}
}

/**
 * Creates convenience helper functions bound to a specific ToastStore instance.
 * @param store - The ToastStore instance to bind the helpers to.
 * @returns An object with typed shorthand methods for each toast severity level.
 */
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
