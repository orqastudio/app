// Icon mapping for artifact types used in traceability and relationship displays.
// Centralised here so TraceabilityPanel and any future relationship views share the same mapping.

/**
 * Returns the Lucide icon name for a given artifact type.
 * Falls back to "file-text" for unrecognised types.
 */
export function iconForArtifactType(artifactType: string): string {
	const icons: Record<string, string> = {
		pillar: "columns-3",
		vision: "telescope",
		epic: "layers",
		task: "check-square",
		milestone: "flag",
		idea: "lightbulb",
		decision: "scale",
		research: "microscope",
		rule: "shield",
		knowledge: "book-open",
		agent: "bot",
		hook: "webhook",
	};
	return icons[artifactType] ?? "file-text";
}
