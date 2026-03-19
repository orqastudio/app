/**
 * Navigation state management.
 *
 * Cross-store references (projectStore, artifactStore, artifactGraphSDK) are
 * resolved lazily via `getStores()` so there are no circular import issues.
 * By the time any method or getter is called, `initializeStores()` has
 * already run.
 *
 * Supports both the new navigation model (project.json `navigation` array)
 * and the legacy `artifacts` config. When `navigation` is present, the new
 * model takes precedence.
 */

import { getStores } from "../registry.svelte.js";
import type { NavDocNode, NavType, NavigationItem } from "@orqastudio/types";
import { parseHash, pushRoute, currentRoute, type ParsedRoute } from "../router.js";
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
	| "plugin-view"
	| "settings";

/** Sub-category display config. */
export interface SubCategoryConfig {
	key: string;
	label: string;
	icon?: string;
	type?: NavigationItem["type"];
	pluginSource?: string;
}

/** The resolved active navigation item for routing. */
export interface ActiveNavItem {
	key: string;
	type: NavigationItem["type"];
	icon: string;
	label?: string;
	pluginSource?: string;
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

	/** The currently active navigation item for routing. */
	activeNavItem = $state<ActiveNavItem | null>(null);

	/** Whether we're currently applying a route from hashchange (prevents loops). */
	private _applyingRoute = false;

	// -----------------------------------------------------------------------
	// Hash Router Integration
	// -----------------------------------------------------------------------

	/**
	 * Initialize the hash router. Call once after stores are ready.
	 * Reads the current hash to restore state and listens for hashchange.
	 */
	initRouter(): void {
		// Restore state from current hash (survives hot reload)
		const route = currentRoute();
		if (route.type !== "default") {
			this.applyRoute(route);
		}

		// Listen for back/forward
		window.addEventListener("hashchange", () => {
			const newRoute = currentRoute();
			this.applyRoute(newRoute);
		});
	}

	/**
	 * Apply a parsed route to navigation state.
	 * Called from hashchange listener and on init.
	 */
	private applyRoute(route: ParsedRoute): void {
		this._applyingRoute = true;
		try {
			switch (route.type) {
				case "project":
					this.setActivity("project");
					break;
				case "settings":
					this.setActivity("settings");
					break;
				case "graph":
					this.setActivity("artifact-graph");
					break;
				case "plugin":
					if (route.pluginName && route.viewKey) {
						this.setActivity(route.viewKey);
					}
					break;
				case "artifact":
					if (route.activity) this.setActivity(route.activity);
					if (route.artifactPath) this.openArtifact(route.artifactPath, []);
					break;
				case "artifacts":
					if (route.activity) this.setActivity(route.activity);
					break;
				default:
					this.setActivity("chat");
					break;
			}
		} finally {
			this._applyingRoute = false;
		}
	}

	/**
	 * Push the current navigation state to the URL hash.
	 * Skipped when applying a route from hashchange (prevents loops).
	 */
	private syncToHash(): void {
		if (this._applyingRoute) return;

		if (this.activeNavItem?.type === "plugin" && this.activeNavItem.pluginSource) {
			pushRoute({ type: "plugin", pluginName: this.activeNavItem.pluginSource, viewKey: this.activeNavItem.key });
		} else if (this.selectedArtifactPath) {
			pushRoute({ type: "artifact", activity: this.activeActivity, artifactPath: this.selectedArtifactPath });
		} else if (this.activeActivity === "project") {
			pushRoute({ type: "project" });
		} else if (this.activeActivity === "settings" || this.activeActivity === "configure") {
			pushRoute({ type: "settings" });
		} else if (this.activeActivity === "artifact-graph") {
			pushRoute({ type: "graph" });
		} else if (this.activeActivity === "chat") {
			pushRoute({ type: "default" });
		} else {
			pushRoute({ type: "artifacts", activity: this.activeActivity });
		}
	}

	// -----------------------------------------------------------------------
	// Navigation tree accessors (new model)
	// -----------------------------------------------------------------------

	/** The navigation tree — from project.json or null for legacy projects. */
	private get _navTree(): NavigationItem[] | null {
		const { projectStore } = getStores();
		return projectStore.navigation;
	}

	/** Whether the project uses the new navigation model. */
	private get _useNewNav(): boolean {
		return this._navTree !== null;
	}

	/**
	 * Find a NavigationItem by key in the navigation tree (recursive).
	 */
	findNavItem(key: string): NavigationItem | null {
		const tree = this._navTree;
		if (!tree) return null;
		return this._findInNavTree(tree, key);
	}

	private _findInNavTree(items: NavigationItem[], key: string): NavigationItem | null {
		for (const item of items) {
			if (item.key === key) return item;
			if (item.children) {
				const found = this._findInNavTree(item.children, key);
				if (found) return found;
			}
		}
		return null;
	}

	/**
	 * Find the parent group for a given nav item key.
	 */
	private _findParentGroup(key: string): NavigationItem | null {
		const tree = this._navTree;
		if (!tree) return null;
		for (const item of tree) {
			if (item.type === "group" && item.children) {
				for (const child of item.children) {
					if (child.key === key) return item;
				}
			}
		}
		return null;
	}

	// -----------------------------------------------------------------------
	// Getters (work with both old and new nav model)
	// -----------------------------------------------------------------------

