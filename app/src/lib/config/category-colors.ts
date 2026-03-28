// Tailwind class strings for lesson category badges.
// Category colours use distinct palette values to differentiate category identity,
// not semantic state tokens. All components that display category chips import from here.

/**
 * Returns the Tailwind class string for a lesson category badge.
 */
export function categoryColor(category: string): string {
	switch (category) {
		case "process":
			return "bg-blue-500/10 text-blue-600 dark:text-blue-400";
		case "coding":
			return "bg-violet-500/10 text-violet-600 dark:text-violet-400";
		case "architecture":
			return "bg-warning/10 text-warning";
		default:
			return "bg-muted text-muted-foreground";
	}
}
