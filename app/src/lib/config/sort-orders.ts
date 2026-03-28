// Sort order configuration for DynamicArtifactTable.
// Defines numeric sort keys for priority and status values used when ordering
// artifact rows. Lower numbers sort first; unrecognised values fall back to 99/50.

// Priority sort order — P1 first, then P2, P3, unset last.
export const PRIORITY_ORDER: Record<string, number> = {
	P1: 0,
	P2: 1,
	P3: 2,
};

// Status sort order for secondary sorting within a priority group.
export const STATUS_ORDER: Record<string, number> = {
	active: 0,
	review: 1,
	ready: 2,
	prioritised: 3,
	exploring: 4,
	captured: 5,
	blocked: 6,
	hold: 7,
	completed: 8,
	surpassed: 9,
};
