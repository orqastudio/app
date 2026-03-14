/**
 * Global toast notification store.
 *
 * Provides a reactive list of toast notifications with auto-dismiss support.
 * Toasts are typed by severity: success, error, warning, info.
 *
 * Usage:
 *   import { toast } from "$lib/stores/toast.svelte";
 *   toast.success("Item saved");
 *   toast.error("Something went wrong");
 *   toast.warning("Unsaved changes");
 *   toast.info("New version available");
 */

export type ToastType = "success" | "error" | "warning" | "info";

export interface Toast {
	id: string;
	message: string;
	type: ToastType;
	duration: number;
}

import { SvelteMap } from "svelte/reactivity";

const DEFAULT_DURATION_MS = 4000;
const MAX_TOASTS = 10;

let nextId = 0;
let toasts = $state<Toast[]>([]);
const dismissTimers = new SvelteMap<string, ReturnType<typeof setTimeout>>();

function generateId(): string {
	return `toast-${nextId++}-${Date.now()}`;
}

function add(message: string, type: ToastType, duration: number = DEFAULT_DURATION_MS): string {
	const id = generateId();
	const entry: Toast = { id, message, type, duration };

	toasts = [entry, ...toasts].slice(0, MAX_TOASTS);

	if (duration > 0) {
		const timer = setTimeout(() => {
			dismiss(id);
		}, duration);
		dismissTimers.set(id, timer);
	}

	return id;
}

function dismiss(id: string): void {
	const timer = dismissTimers.get(id);
	if (timer !== undefined) {
		clearTimeout(timer);
		dismissTimers.delete(id);
	}
	toasts = toasts.filter((t) => t.id !== id);
}

function dismissAll(): void {
	for (const timer of dismissTimers.values()) {
		clearTimeout(timer);
	}
	dismissTimers.clear();
	toasts = [];
}

export const toastStore = {
	get toasts(): Toast[] {
		return toasts;
	},
	add,
	dismiss,
	dismissAll,
};

/** Convenience functions for triggering toasts by type. */
export const toast = {
	success(message: string, duration?: number): string {
		return add(message, "success", duration);
	},
	error(message: string, duration?: number): string {
		return add(message, "error", duration);
	},
	warning(message: string, duration?: number): string {
		return add(message, "warning", duration);
	},
	info(message: string, duration?: number): string {
		return add(message, "info", duration);
	},
	dismiss(id: string): void {
		dismiss(id);
	},
	dismissAll(): void {
		dismissAll();
	},
};
