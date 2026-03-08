export type ActivityView =
	| "chat"
	| "project"
	| "docs"
	| "research"
	| "plans"
	| "agents"
	| "rules"
	| "lessons"
	| "skills"
	| "hooks"
	| "settings"
	| "configure"
	| "milestones"
	| "epics"
	| "tasks"
	| "ideas"
	| "decisions"
	| "orchestrator";

export type ActivityGroup = "documentation" | "planning" | "team" | "governance";

export type ExplorerView =
	| "artifact-list"
	| "artifact-viewer"
	| "project-dashboard"
	| "settings";

/** Sub-categories within each group */
export const GROUP_SUB_CATEGORIES: Record<ActivityGroup, ActivityView[]> = {
	documentation: ["docs"],
	planning: ["ideas", "research", "plans", "milestones", "epics", "tasks"],
	team: ["orchestrator", "agents", "skills"],
	governance: ["lessons", "decisions", "rules", "hooks"],
};

/** Sub-category display config */
export interface SubCategoryConfig {
	key: ActivityView;
	label: string;
}

export const SUB_CATEGORY_LABELS: Record<ActivityView, string> = {
	chat: "Chat",
	project: "Project",
	docs: "Docs",
	research: "Research",
	plans: "Plans",
	agents: "Agents",
	rules: "Rules",
	lessons: "Lessons",
	skills: "Skills",
	hooks: "Hooks",
	settings: "Settings",
	configure: "Configuration",
	milestones: "Milestones",
	epics: "Epics",
	tasks: "Tasks",
	ideas: "Ideas",
	decisions: "Decisions",
	orchestrator: "Orchestrator",
};

/** Activity views that use the nav sub-panel for sub-navigation */
const ACTIVITIES_WITH_NAV_PANEL: ActivityView[] = [
	"docs",
	"research",
	"plans",
	"agents",
	"rules",
	"skills",
	"hooks",
	"settings",
	"configure",
	"chat",
	"milestones",
	"epics",
	"tasks",
	"ideas",
	"decisions",
	"orchestrator",
	"lessons",
];

/** Activity views that show artifact browsing in the explorer */
const ARTIFACT_ACTIVITIES: ActivityView[] = ["docs", "research", "plans", "agents", "rules", "skills", "hooks"];

/** Sub-categories that have no backend reader yet (show EmptyState) */
export const COMING_SOON_ACTIVITIES: ActivityView[] = [];

/** Maps artifact ID prefixes to their group and sub-category */
const ARTIFACT_PREFIX_MAP: Record<string, { group: ActivityGroup; subCategory: ActivityView }> = {
	MS: { group: "planning", subCategory: "milestones" },
	EPIC: { group: "planning", subCategory: "epics" },
	TASK: { group: "planning", subCategory: "tasks" },
	IDEA: { group: "planning", subCategory: "ideas" },
	AD: { group: "governance", subCategory: "decisions" },
	IMPL: { group: "governance", subCategory: "lessons" },
};

class NavigationStore {
	activeActivity = $state<ActivityView>("chat");
	activeGroup = $state<ActivityGroup | null>(null);
	activeSubCategory = $state<ActivityView | null>(null);
	explorerView = $state<ExplorerView>("artifact-list");
	selectedArtifactPath = $state<string | null>(null);
	navPanelCollapsed = $state(false);
	breadcrumbs = $state<string[]>([]);
	/** Pending artifact ID to auto-select after navigating to a sub-category via cross-link. */
	pendingArtifactId = $state<string | null>(null);

	get showNavPanel(): boolean {
		if (this.navPanelCollapsed) return false;
		// If a group is active, always show nav panel
		if (this.activeGroup !== null) return true;
		return ACTIVITIES_WITH_NAV_PANEL.includes(this.activeActivity);
	}

	get isArtifactActivity(): boolean {
		return ARTIFACT_ACTIVITIES.includes(this.activeActivity);
	}

