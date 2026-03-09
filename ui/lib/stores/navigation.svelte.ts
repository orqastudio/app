import { artifactStore } from "$lib/stores/artifact.svelte";
import { projectStore } from "$lib/stores/project.svelte";
import { isArtifactGroup } from "$lib/types/project";

/**
 * Convert a config key to a human-readable label.
 * Replaces hyphens and underscores with spaces and title-cases each word.
 * Mirrors the Rust `humanize_name` logic for the frontend fallback path.
 */
function humanizeKey(key: string): string {
	return key
		.replace(/[-_]/g, " ")
		.replace(/\b\w/g, (c) => c.toUpperCase());
}

/**
 * ActivityView is now a string since artifact type keys come from config.
 * Built-in non-artifact views are: "chat", "project", "settings", "configure".
 */
export type ActivityView = string;

/**
 * ActivityGroup is now a string since group keys come from config.
 */
export type ActivityGroup = string;

export type ExplorerView =
	| "artifact-list"
	| "artifact-viewer"
	| "project-dashboard"
	| "settings";

/** Sub-category display config */
export interface SubCategoryConfig {
	key: string;
	label: string;
}

/** Maps artifact ID prefixes to their group and sub-category for cross-link navigation. */
const ARTIFACT_PREFIX_MAP: Record<string, { group: string; subCategory: string }> = {
	MS: { group: "planning", subCategory: "milestones" },
	EPIC: { group: "planning", subCategory: "epics" },
	TASK: { group: "planning", subCategory: "tasks" },
	IDEA: { group: "planning", subCategory: "ideas" },
	AD: { group: "governance", subCategory: "decisions" },
	IMPL: { group: "governance", subCategory: "lessons" },
};

class NavigationStore {
	activeActivity = $state<string>("chat");
	activeGroup = $state<string | null>(null);
	activeSubCategory = $state<string | null>(null);
	explorerView = $state<ExplorerView>("artifact-list");
	selectedArtifactPath = $state<string | null>(null);
	navPanelCollapsed = $state(false);
	breadcrumbs = $state<string[]>([]);
	/** Pending artifact ID to auto-select after navigating to a sub-category via cross-link. */
	pendingArtifactId = $state<string | null>(null);

	/** Flat list of all artifact type keys from config (groups expanded to their children). */
	get allArtifactKeys(): string[] {
		const config = projectStore.artifactConfig;
		const keys: string[] = [];
		for (const entry of config) {
			if (isArtifactGroup(entry)) {
				for (const child of entry.children) {
					keys.push(child.key);
				}
			} else {
				keys.push(entry.key);
			}
		}
		return keys;
	}

	/** Keys of entries that are groups (have children). */
	get groupKeys(): string[] {
		return projectStore.artifactConfig
			.filter(isArtifactGroup)
			.map((e) => e.key);
	}

	/** Whether the given key is a group key. */
	isGroupKey(key: string): boolean {
		return this.groupKeys.includes(key);
	}

	/** Whether the current activity is an artifact activity (not a built-in view). */
	get isArtifactActivity(): boolean {
		return this.allArtifactKeys.includes(this.activeActivity);
	}

	/** Whether any view should show the nav panel. */
	get showNavPanel(): boolean {
		if (this.navPanelCollapsed) return false;
		// If a group is active, always show nav panel
		if (this.activeGroup !== null) return true;
		// Built-in views that use the nav panel
		if (this.activeActivity === "chat") return true;
		if (this.activeActivity === "settings") return true;
		if (this.activeActivity === "configure") return true;
		// Any artifact activity shows the nav panel
		if (this.isArtifactActivity) return true;
		return false;
	}

	/** Get label for a given key from the artifact config.
	 *
	 * Falls back to the navTree label (sourced from README frontmatter) when the
	 * config entry has no explicit label. Humanizes the key as a last resort.
	 */
	getLabelForKey(key: string): string {
		const config = projectStore.artifactConfig;
		for (const entry of config) {
			if (entry.key === key) return entry.label ?? humanizeKey(key);
			if (isArtifactGroup(entry)) {
				for (const child of entry.children) {
					if (child.key === key) return child.label ?? humanizeKey(child.key);
				}
			}
		}
		return humanizeKey(key);
	}

	/**
	 * Sub-categories (children) for a given group key.
	 * Derived from the artifact config.
	 */
	getGroupChildren(groupKey: string): SubCategoryConfig[] {
		const config = projectStore.artifactConfig;
		for (const entry of config) {
			if (isArtifactGroup(entry) && entry.key === groupKey) {
				return entry.children.map((c) => ({ key: c.key, label: c.label ?? humanizeKey(c.key) }));
			}
		}
		return [];
	}

	/**
	 * All group sub-categories, keyed by group key.
	 * Derived from config. Kept for compatibility with components that iterate groups.
	 */
	get groupSubCategories(): Record<string, string[]> {
		const config = projectStore.artifactConfig;
		const result: Record<string, string[]> = {};
		for (const entry of config) {
			if (isArtifactGroup(entry)) {
				result[entry.key] = entry.children.map((c) => c.key);
			}
		}
		return result;
	}

	/**
	 * Find the NavType for the given activity string, if the navTree has loaded.
	 * Returns null if the navTree is not yet available or the type is not found.
	 *
	 * Matching strategy: look up the configured path for the view key, then match
	 * NavType entries by path. This handles both group children (where the key is
	 * the last path segment) and direct-type entries (where the key may differ from
	 * the last path segment, e.g. key="docs" but path=".orqa/documentation").
	 */
	getNavType(view: string) {
		const tree = artifactStore.navTree;
		if (!tree) return null;

		// Resolve the configured path for this view key.
		const configPath = this.getConfiguredPath(view);

		for (const group of tree.groups) {
			for (const type of group.types) {
				// Match by configured path when available; fall back to matching
				// the last path segment (legacy behaviour for group children).
				if (configPath !== null) {
					if (type.path === configPath) return type;
				} else {
					const typeKey = type.path.split("/").pop();
					if (typeKey === view) return type;
				}
			}
		}
		return null;
	}

	/**
	 * Return the configured `path` for the given artifact key, or null if not found.
	 * Searches both direct-type entries and group children.
	 */
	getConfiguredPath(key: string): string | null {
		const config = projectStore.artifactConfig;
		for (const entry of config) {
			if (isArtifactGroup(entry)) {
				for (const child of entry.children) {
					if (child.key === key) return child.path;
				}
			} else {
				if (entry.key === key) return entry.path;
			}
		}
		return null;
	}

	setGroup(group: string) {
		this.activeGroup = group;
		const children = this.getGroupChildren(group);
		if (children.length > 0) {
			this.setSubCategory(children[0].key);
		}
	}

	setSubCategory(key: string) {
		this.activeSubCategory = key;
		this.setActivity(key);
	}

	setActivity(view: string) {
		this.activeActivity = view;
		this.selectedArtifactPath = null;
		this.breadcrumbs = [];
		// Clear any pending cross-link ID that was not consumed. This prevents a
		// stale ID from auto-selecting an unrelated artifact on a future navigation.
		this.pendingArtifactId = null;

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
		} else if (this.isArtifactActivity) {
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
