/**
 * Returns Tailwind class string for a lesson category badge.
 * Category colors use distinct palette values to differentiate category identity,
 * not semantic state tokens. Duplication across components is eliminated here.
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
