// Field classification configuration for FrontmatterHeader rendering.
// Centralises all display-routing decisions so the component stays declarative.

/**
 * Fields always rendered in the fixed header row (id, status, priority)
 * or handled outside the metadata card (title, description),
 * or excluded because they are relationship fields shown in RelationshipsList.
 * These are skipped in the dynamic body loop.
 */
export const SKIP_FIELDS = new Set([
	"id",
	"title",
	"description",
	"status",
	"priority",
	"scoring",
	"bodyTemplate",
	"tools",
	"capabilities",
	"created",
	"updated",
	"relationships",
	"enforcement",
	"rule-overrides",
	"acceptance",
]);

/** ISO date fields — rendered as human-readable dates. */
export const DATE_FIELDS = new Set(["created", "updated", "deadline"]);

/**
 * LINK_FIELDS: values that are artifact IDs and should render as clickable ArtifactLink chips.
 * After graph-first migration, most connection fields have moved to relationships.
 */
export const LINK_FIELDS = new Set(["assignee", "scope"]);

/**
 * CHIP_FIELDS: rendered as styled chips but NOT clickable links.
 */
export const CHIP_FIELDS = new Set<string>([
	"layer",
	"model",
	"maturity",
	"recurrence",
	"category",
	"version",
	"horizon",
]);

/** BOOLEAN_FIELDS: rendered as check/x icons instead of "true"/"false" text. */
export const BOOLEAN_FIELDS = new Set<string>(["user-invocable"]);

/**
 * Explicit field display order. Fields listed here are sorted to the
 * front in the given order; unlisted fields appear after them in their
 * original YAML source order.
 */
export const FIELD_ORDER: string[] = [
	"layer",
	"maturity",
	"recurrence",
	"category",
	"version",
	"horizon",
	"assignee",
];

/**
 * Returns Tailwind classes for priority badges.
 * @param priority
 */
export function priorityClass(priority: string): string {
	if (priority === "P1") return "bg-destructive/15 text-destructive border-destructive/30";
	if (priority === "P2") return "bg-warning/15 text-warning border-warning/30";
	if (priority === "P3")
		return "bg-emerald-500/15 text-emerald-600 dark:text-emerald-400 border-emerald-500/30";
	return "";
}

/**
 * Returns human-readable label for priority.
 * @param priority
 */
export function priorityLabel(priority: string): string {
	if (priority === "P1") return "P1 — Critical";
	if (priority === "P2") return "P2 — Important";
	if (priority === "P3") return "P3 — Nice to Have";
	return priority;
}
