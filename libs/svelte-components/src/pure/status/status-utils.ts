/**
 * Status utilities — used internally by the Status component
 * and exported for consumers needing consistent status rendering.
 */

export interface StatusConfig {
	readonly key: string;
	readonly label: string;
	readonly icon: string;
	readonly color?: string;
	readonly spin?: boolean;
}

/** Default OrqaStudio canonical statuses. */
export const DEFAULT_STATUSES: StatusConfig[] = [
	{ key: "captured", label: "Captured", icon: "circle", color: "muted" },
	{ key: "exploring", label: "Exploring", icon: "compass", color: "primary" },
	{ key: "ready", label: "Ready", icon: "circle-dot", color: "success" },
	{ key: "prioritised", label: "Prioritised", icon: "circle-star", color: "warning" },
	{ key: "active", label: "Active", icon: "circle-dot-dashed", color: "primary", spin: true },
	{ key: "hold", label: "On Hold", icon: "circle-pause", color: "warning" },
	{ key: "blocked", label: "Blocked", icon: "circle-stop", color: "destructive" },
	{ key: "review", label: "Review", icon: "circle-user-round", color: "primary" },
	{ key: "completed", label: "Completed", icon: "circle-check-big", color: "success" },
	{ key: "surpassed", label: "Surpassed", icon: "circle-minus", color: "muted" },
	{ key: "recurring", label: "Recurring", icon: "circle-fading-arrow-up", color: "primary" },
	{ key: "archived", label: "Archived", icon: "archive", color: "muted" },
];

/**
 * Resolve a status key to its full config.
 * Searches the provided statuses first, falls back to defaults.
 * @param key
 * @param statuses
 */
export function resolveStatus(key: string, statuses?: readonly StatusConfig[]): StatusConfig {
	const k = key.toLowerCase();
	const found = (statuses ?? DEFAULT_STATUSES).find((s) => s.key === k);
	if (found) return found;
	// Check defaults if custom statuses were provided but didn't match
	if (statuses) {
		const fallback = DEFAULT_STATUSES.find((s) => s.key === k);
		if (fallback) return fallback;
	}
	return { key: k, label: key, icon: "circle", color: "muted" };
}

/**
 * Get the display label for a status key.
 * @param key
 * @param statuses
 */
export function statusLabel(key: string, statuses?: readonly StatusConfig[]): string {
	return resolveStatus(key, statuses).label;
}

/**
 * Get the icon name for a status key.
 * @param key
 * @param statuses
 */
export function statusIconName(key: string, statuses?: readonly StatusConfig[]): string {
	return resolveStatus(key, statuses).icon;
}

/**
 * Get the semantic color for a status key.
 * @param key
 * @param statuses
 */
export function statusColor(key: string, statuses?: readonly StatusConfig[]): string {
	return resolveStatus(key, statuses).color ?? "muted";
}

/**
 * Whether a status should show a spinning animation.
 * @param key
 * @param statuses
 */
export function statusIsSpinning(key: string, statuses?: readonly StatusConfig[]): boolean {
	return resolveStatus(key, statuses).spin === true;
}

/**
 * Map semantic color names to Tailwind text classes.
 * Used by the Status component and available for custom rendering.
 */
export const STATUS_COLOR_CLASSES: Record<string, string> = {
	primary: "text-primary",
	success: "text-success",
	warning: "text-warning",
	destructive: "text-destructive",
	muted: "text-muted-foreground",
};

/**
 * Get the Tailwind text color class for a status key.
 * @param key
 * @param statuses
 */
export function statusColorClass(key: string, statuses?: readonly StatusConfig[]): string {
	const color = statusColor(key, statuses);
	return STATUS_COLOR_CLASSES[color] ?? "text-muted-foreground";
}
