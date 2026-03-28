// Action label configuration for the DecisionQueueWidget.
// Maps artifact types to the human-readable action required for review.
// Centralised so that adding new types only requires editing this file.

/** Maps artifact type strings to the human-readable review action label. */
export const ACTION_LABELS: Record<string, string> = {
	task: "Verify task completion",
	epic: "Review epic deliverables",
	idea: "Decide on promotion",
	decision: "Accept or reject decision",
	lesson: "Promote to rule or knowledge",
	research: "Review research findings",
	milestone: "Verify milestone gate",
};

/** Default label used when an artifact type has no specific entry. */
export const DEFAULT_ACTION_LABEL = "Review required";
