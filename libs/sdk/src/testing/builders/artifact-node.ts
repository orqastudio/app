/**
 * Test builder for ArtifactNode objects.
 *
 * Produces valid nodes with sensible defaults. All fields are overridable.
 * The @orqastudio/types dependency will be added later — for now, the
 * interface is defined locally.
 */

/** Minimal ArtifactNode shape for test building. */
export interface ArtifactNode {
	id: string;
	path: string;
	artifactType: string;
	title: string;
	status: string;
	frontmatter: Record<string, unknown>;
	referencesOut: string[];
	referencesIn: string[];
	body: string;
}

const defaults: ArtifactNode = {
	id: "TEST-001",
	path: ".orqa/implementation/tasks/TEST-001.md",
	artifactType: "task",
	title: "Test artifact",
	status: "todo",
	frontmatter: {},
	referencesOut: [],
	referencesIn: [],
	body: "",
};

/**
 * Create a test ArtifactNode with sensible defaults.
 * Pass partial overrides to customise specific fields.
 *
 * ```ts
 * const node = createTestNode({ id: "EPIC-042", artifactType: "epic", status: "draft" });
 * ```
 */
export function createTestNode(overrides: Partial<ArtifactNode> = {}): ArtifactNode {
	return {
		...defaults,
		...overrides,
		frontmatter: {
			...defaults.frontmatter,
			...(overrides.frontmatter ?? {}),
		},
		referencesOut: overrides.referencesOut ?? [...defaults.referencesOut],
		referencesIn: overrides.referencesIn ?? [...defaults.referencesIn],
	};
}
