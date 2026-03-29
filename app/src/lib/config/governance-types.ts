// Governance artifact types tracked by the dashboard and improvement trends widget.
// Centralised so that adding a new governance type only requires editing this file.

/** All artifact types that count as governance artifacts. */
export const GOVERNANCE_TYPES = ["rule", "lesson", "decision"] as const;

export type GovernanceType = (typeof GOVERNANCE_TYPES)[number];

/** Canonical artifact type keys referenced by dashboard components. */
export const ARTIFACT_TYPES = {
	task: "task",
	epic: "epic",
	milestone: "milestone",
	lesson: "lesson",
	decision: "decision",
	rule: "rule",
	research: "research",
	idea: "idea",
} as const;

/** Artifact status values used for filtering and display. */
export const ARTIFACT_STATUSES = {
	review: "review",
	active: "active",
	ready: "ready",
	prioritised: "prioritised",
	completed: "completed",
	planned: "planned",
} as const;

/** Relationship type keys used by dashboard components. */
export const RELATIONSHIP_TYPES = {
	contains: "contains",
} as const;

/** Priority values used for filtering and sorting. */
export const PRIORITY_VALUES = {
	p1: "P1",
	p2: "P2",
	p3: "P3",
} as const;

/** Relationship semantic categories used to derive pipeline stages. */
export const RELATIONSHIP_SEMANTICS = {
	governance: "governance",
	knowledgeFlow: "knowledge-flow",
} as const;
