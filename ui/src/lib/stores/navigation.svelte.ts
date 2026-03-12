import { artifactStore } from "$lib/stores/artifact.svelte";
import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
import { projectStore } from "$lib/stores/project.svelte";
import { isArtifactGroup } from "$lib/types/project";
import type { DocNode, NavType } from "$lib/types/nav-tree";

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


class NavigationStore {
	activeActivity = $state<string>("chat");
	activeGroup = $state<string | null>(null);
	activeSubCategory = $state<string | null>(null);
	explorerView = $state<ExplorerView>("artifact-list");
	selectedArtifactPath = $state<string | null>(null);
	navPanelCollapsed = $state(false);
	breadcrumbs = $state<string[]>([]);
	searchOverlayOpen = $state(false);

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
	 * Uses the SDK to resolve the ID to a path, then calls navigateToPath.
	 */
	navigateToArtifact(id: string) {
		const node = artifactGraphSDK.resolve(id);
		if (!node) {
			console.warn(`[navigateToArtifact] could not resolve artifact ID: ${id}`);
			return;
		}
		this.navigateToPath(node.path);
	}

	/**
	 * Navigate to an artifact by its relative file path
	 * (e.g. ".orqa/planning/epics/EPIC-048.md").
	 *
	 * Walks the full NavTree — including tree children — to find the node
	 * that matches the path, then sets the activity, group, and sub-category
	 * before opening the artifact viewer.
	 */
	navigateToPath(path: string) {
		const tree = artifactStore.navTree;
		if (!tree) {
			console.warn(`[navigateToPath] navTree not yet loaded, cannot navigate to: ${path}`);
			return;
		}

		const normalizedPath = path.replace(/\\/g, "/");

		for (const group of tree.groups) {
			for (const navType of group.types) {
				const found = this._findNodeInNavType(navType, normalizedPath);
				if (found) {
					// Determine the config key for this NavType by matching its path
					const typeKey = this._resolveKeyForNavTypePath(navType.path);
					if (!typeKey) {
						console.warn(`[navigateToPath] no config key for NavType path: ${navType.path}`);
						return;
					}

					// Determine the group key — use the group key from config that matches
					const groupKey = this._resolveGroupKeyForNavTypePath(navType.path);

					if (groupKey) {
						this.activeGroup = groupKey;
						this.activeSubCategory = typeKey;
					} else {
						this.activeGroup = null;
						this.activeSubCategory = null;
					}

					// Set the activity to the type key and open the artifact
					this.activeActivity = typeKey;
					this.explorerView = "artifact-viewer";
					if (this.navPanelCollapsed) this.navPanelCollapsed = false;
					this.selectedArtifactPath = found.path;
					this.breadcrumbs = [found.label];
					return;
				}
			}
		}

		console.warn(`[navigateToPath] no NavTree node found for path: ${path}`);
	}

	/**
	 * Recursively search a NavType's nodes (including tree children) for a node
	 * whose path matches the given normalised path. Returns the matching leaf DocNode
	 * or null if not found.
	 */
	private _findNodeInNavType(navType: NavType, normalizedPath: string): DocNode | null {
		return this._findNodeInList(navType.nodes, normalizedPath);
	}

	private _findNodeInList(nodes: DocNode[], normalizedPath: string): DocNode | null {
		for (const node of nodes) {
			if (node.children) {
				const found = this._findNodeInList(node.children, normalizedPath);
				if (found) return found;
			} else if (node.path) {
				const np = node.path.replace(/\\/g, "/");
				// NavTree paths may omit the .md extension; match both forms
				if (np === normalizedPath || `${np}.md` === normalizedPath || np === normalizedPath.replace(/\.md$/, "")) {
					return node;
				}
			}
		}
		return null;
	}

	/**
	 * Given a NavType's directory path, find the config key that maps to it.
	 * Searches both direct-type entries and group children.
	 */
	private _resolveKeyForNavTypePath(navTypePath: string): string | null {
		const config = projectStore.artifactConfig;
		const normalized = navTypePath.replace(/\\/g, "/").replace(/\/$/, "");
		for (const entry of config) {
			if (isArtifactGroup(entry)) {
				for (const child of entry.children) {
					if (child.path && child.path.replace(/\\/g, "/").replace(/\/$/, "") === normalized) {
						return child.key;
					}
				}
			} else {
				if (entry.path && entry.path.replace(/\\/g, "/").replace(/\/$/, "") === normalized) {
					return entry.key;
				}
			}
		}
		return null;
	}

	/**
	 * Given a NavType's directory path, find the group key that contains it,
	 * or null if it is a top-level (non-grouped) type.
	 */
	private _resolveGroupKeyForNavTypePath(navTypePath: string): string | null {
		const config = projectStore.artifactConfig;
		const normalized = navTypePath.replace(/\\/g, "/").replace(/\/$/, "");
		for (const entry of config) {
			if (isArtifactGroup(entry)) {
				for (const child of entry.children) {
					if (child.path && child.path.replace(/\\/g, "/").replace(/\/$/, "") === normalized) {
						return entry.key;
					}
				}
			}
		}
		return null;
	}

	toggleNavPanel() {
		this.navPanelCollapsed = !this.navPanelCollapsed;
	}

	toggleSearch() {
		this.searchOverlayOpen = !this.searchOverlayOpen;
	}
}

export const navigationStore = new NavigationStore();
