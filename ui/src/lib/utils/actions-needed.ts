import type { ArtifactNode } from "$lib/types/artifact-graph";

/** Terminal states where no actions are needed. */
const TERMINAL_STATES = new Set([
	"done",
	"complete",
	"promoted",
	"accepted",
	"archived",
	"superseded",
	"surpassed",
]);

/** Minimal interface for the graph query methods we need. */
interface GraphResolver {
	resolve(id: string): ArtifactNode | undefined;
}

/**
 * Determine whether an artifact has pending actions that need attention.
 *
 * Uses the same logic as ActionsNeeded.svelte but returns a boolean.
 * Also checks graph-level conditions like epics without tasks.
 */
export function hasActionsNeeded(
	node: ArtifactNode,
	graph: GraphResolver,
): boolean {
	const status = (node.status ?? "").toLowerCase();
	const type = node.artifact_type.toLowerCase();

	if (TERMINAL_STATES.has(status)) return false;

	// Status-based checks (mirroring ActionsNeeded.svelte)
	if (type === "task") {
		if (status === "todo" || status === "in-progress") return true;
	}

	if (type === "epic") {
		if (
			status === "draft" ||
			status === "ready" ||
			status === "in-progress" ||
			status === "review"
		) {
			return true;
		}

		// Epic without tasks: no incoming references from tasks via the "epic" field
		const incomingTasks = node.references_in.filter(
			(ref) =>
				ref.field === "epic" &&
				graph.resolve(ref.source_id)?.artifact_type === "task",
		);
		if (incomingTasks.length === 0) return true;
	}

	if (type === "idea") {
		if (
			status === "captured" ||
			status === "exploring" ||
			status === "shaped"
		) {
			return true;
		}
	}

	if (type === "lesson") {
		if (status === "recurring") return true;
		if (status === "active") {
			const recurrence = Number(node.frontmatter["recurrence"] ?? 0);
			if (recurrence >= 2) return true;
		}
	}

	if (type === "decision") {
		if (status === "proposed") return true;
	}

	if (type === "research") {
		if (status === "draft") return true;
	}

	if (type === "milestone") {
		if (status === "planning" || status === "active") return true;
	}

	return false;
}