	setGroup(group: ActivityGroup) {
		this.activeGroup = group;
		const subCategories = GROUP_SUB_CATEGORIES[group];
		const firstSub = subCategories[0];
		this.setSubCategory(firstSub);
	}

	setSubCategory(key: ActivityView) {
		this.activeSubCategory = key;
		this.setActivity(key);
	}

	setActivity(view: ActivityView) {
		this.activeActivity = view;
		this.selectedArtifactPath = null;
		this.breadcrumbs = [];

		if (view === "project") {
			this.activeGroup = null;
			this.activeSubCategory = null;
			this.explorerView = "project-dashboard";
			this.navPanelCollapsed = true;
		} else if (view === "settings" || view === "configure") {
			this.explorerView = "settings";
			if (this.navPanelCollapsed) {
				this.navPanelCollapsed = false;
			}
		} else if (view === "docs") {
			this.explorerView = "artifact-viewer";
			this.selectedArtifactPath = "README";
			this.breadcrumbs = [];
			if (this.navPanelCollapsed) {
				this.navPanelCollapsed = false;
			}
		} else if (view === "research") {
			this.explorerView = "artifact-viewer";
			this.selectedArtifactPath = "README";
			this.breadcrumbs = [];
			if (this.navPanelCollapsed) {
				this.navPanelCollapsed = false;
			}
		} else if (view === "plans") {
			this.explorerView = "artifact-viewer";
			this.selectedArtifactPath = "README";
			this.breadcrumbs = [];
			if (this.navPanelCollapsed) {
				this.navPanelCollapsed = false;
			}
		} else if (view === "orchestrator") {
			// Load the orchestrator agent file directly
			this.explorerView = "artifact-viewer";
			this.selectedArtifactPath = "orchestrator";
			this.breadcrumbs = ["Orchestrator"];
			if (this.navPanelCollapsed) {
				this.navPanelCollapsed = false;
			}
		} else if (view === "milestones" || view === "epics" || view === "tasks" || view === "ideas") {
			// Planning artifacts: open with README as entry point, same as docs/research/plans
			this.explorerView = "artifact-viewer";
			this.selectedArtifactPath = "README";
			this.breadcrumbs = [];
			if (this.navPanelCollapsed) {
				this.navPanelCollapsed = false;
			}
		} else if (view === "decisions" || view === "lessons") {
			// Governance artifacts: show list in sidebar, nothing selected until user clicks
			this.explorerView = "artifact-list";
			if (this.navPanelCollapsed) {
				this.navPanelCollapsed = false;
			}
		} else if (ARTIFACT_ACTIVITIES.includes(view)) {
			this.explorerView = "artifact-list";
			if (this.navPanelCollapsed) {
				this.navPanelCollapsed = false;
			}
		} else {
			if (this.navPanelCollapsed) {
				this.navPanelCollapsed = false;
			}
		}
	}

	openArtifact(path: string, breadcrumbs: string[]) {
		this.selectedArtifactPath = path;
		this.explorerView = "artifact-viewer";
		this.breadcrumbs = breadcrumbs;
	}

	closeArtifact() {
		this.selectedArtifactPath = null;
		this.explorerView = "artifact-list";
		this.breadcrumbs = [];
	}

	/**
	 * Navigate to an artifact by its ID string (e.g. "EPIC-005", "MS-001", "AD-017").
	 * Resolves the prefix to the correct group and sub-category, then opens the artifact.
	 */
	navigateToArtifact(id: string) {
		const prefix = id.split("-")[0];
		const target = ARTIFACT_PREFIX_MAP[prefix];
		if (!target) return;
		this.activeGroup = target.group;
		this.setSubCategory(target.subCategory);
		// The artifact list will be loaded by AppLayout's $effect.
		// We store the pending ID so the list can auto-select it once loaded.
		this.pendingArtifactId = id;
	}

	toggleNavPanel() {
		this.navPanelCollapsed = !this.navPanelCollapsed;
	}
}

export const navigationStore = new NavigationStore();
