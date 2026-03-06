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
	| "configure";

export type ExplorerView =
	| "artifact-list"
	| "artifact-viewer"
	| "project-dashboard"
	| "settings";

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
];

/** Activity views that show artifact browsing in the explorer */
const ARTIFACT_ACTIVITIES: ActivityView[] = ["docs", "research", "plans", "agents", "rules", "skills", "hooks"];

class NavigationStore {
	activeActivity = $state<ActivityView>("chat");
	explorerView = $state<ExplorerView>("artifact-list");
	selectedArtifactPath = $state<string | null>(null);
	navPanelCollapsed = $state(false);
	breadcrumbs = $state<string[]>([]);

	get showNavPanel(): boolean {
		if (this.navPanelCollapsed) return false;
		return ACTIVITIES_WITH_NAV_PANEL.includes(this.activeActivity);
	}

	get isArtifactActivity(): boolean {
		return ARTIFACT_ACTIVITIES.includes(this.activeActivity);
	}

	setActivity(view: ActivityView) {
		this.activeActivity = view;
		this.selectedArtifactPath = null;
		this.breadcrumbs = [];

		if (view === "project") {
			this.explorerView = "project-dashboard";
			this.navPanelCollapsed = true;
		} else if (view === "settings" || view === "configure") {
			this.explorerView = "settings";
			if (this.navPanelCollapsed) {
				this.navPanelCollapsed = false;
			}
		} else if (view === "docs") {
			// Auto-open the docs homepage (no breadcrumb — home icon suffices)
			this.explorerView = "artifact-viewer";
			this.selectedArtifactPath = "README";
			this.breadcrumbs = [];
			if (this.navPanelCollapsed) {
				this.navPanelCollapsed = false;
			}
		} else if (view === "research") {
			// Auto-open the research index
			this.explorerView = "artifact-viewer";
			this.selectedArtifactPath = "README";
			this.breadcrumbs = [];
			if (this.navPanelCollapsed) {
				this.navPanelCollapsed = false;
			}
		} else if (view === "plans") {
			// Auto-open the plans index
			this.explorerView = "artifact-viewer";
			this.selectedArtifactPath = "README";
			this.breadcrumbs = [];
			if (this.navPanelCollapsed) {
				this.navPanelCollapsed = false;
			}
		} else if (view === "lessons") {
			// This view manages its own internal panel — collapse the nav sub-panel.
			this.navPanelCollapsed = true;
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

	toggleNavPanel() {
		this.navPanelCollapsed = !this.navPanelCollapsed;
	}
}

export const navigationStore = new NavigationStore();
