/**
 * Tests for NavigationStore state management.
 *
 * NavigationStore calls getStores() for cross-store access (projectStore,
 * artifactStore, pluginRegistry). We mock the registry module so only the
 * navigation logic is under test.
 *
 * Focus: activity switching, artifact selection, breadcrumb management,
 * nav panel collapse, search overlay, and explorerView transitions.
 */
import { describe, it, expect, beforeEach, vi } from "vitest";

// ---------------------------------------------------------------------------
// Mock the store registry before importing NavigationStore
// ---------------------------------------------------------------------------

const mockProjectStore = {
	hasProject: false,
	navigation: null,
};

const mockPluginRegistry = {
	plugins: new Map(),
	allSchemas: [],
	resolveNavigationItem: vi.fn((item: { key: string; type: string; icon: string }) => ({
		key: item.key,
		label: item.key,
		icon: item.icon,
		type: item.type,
	})),
	getSchema: vi.fn(() => null),
};

const mockArtifactStore = {
	navTree: null,
	activeContent: null,
	activeContentLoading: false,
	activeContentError: null,
};

const mockArtifactGraphSDK = {
	resolve: vi.fn(() => undefined),
};

vi.mock("../../src/registry.svelte.js", () => ({
	getStores: () => ({
		projectStore: mockProjectStore,
		pluginRegistry: mockPluginRegistry,
		artifactStore: mockArtifactStore,
		artifactGraphSDK: mockArtifactGraphSDK,
	}),
}));

// Mock the router to prevent window.location access in tests
vi.mock("../../src/router.js", () => ({
	pushRoute: vi.fn(),
	currentRoute: vi.fn(() => ({ type: "default" })),
}));

// ---------------------------------------------------------------------------
// Import after mocks are established
// ---------------------------------------------------------------------------

