/**
 * Navigation state management.
 *
 * Cross-store references (projectStore, artifactStore, artifactGraphSDK) are
 * resolved lazily via `getStores()` so there are no circular import issues.
 * By the time any method or getter is called, `initializeStores()` has
 * already run.
 *
 * Navigation tree priority:
 * 1. Explicit `navigation` array from project.json (full custom tree).
 * 2. PLATFORM_NAVIGATION merged with plugin defaultNavigation (default for
 *    projects without explicit navigation config).
 * Path resolution uses plugin schemas exclusively — no fallback to legacy config.
 */

import { SvelteSet } from "svelte/reactivity";
import { getStores } from "../registry.svelte.js";
import type { NavDocNode, NavType, NavigationItem } from "@orqastudio/types";
import { PLATFORM_NAVIGATION } from "@orqastudio/types";
import { pushRoute, currentRoute, type ParsedRoute } from "../router.js";
import { logger } from "../logger.js";

const log = logger("navigation");

/**
 * Convert a config key to a human-readable label.
 * Replaces hyphens and underscores with spaces and title-cases each word.
 * @param key - Raw config key (e.g. "my-artifact-type").
 * @returns Title-cased label (e.g. "My Artifact Type").
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

/** Manages all navigation state: active views, artifact selection, breadcrumbs, and URL hash sync. */
export class NavigationStore {
	/** Key of the currently active activity (e.g. "chat", "tasks"). */
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

