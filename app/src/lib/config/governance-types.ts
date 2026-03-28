// Governance artifact types tracked by the dashboard and improvement trends widget.
// Centralised so that adding a new governance type only requires editing this file.

/** All artifact types that count as governance artifacts. */
export const GOVERNANCE_TYPES = ["rule", "lesson", "decision"] as const;

export type GovernanceType = (typeof GOVERNANCE_TYPES)[number];
