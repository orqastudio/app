/**
 * Navigation state management.
 *
 * Cross-store references (projectStore, artifactStore, artifactGraphSDK) are
 * resolved lazily via `getStores()` so there are no circular import issues.
 * By the time any method or getter is called, `initializeStores()` has
 * already run.
 */

import { getStores } from "../registry.svelte.js";
import type { NavDocNode, NavType } from "@orqastudio/types";
import { isArtifactGroup } from "@orqastudio/types";

/**
 * Convert a config key to a human-readable label.
 * Replaces hyphens and underscores with spaces and title-cases each word.
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

/** ActivityGroup is now a string since group keys come from config. */
export type ActivityGroup = string;

export type ExplorerView =
	| "artifact-list"
	| "artifact-viewer"
	| "project-dashboard"
	| "roadmap"
	| "settings";

/** Sub-category display config. */
export interface SubCategoryConfig {
	key: string;
	label: string;
}

export class NavigationStore {
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
		const { projectStore } = getStores();
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
		const { projectStore } = getStores();
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
		if (this.activeGroup !== null) return true;
		if (this.activeActivity === "chat") return true;
		if (this.activeActivity === "settings") return true;
		if (this.activeActivity === "configure") return true;
		if (this.isArtifactActivity) return true;
		return false;
	}

	/**
	 * Get label for a given key from the artifact config.
	 * Falls back to humanized key.
	 */
	getLabelForKey(key: string): string {
		const { projectStore } = getStores();
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

	/** Sub-categories (children) for a given group key. */
	getGroupChildren(groupKey: string): SubCategoryConfig[] {
		const { projectStore } = getStores();
		const config = projectStore.artifactConfig;
		for (const entry of config) {
			if (isArtifactGroup(entry) && entry.key === groupKey) {
				return entry.children.map((c) => ({ key: c.key, label: c.label ?? humanizeKey(c.key) }));
			}
		}
		return [];
	}

	/** All group sub-categories, keyed by group key. */
	get groupSubCategories(): Record<string, string[]> {
		const { projectStore } = getStores();
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
	 */
	getNavType(view: string) {
		const { artifactStore } = getStores();
		const tree = artifactStore.navTree;
		if (!tree) return null;

		const configPath = this.getConfiguredPath(view);

		for (const group of tree.groups) {
			for (const type of group.types) {
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

	/** Return the configured `path` for the given artifact key, or null if not found. */
	getConfiguredPath(key: string): string | null {
		const { projectStore } = getStores();
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
		if (group === "delivery") {
			this.setSubCategory("roadmap");
			return;
		}
		const children = this.getGroupChildren(group);
		const first = children[0];
		if (first) {
			this.setSubCategory(first.key);
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
		} else if (view === "artifact-graph") {
			this.activeGroup = null;
			this.activeSubCategory = null;
			this.navPanelCollapsed = true;
		} else if (view === "roadmap") {
			if (this.activeGroup === "delivery") {
				this.explorerView = "roadmap";
				if (this.navPanelCollapsed) {
					this.navPanelCollapsed = false;
				}
			} else {
				this.activeGroup = null;
				this.activeSubCategory = null;
				this.explorerView = "roadmap";
				this.navPanelCollapsed = true;
			}
		} else if (view === "settings" || view === "configure") {
			this.activeGroup = null;
			this.activeSubCategory = null;
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

	/** Navigate to an artifact by its ID string (e.g. "EPIC-005", "MS-001"). */
	navigateToArtifact(id: string) {
		const { artifactGraphSDK } = getStores();
		const node = artifactGraphSDK.resolve(id);
		if (!node) {
			console.warn(`[navigateToArtifact] could not resolve artifact ID: ${id}`);
			return;
		}
		this.navigateToPath(node.path);
	}

	/** Navigate to an artifact by its relative file path. */
	navigateToPath(path: string) {
		const { artifactStore } = getStores();
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
					const typeKey = this._resolveKeyForNavTypePath(navType.path);
					if (!typeKey) {
						console.warn(`[navigateToPath] no config key for NavType path: ${navType.path}`);
						return;
					}

					const groupKey = this._resolveGroupKeyForNavTypePath(navType.path);

					if (groupKey) {
						this.activeGroup = groupKey;
						this.activeSubCategory = typeKey;
					} else {
						this.activeGroup = null;
						this.activeSubCategory = null;
					}

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

	private _findNodeInNavType(navType: NavType, normalizedPath: string): NavDocNode | null {
		return this._findNodeInList(navType.nodes, normalizedPath);
	}

	private _findNodeInList(nodes: NavDocNode[], normalizedPath: string): NavDocNode | null {
		for (const node of nodes) {
			if (node.children) {
				const found = this._findNodeInList(node.children, normalizedPath);
				if (found) return found;
			} else if (node.path) {
				const np = node.path.replace(/\\/g, "/");
				if (np === normalizedPath || `${np}.md` === normalizedPath || np === normalizedPath.replace(/\.md$/, "")) {
					return node;
				}
			}
		}
		return null;
	}

	private _resolveKeyForNavTypePath(navTypePath: string): string | null {
		const { projectStore } = getStores();
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

	private _resolveGroupKeyForNavTypePath(navTypePath: string): string | null {
		const { projectStore } = getStores();
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