		// Listen for back/forward — popstate fires on history navigation,
		// not on programmatic pushState (which avoids the double-fire loop)
		window.addEventListener("popstate", () => {
			const newRoute = currentRoute();
			this.applyRoute(newRoute);
		});
	}

	/**
	 * Apply a parsed route to navigation state.
	 * Called from hashchange listener and on init.
	 * @param route - The parsed route to apply.
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

	/**
	 * The navigation tree.
	 *
	 * Priority:
	 * 1. Explicit `navigation` array from project.json — full custom tree.
	 * 2. PLATFORM_NAVIGATION merged with plugin defaultNavigation — used when
	 *    the project has no explicit navigation config (legacy projects and
	 *    fresh projects without a methodology plugin installed).
	 *
	 * Returns null only when no project is loaded.
	 * @returns The navigation tree, or null if no project is loaded.
	 */
	private get _navTree(): NavigationItem[] | null {
		const { projectStore } = getStores();
		if (!projectStore.hasProject) return null;

		const explicit = projectStore.navigation;
		if (explicit) return explicit;

		// Build from PLATFORM_NAVIGATION + plugin contributions.
		return this._buildDefaultNavTree();
	}

	/**
	 * Build the default navigation tree from PLATFORM_NAVIGATION extended with
	 * any defaultNavigation contributions from registered plugins.
	 *
	 * Plugin items are inserted before the bottom fixed items (artifact-graph,
	 * plugins, settings).
	 * @returns The merged navigation tree.
	 */
	private _buildDefaultNavTree(): NavigationItem[] {
		const { pluginRegistry } = getStores();
		const base = [...PLATFORM_NAVIGATION] as NavigationItem[];

		// Collect plugin navigation contributions, merging groups that share the
		// same key (e.g. multiple plugins contributing to "discovery").
		const groupMap = new Map<string, NavigationItem>();
		const insertionOrder: string[] = [];

		for (const [, plugin] of pluginRegistry.plugins) {
			if (plugin.manifest.defaultNavigation) {
				for (const raw of plugin.manifest.defaultNavigation) {
					const item = raw as NavigationItem;
					const existing = groupMap.get(item.key);
					if (existing && existing.type === "group" && item.type === "group") {
						// Merge children, avoiding duplicate keys
						const existingKeys = new Set((existing.children ?? []).map((c) => c.key));
						for (const child of item.children ?? []) {
							if (!existingKeys.has(child.key)) {
								(existing.children ??= []).push(child);
							}
						}
					} else {
						groupMap.set(item.key, { ...item });
						insertionOrder.push(item.key);
					}
				}
			}
		}

		// Sort groups by pipeline stage order — tells the story of the workflow.
		// Groups not in this list appear after all listed groups.
		const STAGE_ORDER = ["discovery", "planning", "delivery", "learning", "documentation", "agents"];
		const pluginItems: NavigationItem[] = [];
		const ordered = new Set<string>();

		for (const key of STAGE_ORDER) {
			const item = groupMap.get(key);
			if (item) {
				pluginItems.push(item);
				ordered.add(key);
			}
		}
		// Append any groups not in the stage order list
		for (const key of insertionOrder) {
			if (!ordered.has(key)) {
				const item = groupMap.get(key);
				if (item) pluginItems.push(item);
				ordered.add(key);
			}
		}

		if (pluginItems.length === 0) return base;

		// Insert plugin items before the first bottom item (artifact-graph).
		const bottomKeys = new SvelteSet(["artifact-graph", "plugins", "settings"]);
		const insertAt = base.findIndex((i) => bottomKeys.has(i.key));
		if (insertAt === -1) return [...base, ...pluginItems];
		return [...base.slice(0, insertAt), ...pluginItems, ...base.slice(insertAt)];
	}

	/**
	 * Find a NavigationItem by key in the navigation tree (recursive).
	 * @param key - The navigation item key to search for.
	 * @returns The matching NavigationItem, or null if not found.
	 */
	findNavItem(key: string): NavigationItem | null {
		const tree = this._navTree;
		if (!tree) return null;
		return this._findInNavTree(tree, key);
	}

	/**
	 * Recursively search a list of NavigationItems for a matching key.
	 * @param items - The list to search.
	 * @param key - The key to find.
	 * @returns The matching item, or null.
	 */
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
	 * @param key - The child item key whose parent group to find.
	 * @returns The parent group NavigationItem, or null if not found.
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
	// Getters
	// -----------------------------------------------------------------------

	/**
	 * Flat list of all artifact type keys from the navigation tree (groups expanded to their children).
	 * @returns Array of artifact type key strings.
	 */
	get allArtifactKeys(): string[] {
		return this._allNavLeafKeys();
	}

	/**
	 * Get all leaf keys from the navigation tree.
	 * @returns Array of leaf key strings from the nav tree.
	 */
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

	/**
	 * Keys of entries that are groups (have children).
	 * @returns Array of group key strings.
	 */
	get groupKeys(): string[] {
		const tree = this._navTree ?? [];
		return tree.filter((i) => i.type === "group").map((i) => i.key);
	}

	/**
	 * Whether the given key is a group key.
	 * @param key - Navigation item key to test.
	 * @returns True if the key identifies a group.
	 */
	isGroupKey(key: string): boolean {
		return this.groupKeys.includes(key);
	}

	/**
	 * Whether the current activity is an artifact activity (not a built-in view).
	 * @returns True if the active view is an artifact type.
	 */
	get isArtifactActivity(): boolean {
		if (this.activeNavItem) {
			const builtinViews = ["project", "artifact-graph", "settings", "configure", "chat", "plugins"];
			if (builtinViews.includes(this.activeNavItem.key)) return false;
			if (this.activeNavItem.type === "plugin") return false;
			return this.activeNavItem.type === "builtin";
		}
		return this.allArtifactKeys.includes(this.activeActivity);
	}

	/**
	 * Whether any view should show the nav panel.
	 * @returns True if the nav panel should be visible.
	 */
	get showNavPanel(): boolean {
		if (this.navPanelCollapsed) return false;
		if (this.activeGroup !== null) return true;
		if (this.activeActivity === "chat") return true;
		if (this.activeActivity === "settings") return true;
		if (this.activeActivity === "configure") return true;
		if (this.activeActivity === "plugins") return true;
		if (this.isArtifactActivity) return true;
		return false;
	}

	/**
	 * Get label for a given key from the navigation tree.
	 * Falls back to humanized key.
	 * @param key - Navigation item key to look up.
	 * @returns Display label for the key.
	 */
	getLabelForKey(key: string): string {
		const item = this.findNavItem(key);
		if (item?.label) return item.label;
		const { pluginRegistry } = getStores();
		if (item?.type === "plugin" && item.pluginSource) {
			const resolved = pluginRegistry.resolveNavigationItem(item);
			return resolved.label;
		}
		return humanizeKey(key);
	}

	/**
	 * Sub-categories (children) for a given group key.
	 * @param groupKey - The group key whose children to return.
	 * @returns Array of sub-category configs, empty if the group has none.
	 */
	getGroupChildren(groupKey: string): SubCategoryConfig[] {
		const tree = this._navTree ?? [];
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

	/**
	 * All group sub-categories, keyed by group key.
	 * @returns Map of group key to child key arrays.
	 */
	get groupSubCategories(): Record<string, string[]> {
		const tree = this._navTree ?? [];
		const result: Record<string, string[]> = {};
		for (const item of tree) {
			if (item.type === "group" && item.children) {
				result[item.key] = item.children.filter((c) => !c.hidden).map((c) => c.key);
			}
		}
		return result;
	}

	/**
	 * Get top-level navigation items for the ActivityBar.
	 * Returns the navigation tree when a project is loaded, null otherwise.
	 * The tree is always available once a project is open — either from
	 * project.json or derived from PLATFORM_NAVIGATION + plugin contributions.
	 * @returns The navigation tree, or null when no project is loaded.
	 */
	get topLevelNavItems(): NavigationItem[] | null {
		return this._navTree;
	}

	/**
	 * Find the NavType for the given activity string, if the navTree has loaded.
	 * @param view - Activity key to look up.
	 * @returns The matching NavType, or null if not found.
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
					// Match by last path segment (e.g. "tasks" matches ".orqa/implementation/tasks")
					const typeKey = type.path.split("/").pop();
					if (typeKey === view) return type;
					// Match stage-prefixed keys (e.g. "discovery-research" matches ".orqa/discovery/research")
					// by joining the last two path segments with a hyphen.
					const segments = type.path.split("/");
					if (segments.length >= 2) {
						const compound = `${segments[segments.length - 2]}-${segments[segments.length - 1]}`;
						if (compound === view) return type;
					}
				}
			}
		}
		return null;
	}

	/**
	 * Return the configured `path` for the given artifact key from the plugin schema registry, or null if not found.
	 * @param key - Artifact type key to look up.
	 * @returns The default path from the schema, or null.
	 */
	getConfiguredPath(key: string): string | null {
		const { pluginRegistry } = getStores();
		const schema = pluginRegistry.getSchema(key);
		if (schema) return schema.defaultPath;
		return null;
	}

	// -----------------------------------------------------------------------
	// Navigation actions
	// -----------------------------------------------------------------------

	/**
	 * Activate a navigation group and select its first sub-category.
	 * @param group - Key of the group to activate.
	 */
	setGroup(group: string) {
		this.activeGroup = group;
		const children = this.getGroupChildren(group);
		const first = children[0];
		if (first) {
			this.setSubCategory(first.key);
		}
	}

	/**
	 * Activate a sub-category and navigate to its activity view.
	 * @param key - Sub-category key to activate.
	 */
	setSubCategory(key: string) {
		this.activeSubCategory = key;
		this.setActivity(key);
	}

	/**
	 * Navigate to the given activity view, updating all related nav state and the URL hash.
	 * @param view - Activity key to navigate to.
	 */
	setActivity(view: string) {
		log.info(`setActivity: ${this.activeActivity} → ${view}`);
		this.activeActivity = view;
		this.selectedArtifactPath = null;
		this.breadcrumbs = [];

		// Update activeNavItem for routing
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
		} else if (view === "plugins") {
			// Plugin browser lives in the nav panel; collapse the explorer.
			this.activeGroup = null;
			this.activeSubCategory = null;
			this.navPanelCollapsed = false;
		} else if (this.activeNavItem?.type === "plugin") {
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

	/**
	 * Open an artifact viewer for the given path.
	 * @param path - Relative file path of the artifact to open.
	 * @param breadcrumbs - Breadcrumb labels to display above the viewer.
	 */
	openArtifact(path: string, breadcrumbs: string[]) {
		log.info(`openArtifact: ${path}`);
		// Set loading state immediately so the spinner shows before the $effect fires loadContent
		const { artifactStore } = getStores();
		artifactStore.activeContent = null;
		artifactStore.activeContentLoading = true;
		artifactStore.activeContentError = null;

		this.selectedArtifactPath = path;
		this.explorerView = "artifact-viewer";
		this.breadcrumbs = breadcrumbs;
		this.syncToHash();
		// The timeEnd will be called manually when we want to measure render
		requestAnimationFrame(() => {
			log.perf("openArtifact → render");
		});
	}

	/** Close the artifact viewer and return to the artifact list. */
	closeArtifact() {
		this.selectedArtifactPath = null;
		this.explorerView = "artifact-list";
		this.breadcrumbs = [];
		this.syncToHash();
	}

	/**
	 * Navigate to an artifact by its ID string (e.g. "EPIC-005", "MS-001").
	 * @param id - The artifact ID to resolve and navigate to.
	 */
	navigateToArtifact(id: string) {
		const { artifactGraphSDK } = getStores();
		const node = artifactGraphSDK.resolve(id);
		if (!node) {
			log.warn(`navigateToArtifact: could not resolve artifact ID: ${id}`);
			return;
		}
		this.navigateToPath(node.path);
	}

	/**
	 * Navigate to an artifact by its relative file path.
	 * @param path - Relative file path of the artifact to navigate to.
	 */
	navigateToPath(path: string) {
		const { artifactStore } = getStores();
		const tree = artifactStore.navTree;
		if (!tree) {
			log.warn(`navigateToPath: navTree not yet loaded, cannot navigate to: ${path}`);
			return;
		}

		const normalizedPath = path.replace(/\\/g, "/");

		for (const group of tree.groups) {
			for (const navType of group.types) {
				const found = this._findNodeInNavType(navType, normalizedPath);
				if (found) {
					const typeKey = this._resolveKeyForNavTypePath(navType.path);
					if (!typeKey) {
						log.warn(`navigateToPath: no config key for NavType path: ${navType.path}`);
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

		log.warn(`navigateToPath: no NavTree node found for path: ${path}`);
	}

	/**
	 * Search for a NavDocNode within a NavType by normalized path.
	 * @param navType - The NavType whose nodes to search.
	 * @param normalizedPath - Forward-slash-normalized path to find.
	 * @returns The matching node, or null.
	 */
	private _findNodeInNavType(navType: NavType, normalizedPath: string): NavDocNode | null {
		return this._findNodeInList(navType.nodes, normalizedPath);
	}

	/**
	 * Recursively find a NavDocNode in a list by normalized path.
	 * @param nodes - List of nodes to search recursively.
	 * @param normalizedPath - Forward-slash-normalized path to match.
	 * @returns The matching node, or null.
	 */
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

	/**
	 * Resolve the plugin schema key for a given NavType path.
	 * @param navTypePath - Path of the NavType to look up.
	 * @returns The schema key, or null if not found.
	 */
	private _resolveKeyForNavTypePath(navTypePath: string): string | null {
		const { pluginRegistry } = getStores();
		const normalized = navTypePath.replace(/\\/g, "/").replace(/\/$/, "");
		for (const schema of pluginRegistry.allSchemas) {
			if (schema.defaultPath.replace(/\\/g, "/").replace(/\/$/, "") === normalized) {
				return schema.key;
			}
		}
		return null;
	}

	/**
	 * Resolve the parent group key for a given NavType path.
	 * @param navTypePath - Path of the NavType whose parent group to find.
	 * @returns The parent group key, or null if the type is not under a group.
	 */
	private _resolveGroupKeyForNavTypePath(navTypePath: string): string | null {
		const normalized = navTypePath.replace(/\\/g, "/").replace(/\/$/, "");
		const { pluginRegistry } = getStores();
		for (const schema of pluginRegistry.allSchemas) {
			if (schema.defaultPath.replace(/\\/g, "/").replace(/\/$/, "") === normalized) {
				const parent = this._findParentGroup(schema.key);
				if (parent) return parent.key;
			}
		}
		return null;
	}

	/** Toggle the collapsed state of the nav panel. */
	toggleNavPanel() {
		this.navPanelCollapsed = !this.navPanelCollapsed;
	}

	/** Toggle the search overlay open/closed. */
	toggleSearch() {
		this.searchOverlayOpen = !this.searchOverlayOpen;
	}
}