	/** Flat list of all artifact type keys from config (groups expanded to their children). */
	get allArtifactKeys(): string[] {
		if (this._useNewNav) {
			return this._allNavLeafKeys();
		}
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

	/** Get all leaf keys from the navigation tree. */
	private _allNavLeafKeys(): string[] {
		const tree = this._navTree;
		if (!tree) return [];
		const keys: string[] = [];
		const collect = (items: NavigationItem[]) => {
			for (const item of items) {
				if (item.type === "group" && item.children) {
					collect(item.children);
				} else if (item.type !== "group") {
					keys.push(item.key);
				}
			}
		};
		collect(tree);
		return keys;
	}

	/** Keys of entries that are groups (have children). */
	get groupKeys(): string[] {
		if (this._useNewNav) {
			const tree = this._navTree!;
			return tree.filter((i) => i.type === "group").map((i) => i.key);
		}
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
		// In new nav model, check if the active nav item is a builtin artifact list
		if (this._useNewNav && this.activeNavItem) {
			const builtinViews = ["project", "artifact-graph", "settings", "configure", "chat"];
			if (builtinViews.includes(this.activeNavItem.key)) return false;
			// Plugin views are not artifact activities
			if (this.activeNavItem.type === "plugin") return false;
			// Builtin items that aren't special views are artifact lists
			return this.activeNavItem.type === "builtin";
		}
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
	 * Get label for a given key from the navigation tree or artifact config.
	 * Falls back to humanized key.
	 */
	getLabelForKey(key: string): string {
		if (this._useNewNav) {
			const item = this.findNavItem(key);
			if (item?.label) return item.label;
			// Try plugin registry
			const { pluginRegistry } = getStores();
			if (item?.type === "plugin" && item.pluginSource) {
				const resolved = pluginRegistry.resolveNavigationItem(item);
				return resolved.label;
			}
			return humanizeKey(key);
		}
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
		if (this._useNewNav) {
			const tree = this._navTree!;
			const group = tree.find((i) => i.key === groupKey && i.type === "group");
			if (!group?.children) return [];
			return group.children
				.filter((c) => !c.hidden)
				.map((c) => ({
					key: c.key,
					label: c.label ?? humanizeKey(c.key),
					icon: c.icon,
					type: c.type,
					pluginSource: c.pluginSource,
				}));
		}
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
		if (this._useNewNav) {
			const tree = this._navTree!;
			const result: Record<string, string[]> = {};
			for (const item of tree) {
				if (item.type === "group" && item.children) {
					result[item.key] = item.children.filter((c) => !c.hidden).map((c) => c.key);
				}
			}
			return result;
		}
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
	 * Get top-level navigation items for the ActivityBar.
	 * In new nav model, returns the navigation tree items.
	 * In legacy mode, returns null (ActivityBar uses artifactConfig).
	 */
	get topLevelNavItems(): NavigationItem[] | null {
		return this._navTree;
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
		// In new nav model, check plugin schemas for paths
		if (this._useNewNav) {
			const { pluginRegistry } = getStores();
			const schema = pluginRegistry.getSchema(key);
			if (schema) return schema.defaultPath;
		}
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

	// -----------------------------------------------------------------------
	// Navigation actions
	// -----------------------------------------------------------------------

	setGroup(group: string) {
		this.activeGroup = group;
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

		// Update activeNavItem for routing
		if (this._useNewNav) {
			const navItem = this.findNavItem(view);
			if (navItem && navItem.type !== "group") {
				this.activeNavItem = {
					key: navItem.key,
					type: navItem.type,
					icon: navItem.icon,
					label: navItem.label,
					pluginSource: navItem.pluginSource,
				};
			} else {
				// Built-in views not in the tree (chat, configure, etc.)
				this.activeNavItem = {
					key: view,
					type: "builtin",
					icon: "circle",
				};
			}
		}

		if (view === "project") {
			this.activeGroup = null;
			this.activeSubCategory = null;
			this.explorerView = "project-dashboard";
			this.navPanelCollapsed = true;
		} else if (view === "artifact-graph") {
			this.activeGroup = null;
			this.activeSubCategory = null;
			this.navPanelCollapsed = true;
		} else if (view === "settings" || view === "configure") {
			this.activeGroup = null;
			this.activeSubCategory = null;
			this.explorerView = "settings";
			if (this.navPanelCollapsed) {
				this.navPanelCollapsed = false;
			}
		} else if (this._useNewNav && this.activeNavItem?.type === "plugin") {
			this.explorerView = "plugin-view";
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

		this.syncToHash();
	}

	openArtifact(path: string, breadcrumbs: string[]) {
		this.selectedArtifactPath = path;
		this.explorerView = "artifact-viewer";
		this.breadcrumbs = breadcrumbs;
		this.syncToHash();
	}

	closeArtifact() {
		this.selectedArtifactPath = null;
		this.explorerView = "artifact-list";
		this.breadcrumbs = [];
		this.syncToHash();
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
		// In new nav model, also check plugin schemas
		if (this._useNewNav) {
			const { pluginRegistry } = getStores();
			const normalized = navTypePath.replace(/\\/g, "/").replace(/\/$/, "");
			for (const schema of pluginRegistry.allSchemas) {
				if (schema.defaultPath.replace(/\\/g, "/").replace(/\/$/, "") === normalized) {
					return schema.key;
				}
			}
		}
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
		// In new nav model, check navigation tree
		if (this._useNewNav) {
			const normalized = navTypePath.replace(/\\/g, "/").replace(/\/$/, "");
			const { pluginRegistry } = getStores();
			// Find which schema matches this path, then find its parent group in nav tree
			for (const schema of pluginRegistry.allSchemas) {
				if (schema.defaultPath.replace(/\\/g, "/").replace(/\/$/, "") === normalized) {
					const parent = this._findParentGroup(schema.key);
					if (parent) return parent.key;
				}
			}
		}
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
