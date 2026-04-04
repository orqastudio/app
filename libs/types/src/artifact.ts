export interface Artifact {
	readonly id: number;
	readonly project_id: number;
	readonly artifact_type: ArtifactType;
	readonly rel_path: string;
	readonly name: string;
	readonly description: string | null;
	readonly content: string;
	readonly file_hash: string | null;
	readonly file_size: number | null;
	readonly file_modified_at: string | null;
	readonly compliance_status: ComplianceStatus;
	readonly relationships: readonly ArtifactRelationship[] | null;
	readonly metadata: Readonly<Record<string, unknown>> | null;
	readonly created_at: string;
	readonly updated_at: string;
}

export interface ArtifactSummary {
	readonly id: number;
	readonly artifact_type: ArtifactType;
	readonly rel_path: string;
	readonly name: string;
	readonly description: string | null;
	readonly compliance_status: ComplianceStatus;
	readonly file_modified_at: string | null;
}

/** Artifact type key — string from plugin registry, not a hardcoded enum. */
export type ArtifactType = string;
export type ComplianceStatus = "compliant" | "non_compliant" | "unknown" | "error";

export interface ArtifactRelationship {
	readonly type: string;
	readonly target: string;
}

/** A node in the documentation tree returned by doc_tree_scan. */
export interface DocNode {
	/** Display name: filename without .md, hyphens replaced with spaces, title-cased. */
	readonly label: string;
	/** Relative path from docs/ without .md extension (e.g. "product/vision"). Null for directories. */
	readonly path: string | null;
	/** Child nodes for directories. Null for leaf files. */
	readonly children: readonly DocNode[] | null;
	/** Optional short description shown below the label for flat-list items. */
	readonly description?: string | null;
}
