// Category configuration for ArtifactLanding.
// Defines icon, label, singular form, description, and filesystem location for each
// Claude Code artifact category. Categories reflect the Phase 7 governance structure:
// process is gone, delivery → implementation, principles → discovery.

export interface CategoryConfig {
	icon: string;
	label: string;
	singular: string;
	description: string;
	location: string;
}

// Maps ActivityView category keys to their display configuration.
export const CATEGORY_CONFIG: Record<string, CategoryConfig> = {
	agents: {
		icon: "bot",
		label: "Agents",
		singular: "agent",
		description:
			"Agent definitions give AI personas specialized knowledge and behavior for your project.",
		location: ".claude/agents/",
	},
	rules: {
		icon: "shield",
		label: "Rules",
		singular: "rule",
		description:
			"Rules enforce coding standards and project conventions. They are loaded automatically by Claude Code.",
		location: ".claude/rules/",
	},
	knowledge: {
		icon: "brain",
		label: "Knowledge",
		singular: "knowledge",
		description:
			"Knowledge files define reusable domain context that agents draw on during sessions.",
		location: ".claude/knowledge/",
	},
	hooks: {
		icon: "git-branch",
		label: "Hooks",
		singular: "hook",
		description:
			"Hooks run automated actions at lifecycle events — before/after prompts, on stop, etc.",
		location: ".claude/hooks/",
	},
};
