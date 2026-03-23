/**
 * Fixture data for testing and Storybook development.
 * Provides realistic artifacts, project config, and statuses
 * so the library can be developed without the app or backend.
 */

export const FIXTURE_STATUSES = [
	{ key: "captured", label: "Captured", icon: "circle-dot", color: "muted" },
	{ key: "exploring", label: "Exploring", icon: "search", color: "primary" },
	{ key: "ready", label: "Ready", icon: "check-circle", color: "success" },
	{ key: "prioritised", label: "Prioritised", icon: "arrow-up-circle", color: "warning" },
	{ key: "active", label: "Active", icon: "play-circle", color: "primary" },
	{ key: "hold", label: "On Hold", icon: "pause-circle", color: "warning" },
	{ key: "blocked", label: "Blocked", icon: "x-circle", color: "destructive" },
	{ key: "review", label: "In Review", icon: "eye", color: "primary" },
	{ key: "completed", label: "Completed", icon: "check-circle-2", color: "success" },
	{ key: "surpassed", label: "Surpassed", icon: "fast-forward", color: "muted" },
	{ key: "archived", label: "Archived", icon: "archive", color: "muted" },
	{ key: "recurring", label: "Recurring", icon: "repeat", color: "primary" },
] as const;

export const FIXTURE_ARTIFACT_TYPES = [
	{ key: "idea", label: "Idea", icon: "lightbulb", plural: "Ideas" },
	{ key: "research", label: "Research", icon: "book-open", plural: "Research" },
	{ key: "epic", label: "Epic", icon: "target", plural: "Epics" },
	{ key: "task", label: "Task", icon: "check-square", plural: "Tasks" },
	{ key: "milestone", label: "Milestone", icon: "flag", plural: "Milestones" },
	{ key: "decision", label: "Decision", icon: "scale", plural: "Decisions" },
	{ key: "rule", label: "Rule", icon: "shield", plural: "Rules" },
	{ key: "lesson", label: "Lesson", icon: "graduation-cap", plural: "Lessons" },
	{ key: "pillar", label: "Pillar", icon: "columns", plural: "Pillars" },
	{ key: "skill", label: "Skill", icon: "zap", plural: "Skills" },
	{ key: "agent", label: "Agent", icon: "bot", plural: "Agents" },
] as const;

export const FIXTURE_ARTIFACTS = [
	{
		id: "EPIC-001",
		type: "epic",
		title: "Core Architecture",
		status: "active",
		description: "Define and implement the foundational architecture for OrqaStudio.",
		relationships: [
			{ type: "delivers", target: "MS-001" },
			{ type: "driven-by", target: "AD-001" },
		],
	},
	{
		id: "EPIC-002",
		type: "epic",
		title: "Artifact Graph Engine",
		status: "active",
		description: "Build the graph-based artifact relationship engine.",
		relationships: [
			{ type: "delivers", target: "MS-001" },
		],
	},
	{
		id: "TASK-001",
		type: "task",
		title: "Set up Tauri v2 project scaffold",
		status: "completed",
		description: "Initialize the Tauri v2 project with Svelte 5 frontend and Rust backend.",
		relationships: [
			{ type: "delivers", target: "EPIC-001" },
		],
	},
	{
		id: "TASK-002",
		type: "task",
		title: "Implement SQLite schema",
		status: "active",
		description: "Design and implement the SQLite schema for artifact storage.",
		relationships: [
			{ type: "delivers", target: "EPIC-001" },
			{ type: "depends-on", target: "TASK-001" },
		],
	},
	{
		id: "TASK-003",
		type: "task",
		title: "Build graph query engine",
		status: "exploring",
		description: "Implement graph traversal and query capabilities.",
		relationships: [
			{ type: "delivers", target: "EPIC-002" },
		],
	},
	{
		id: "MS-001",
		type: "milestone",
		title: "MVP Foundation",
		status: "active",
		description: "Core platform capabilities for first usable version.",
		relationships: [],
	},
	{
		id: "AD-001",
		type: "decision",
		title: "Use Tauri v2 for desktop runtime",
		status: "completed",
		description: "Chosen for native performance, small binary size, and Rust backend.",
		relationships: [
			{ type: "drives", target: "EPIC-001" },
		],
	},
	{
		id: "RULE-001",
		type: "rule",
		title: "All relationships must use canonical vocabulary",
		status: "active",
		description: "No ad-hoc relationship types. All must come from the canonical set.",
		relationships: [
			{ type: "enforces", target: "AD-001" },
		],
	},
	{
		id: "PILLAR-001",
		type: "pillar",
		title: "Clarity Through Structure",
		status: "active",
		description: "Making thinking visible. If it's not structured and browsable, it doesn't exist yet.",
		relationships: [],
	},
	{
		id: "PILLAR-002",
		type: "pillar",
		title: "Learning Through Reflection",
		status: "active",
		description: "The system improves over time through documented mistakes and extracted patterns.",
		relationships: [],
	},
	{
		id: "PILLAR-003",
		type: "pillar",
		title: "Purpose Through Continuity",
		status: "active",
		description: "Original intent survives implementation pressure.",
		relationships: [],
	},
	{
		id: "IMPL-001",
		type: "lesson",
		title: "Always validate relationship vocabulary at commit time",
		status: "active",
		description: "Discovered through a bug where ad-hoc relationship types bypassed validation.",
		relationships: [
			{ type: "informed-by", target: "RULE-001" },
		],
	},
	{
		id: "IDEA-001",
		type: "idea",
		title: "Plugin marketplace for community extensions",
		status: "captured",
		description: "Allow users to share and install plugins from a curated marketplace.",
		relationships: [],
	},
	{
		id: "RES-001",
		type: "research",
		title: "Graph visualization libraries comparison",
		status: "completed",
		description: "Evaluated Cytoscape, D3, and Sigma.js for graph rendering.",
		relationships: [
			{ type: "informs", target: "EPIC-002" },
		],
	},
	{
		id: "SKILL-001",
		type: "skill",
		title: "Artifact Status Management",
		status: "active",
		description: "Skill for managing artifact lifecycle status transitions.",
		relationships: [],
	},
];

export const FIXTURE_PROJECT_SETTINGS = {
	name: "OrqaStudio",
	version: "0.1.0",
	statuses: FIXTURE_STATUSES,
	artifactTypes: FIXTURE_ARTIFACT_TYPES,
	delivery: {
		milestoneTypes: ["milestone"],
		epicTypes: ["epic"],
		taskTypes: ["task"],
	},
};
