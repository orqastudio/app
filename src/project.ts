export interface Project {
	id: number;
	name: string;
	path: string;
	description: string | null;
	detected_stack: DetectedStack | null;
	created_at: string;
	updated_at: string;
}

export interface ProjectSummary {
	id: number;
	name: string;
	path: string;
	detected_stack: DetectedStack | null;
	session_count: number;
	artifact_count: number;
	updated_at: string;
}

export interface DetectedStack {
	languages: string[];
	frameworks: string[];
	package_manager: string | null;
	has_claude_config: boolean;
	has_design_tokens: boolean;
}

export interface ScanResult {
	project_id: number;
	detected_stack: DetectedStack;
	artifact_counts: Record<string, number>;
	design_tokens_found: boolean;
	scan_duration_ms: number;
}

export interface ProjectSettings {
	name: string;
	description: string | null;
	default_model: string;
	excluded_paths: string[];
	stack: DetectedStack | null;
	governance: GovernanceCounts | null;
	icon: string | null;
	show_thinking: boolean;
	custom_system_prompt: string | null;
	artifacts?: ArtifactEntry[];
}

export interface GovernanceCounts {
	docs: number;
	agents: number;
	rules: number;
	skills: number;
	hooks: number;
	has_claude_config: boolean;
}

export interface ProjectScanResult {
	stack: DetectedStack;
	governance: GovernanceCounts;
	scan_duration_ms: number;
}

/** A single artifact type entry from project.json artifacts config.
 *
 * `label` and `icon` are optional — the scanner reads them from the directory's
 * README.md frontmatter when absent, falling back to a humanized key name.
 */
export interface ArtifactTypeConfig {
	key: string;
	label?: string;
	icon?: string;
	path: string;
}

/** A group entry containing child artifact types.
 *
 * `label` and `icon` are optional — presentation metadata comes from the group
 * directory's README.md frontmatter, not from this config.
 */
export interface ArtifactGroupConfig {
	key: string;
	label?: string;
	icon?: string;
	children: ArtifactTypeConfig[];
}

/** An entry in the artifacts config — either a direct type or a group. */
export type ArtifactEntry = ArtifactTypeConfig | ArtifactGroupConfig;

/** Type guard: is this entry a group (has children)? */
export function isArtifactGroup(entry: ArtifactEntry): entry is ArtifactGroupConfig {
	return "children" in entry;
}

/**
 * A project-level relationship type defined in `project.json`.
 *
 * Extends the canonical relationship vocabulary with project-specific pairs
 * (e.g. `depends-on` / `depended-on-by`).
 */
export interface ProjectRelationshipConfig {
	/** The forward relationship key (e.g. "depends-on"). */
	key: string;
	/** The inverse relationship key (e.g. "depended-on-by"). */
	inverse: string;
	/** Human-readable label for the forward direction. */
	label?: string;
	/** Human-readable label for the inverse direction. */
	inverse_label?: string;
}
