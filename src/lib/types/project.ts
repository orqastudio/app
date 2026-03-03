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
