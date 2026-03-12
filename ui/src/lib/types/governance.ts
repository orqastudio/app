export interface GovernanceScanResult {
	areas: GovernanceArea[];
	coverage_ratio: number;
}

export interface GovernanceArea {
	name: string;
	source: string;
	files: GovernanceFile[];
	covered: boolean;
}

export interface GovernanceFile {
	path: string;
	size_bytes: number;
	content_preview: string;
}

export interface GovernanceAnalysis {
	id: number;
	project_id: number;
	scan_data: GovernanceScanResult;
	summary: string;
	strengths: string[];
	gaps: string[];
	session_id: number | null;
	created_at: string;
}

export type RecommendationPriority = "critical" | "recommended" | "optional";
export type RecommendationStatus = "pending" | "approved" | "rejected" | "applied";

export interface Recommendation {
	id: number;
	project_id: number;
	analysis_id: number;
	category: string;
	priority: RecommendationPriority;
	title: string;
	description: string;
	artifact_type: string;
	target_path: string;
	content: string;
	rationale: string;
	status: RecommendationStatus;
	applied_at: string | null;
	created_at: string;
	updated_at: string;
}
