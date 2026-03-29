// Milestone display configuration for the RoadmapView.
// Centralises status-to-colour mappings so all milestone components share one definition.

/** Tailwind class for the milestone status dot colour. */
export const MILESTONE_STATUS_DOT_COLORS: Record<string, string> = {
	active: "bg-green-500",
	completed: "bg-muted-foreground",
	planned: "bg-blue-400",
};

/** Tailwind classes for the milestone status badge (background + text). */
export const MILESTONE_STATUS_BADGE_COLORS: Record<string, string> = {
	active: "bg-green-500/20 text-green-700 dark:text-green-400",
};

/** Default dot colour for unknown milestone statuses. */
export const MILESTONE_STATUS_DOT_DEFAULT = "bg-muted-foreground";

/** Default badge colour for unknown milestone statuses. */
export const MILESTONE_STATUS_BADGE_DEFAULT = "bg-muted text-muted-foreground";