import { NavigationStore } from "../../src/stores/navigation.svelte.js";

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("NavigationStore", () => {
	let nav: NavigationStore;

	beforeEach(() => {
		// Reset mock state between tests
		mockProjectStore.hasProject = false;
		mockProjectStore.navigation = null;
		vi.clearAllMocks();

		nav = new NavigationStore();
	});

	describe("initial state", () => {
		it("starts with chat as the active activity", () => {
			expect(nav.activeActivity).toBe("chat");
		});

		it("starts with no active group or sub-category", () => {
			expect(nav.activeGroup).toBeNull();
			expect(nav.activeSubCategory).toBeNull();
		});

		it("starts with artifact-list explorer view", () => {
			expect(nav.explorerView).toBe("artifact-list");
		});

		it("starts with no selected artifact path", () => {
			expect(nav.selectedArtifactPath).toBeNull();
		});

		it("starts with nav panel expanded", () => {
			expect(nav.navPanelCollapsed).toBe(false);
		});

		it("starts with empty breadcrumbs", () => {
			expect(nav.breadcrumbs).toEqual([]);
		});

		it("starts with search overlay closed", () => {
			expect(nav.searchOverlayOpen).toBe(false);
		});

		it("starts with no active nav item", () => {
			expect(nav.activeNavItem).toBeNull();
		});
	});

	describe("setActivity", () => {
		it("updates activeActivity", () => {
			nav.setActivity("settings");
			expect(nav.activeActivity).toBe("settings");
		});

		it("clears selectedArtifactPath and breadcrumbs", () => {
			nav.selectedArtifactPath = ".orqa/tasks/TASK-001.md";
			nav.breadcrumbs = ["Tasks", "TASK-001"];

			nav.setActivity("chat");

			expect(nav.selectedArtifactPath).toBeNull();
			expect(nav.breadcrumbs).toEqual([]);
		});

		it("collapses nav panel for 'project' activity", () => {
			nav.navPanelCollapsed = false;
			nav.setActivity("project");

			expect(nav.explorerView).toBe("project-dashboard");
			expect(nav.navPanelCollapsed).toBe(true);
			expect(nav.activeGroup).toBeNull();
			expect(nav.activeSubCategory).toBeNull();
		});

		it("collapses nav panel for 'artifact-graph' activity", () => {
			nav.navPanelCollapsed = false;
			nav.setActivity("artifact-graph");

			expect(nav.navPanelCollapsed).toBe(true);
			expect(nav.activeGroup).toBeNull();
			expect(nav.activeSubCategory).toBeNull();
		});

		it("sets explorer view to settings for 'settings' activity", () => {
			nav.setActivity("settings");

			expect(nav.explorerView).toBe("settings");
			expect(nav.activeGroup).toBeNull();
			expect(nav.activeSubCategory).toBeNull();
		});

		it("sets explorer view to settings for 'configure' activity", () => {
			nav.setActivity("configure");

			expect(nav.explorerView).toBe("settings");
		});

		it("clears group for 'plugins' activity", () => {
			nav.activeGroup = "delivery";
			nav.setActivity("plugins");

			expect(nav.activeGroup).toBeNull();
			expect(nav.activeSubCategory).toBeNull();
		});

		it("sets activeNavItem to builtin type for unknown views", () => {
			nav.setActivity("unknown-activity");

			expect(nav.activeNavItem?.type).toBe("builtin");
			expect(nav.activeNavItem?.key).toBe("unknown-activity");
		});
	});

	describe("closeArtifact", () => {
		it("clears selectedArtifactPath and returns to artifact-list", () => {
			nav.selectedArtifactPath = ".orqa/tasks/TASK-001.md";
			nav.explorerView = "artifact-viewer";
			nav.breadcrumbs = ["Tasks"];

			nav.closeArtifact();

			expect(nav.selectedArtifactPath).toBeNull();
			expect(nav.explorerView).toBe("artifact-list");
			expect(nav.breadcrumbs).toEqual([]);
		});
	});

	describe("toggleNavPanel", () => {
		it("toggles navPanelCollapsed from false to true", () => {
			nav.navPanelCollapsed = false;
			nav.toggleNavPanel();
			expect(nav.navPanelCollapsed).toBe(true);
		});

		it("toggles navPanelCollapsed from true to false", () => {
			nav.navPanelCollapsed = true;
			nav.toggleNavPanel();
			expect(nav.navPanelCollapsed).toBe(false);
		});
	});

	describe("toggleSearch", () => {
		it("opens the search overlay when closed", () => {
			nav.searchOverlayOpen = false;
			nav.toggleSearch();
			expect(nav.searchOverlayOpen).toBe(true);
		});

		it("closes the search overlay when open", () => {
			nav.searchOverlayOpen = true;
			nav.toggleSearch();
			expect(nav.searchOverlayOpen).toBe(false);
		});
	});

	describe("topLevelNavItems", () => {
		it("returns null when no project is loaded", () => {
			mockProjectStore.hasProject = false;
			expect(nav.topLevelNavItems).toBeNull();
		});

		it("returns explicit navigation from project when set", () => {
			mockProjectStore.hasProject = true;
			mockProjectStore.navigation = [
				{ key: "chat", type: "builtin", icon: "message-circle", label: "Chat" },
			] as never;

			expect(nav.topLevelNavItems).toHaveLength(1);
			expect(nav.topLevelNavItems?.[0].key).toBe("chat");
		});
	});

	describe("groupKeys", () => {
		it("returns empty array when no project loaded", () => {
			mockProjectStore.hasProject = false;
			expect(nav.groupKeys).toEqual([]);
		});
	});

	describe("isGroupKey", () => {
		it("returns false for any key when no project loaded", () => {
			mockProjectStore.hasProject = false;
			expect(nav.isGroupKey("delivery")).toBe(false);
		});
	});

	describe("showNavPanel", () => {
		it("returns false when nav panel is collapsed", () => {
			nav.navPanelCollapsed = true;
			expect(nav.showNavPanel).toBe(false);
		});

		it("returns true when there is an active group", () => {
			nav.navPanelCollapsed = false;
			nav.activeGroup = "delivery";
			expect(nav.showNavPanel).toBe(true);
		});

		it("returns true for chat activity", () => {
			nav.navPanelCollapsed = false;
			nav.activeActivity = "chat";
			expect(nav.showNavPanel).toBe(true);
		});

		it("returns true for settings activity", () => {
			nav.navPanelCollapsed = false;
			nav.activeActivity = "settings";
			expect(nav.showNavPanel).toBe(true);
		});

		it("returns true for plugins activity", () => {
			nav.navPanelCollapsed = false;
			nav.activeActivity = "plugins";
			expect(nav.showNavPanel).toBe(true);
		});
	});

	describe("setGroup", () => {
		it("sets activeGroup", () => {
			nav.setGroup("delivery");
			expect(nav.activeGroup).toBe("delivery");
		});

		it("does not throw even when group has no children (nav tree returns empty)", () => {
			mockProjectStore.hasProject = false;
			expect(() => nav.setGroup("discovery")).not.toThrow();
		});
	});

	describe("setSubCategory", () => {
		it("sets activeSubCategory and calls setActivity", () => {
			nav.setSubCategory("tasks");
			expect(nav.activeSubCategory).toBe("tasks");
			expect(nav.activeActivity).toBe("tasks");
		});
	});

	describe("findNavItem", () => {
		it("returns null when no project is loaded (navTree is null)", () => {
			mockProjectStore.hasProject = false;
			expect(nav.findNavItem("chat")).toBeNull();
		});

		it("finds an item in the explicit navigation tree", () => {
			mockProjectStore.hasProject = true;
			mockProjectStore.navigation = [
				{ key: "chat", type: "builtin", icon: "message-circle", label: "Chat" },
				{ key: "settings", type: "builtin", icon: "settings", label: "Settings" },
			] as never;

			const item = nav.findNavItem("settings");
			expect(item).not.toBeNull();
			expect(item?.key).toBe("settings");
		});

		it("finds a nested item in a group", () => {
			mockProjectStore.hasProject = true;
			mockProjectStore.navigation = [
				{
					key: "delivery",
					type: "group",
					icon: "layers",
					label: "Delivery",
					children: [
						{ key: "tasks", type: "builtin", icon: "check-square", label: "Tasks" },
						{ key: "epics", type: "builtin", icon: "flag", label: "Epics" },
					],
				},
			] as never;

			const item = nav.findNavItem("tasks");
			expect(item).not.toBeNull();
			expect(item?.key).toBe("tasks");
		});

		it("returns null for unknown key", () => {
			mockProjectStore.hasProject = true;
			mockProjectStore.navigation = [
				{ key: "chat", type: "builtin", icon: "message-circle", label: "Chat" },
			] as never;

			expect(nav.findNavItem("not-here")).toBeNull();
		});
	});

	describe("getGroupChildren", () => {
		it("returns empty array when no project loaded", () => {
			mockProjectStore.hasProject = false;
			expect(nav.getGroupChildren("delivery")).toEqual([]);
		});

		it("returns children of a group from explicit navigation", () => {
			mockProjectStore.hasProject = true;
			mockProjectStore.navigation = [
				{
					key: "delivery",
					type: "group",
					icon: "layers",
					children: [
						{ key: "tasks", type: "builtin", icon: "check-square", label: "Tasks" },
						{ key: "epics", type: "builtin", icon: "flag", label: "Epics", hidden: false },
					],
				},
			] as never;

			const children = nav.getGroupChildren("delivery");
			expect(children).toHaveLength(2);
			expect(children[0].key).toBe("tasks");
			expect(children[1].key).toBe("epics");
		});

		it("excludes hidden children", () => {
			mockProjectStore.hasProject = true;
			mockProjectStore.navigation = [
				{
					key: "delivery",
					type: "group",
					icon: "layers",
					children: [
						{ key: "tasks", type: "builtin", icon: "check-square", label: "Tasks", hidden: false },
						{ key: "drafts", type: "builtin", icon: "file", label: "Drafts", hidden: true },
					],
				},
			] as never;

			const children = nav.getGroupChildren("delivery");
			expect(children).toHaveLength(1);
			expect(children[0].key).toBe("tasks");
		});
	});

	describe("getLabelForKey", () => {
		it("returns humanized label for unknown key", () => {
			mockProjectStore.hasProject = false;
			const label = nav.getLabelForKey("my-artifact-type");
			expect(label).toBe("My Artifact Type");
		});

		it("returns the label from the nav tree item when available", () => {
			mockProjectStore.hasProject = true;
			mockProjectStore.navigation = [
				{ key: "tasks", type: "builtin", icon: "check-square", label: "Work Items" },
			] as never;

			expect(nav.getLabelForKey("tasks")).toBe("Work Items");
		});
	});

	describe("allArtifactKeys", () => {
		it("returns empty array when no project loaded", () => {
			mockProjectStore.hasProject = false;
			expect(nav.allArtifactKeys).toEqual([]);
		});

		it("returns leaf keys from the nav tree", () => {
			mockProjectStore.hasProject = true;
			mockProjectStore.navigation = [
				{ key: "chat", type: "builtin", icon: "message-circle" },
				{
					key: "delivery",
					type: "group",
					icon: "layers",
					children: [
						{ key: "tasks", type: "builtin", icon: "check-square" },
						{ key: "epics", type: "builtin", icon: "flag" },
					],
				},
			] as never;

			const keys = nav.allArtifactKeys;
			expect(keys).toContain("chat");
			expect(keys).toContain("tasks");
			expect(keys).toContain("epics");
			// Groups themselves should not appear
			expect(keys).not.toContain("delivery");
		});
	});
});
