export interface EnforcementRule {
	name: string;
	scope: string; // "system" | "project"
	entries: EnforcementEntry[];
	prose: string;
}

export interface EnforcementEntry {
	event: "File" | "Bash";
	action: "Block" | "Warn";
	conditions: Condition[];
	pattern: string | null;
}

export interface Condition {
	field: string;
	pattern: string;
}

export interface EnforcementViolation {
	rule_name: string;
	action: "Block" | "Warn";
	tool_name: string;
	detail: string;
	timestamp: string;
}
