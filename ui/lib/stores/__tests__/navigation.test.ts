import { describe, it, expect, vi, beforeEach } from "vitest";
import "./setup";

// Mock the stores that navigation depends on
vi.mock("$lib/stores/artifact.svelte", () => ({
	artifactStore: {
		navTree: null as import("$lib/types/nav-tree").NavTree | null,
	},
}));

vi.mock("$lib/stores/project.svelte", () => ({
	projectStore: {
		artifactConfig: [] as import("$lib/types/project").ArtifactEntry[],
	},
}));

vi.mock("$lib/sdk/artifact-graph.svelte", () => ({
	artifactGraphSDK: {
		resolve: vi.fn(),
	},
}));

import { navigationStore } from "../navigation.svelte";
import { artifactStore } from "$lib/stores/artifact.svelte";
import { projectStore } from "$lib/stores/project.svelte";
import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
import type { ArtifactEntry } from "$lib/types/project";
import type { NavTree } from "$lib/types/nav-tree";

// Sample config with a group and a direct type
const sampleConfig: ArtifactEntry[] = [
	{
		key: "planning",
		label: "Planning",
		icon: "target",
		children: [
			{ key: "epics", label: "Epics", path: ".orqa/planning/epics" },
			{ key: "tasks", label: "Tasks", path: ".orqa/planning/tasks" },
		],
	},
	{
		key: "docs",
		label: "Documentation",
		path: ".orqa/documentation",
	},
];

const sampleNavTree: NavTree = {
	groups: [
		{
			key: "planning",
			label: "Planning",
			icon: "target",
			types: [
				{
					label: "Epics",
					path: ".orqa/planning/epics",
					icon: "rocket",
					description: "Epic artifacts",
					nodes: [
						{
							label: "EPIC-001",
							path: ".orqa/planning/epics/EPIC-001.md",
							description: "First epic",
						},
					],
				},
				{
					label: "Tasks",
					path: ".orqa/planning/tasks",
					icon: "check",
					description: "Task artifacts",
					nodes: [
						{
							label: "TASK-001",
							path: ".orqa/planning/tasks/TASK-001.md",
							description: "First task",
						},
					],
				},
			],
		},
		{
			key: "docs",
			label: "Documentation",
			icon: "book",
			types: [
				{
					label: "Documentation",
					path: ".orqa/documentation",
					icon: "book",
					description: "Docs",
					nodes: [
						{
							label: "Architecture",
							path: ".orqa/documentation/architecture",
							description: "Architecture folder",
							children: [
								{
									label: "Overview",
									path: ".orqa/documentation/architecture/overview.md",
									description: "Arch overview",
								},
							],
						},
					],
				},
			],
		},
	],
};

function resetNav() {
	navigationStore.activeActivity = "chat";
	navigationStore.activeGroup = null;
	navigationStore.activeSubCategory = null;
	navigationStore.explorerView = "artifact-list";
	navigationStore.selectedArtifactPath = null;
	navigationStore.navPanelCollapsed = false;
	navigationStore.breadcrumbs = [];
	navigationStore.searchOverlayOpen = false;
}

beforeEach(() => {
	resetNav();
	(projectStore as { artifactConfig: ArtifactEntry[] }).artifactConfig = sampleConfig;
	(artifactStore as { navTree: NavTree | null }).navTree = null;
	vi.mocked(artifactGraphSDK.resolve).mockReset();
});

describe("NavigationStore", () => {
	describe("allArtifactKeys", () => {
		it("returns flat list expanding groups", () => {
			expect(navigationStore.allArtifactKeys).toEqual(["epics", "tasks", "docs"]);
		});

		it("returns empty when config is empty", () => {
			(projectStore as { artifactConfig: ArtifactEntry[] }).artifactConfig = [];
			expect(navigationStore.allArtifactKeys).toEqual([]);
		});
	});

	describe("groupKeys", () => {
		it("returns keys of group entries only", () => {
			expect(navigationStore.groupKeys).toEqual(["planning"]);
		});
	});

	describe("isGroupKey", () => {
		it("returns true for group keys", () => {
			expect(navigationStore.isGroupKey("planning")).toBe(true);
		});

		it("returns false for non-group keys", () => {
			expect(navigationStore.isGroupKey("docs")).toBe(false);
		});
	});

	describe("isArtifactActivity", () => {
		it("is false for built-in views", () => {
			navigationStore.activeActivity = "chat";
			expect(navigationStore.isArtifactActivity).toBe(false);
		});

		it("is true for artifact type keys", () => {
			navigationStore.activeActivity = "epics";
			expect(navigationStore.isArtifactActivity).toBe(true);
		});
	});

	describe("showNavPanel", () => {
		it("is false when panel is collapsed", () => {
			navigationStore.navPanelCollapsed = true;
			expect(navigationStore.showNavPanel).toBe(false);
		});

		it("is true when a group is active", () => {
			navigationStore.activeGroup = "planning";
			expect(navigationStore.showNavPanel).toBe(true);
		});

		it("is true for chat activity", () => {
			navigationStore.activeActivity = "chat";
			expect(navigationStore.showNavPanel).toBe(true);
		});

		it("is true for settings activity", () => {
			navigationStore.activeActivity = "settings";
			expect(navigationStore.showNavPanel).toBe(true);
		});

		it("is true for configure activity", () => {
			navigationStore.activeActivity = "configure";
			expect(navigationStore.showNavPanel).toBe(true);
		});

		it("is true for artifact activities", () => {
			navigationStore.activeActivity = "docs";
			expect(navigationStore.showNavPanel).toBe(true);
		});

		it("is false for project activity (non-artifact, no group)", () => {
			navigationStore.activeActivity = "project";
			navigationStore.activeGroup = null;
			expect(navigationStore.showNavPanel).toBe(false);
		});
	});

	describe("getLabelForKey", () => {
		it("returns label from config", () => {
			expect(navigationStore.getLabelForKey("docs")).toBe("Documentation");
		});

		it("returns label for group child", () => {
			expect(navigationStore.getLabelForKey("epics")).toBe("Epics");
		});

		it("falls back to humanized key", () => {
			expect(navigationStore.getLabelForKey("unknown-key")).toBe("Unknown Key");
		});
	});

	describe("getGroupChildren", () => {
		it("returns children for a group key", () => {
			const children = navigationStore.getGroupChildren("planning");
			expect(children).toEqual([
				{ key: "epics", label: "Epics" },
				{ key: "tasks", label: "Tasks" },
			]);
		});

		it("returns empty for non-group key", () => {
			expect(navigationStore.getGroupChildren("docs")).toEqual([]);
		});
	});

	describe("groupSubCategories", () => {
		it("returns map of group key to child keys", () => {
			expect(navigationStore.groupSubCategories).toEqual({
				planning: ["epics", "tasks"],
			});
		});
	});

	describe("setActivity", () => {
		it("sets project view correctly", () => {
			navigationStore.setActivity("project");
			expect(navigationStore.activeActivity).toBe("project");
			expect(navigationStore.activeGroup).toBeNull();
			expect(navigationStore.activeSubCategory).toBeNull();
			expect(navigationStore.explorerView).toBe("project-dashboard");
			expect(navigationStore.navPanelCollapsed).toBe(true);
		});

		it("sets settings view correctly", () => {
			navigationStore.navPanelCollapsed = true;
			navigationStore.setActivity("settings");
			expect(navigationStore.explorerView).toBe("settings");
			expect(navigationStore.navPanelCollapsed).toBe(false);
		});

		it("sets artifact view correctly", () => {
			navigationStore.navPanelCollapsed = true;
			navigationStore.setActivity("epics");
			expect(navigationStore.explorerView).toBe("artifact-list");
			expect(navigationStore.navPanelCollapsed).toBe(false);
		});

		it("clears selected artifact and breadcrumbs", () => {
			navigationStore.selectedArtifactPath = "some/path.md";
			navigationStore.breadcrumbs = ["Foo"];
			navigationStore.setActivity("chat");
			expect(navigationStore.selectedArtifactPath).toBeNull();
			expect(navigationStore.breadcrumbs).toEqual([]);
		});
	});

	describe("setGroup", () => {
		it("sets group and auto-selects first child", () => {
			navigationStore.setGroup("planning");
			expect(navigationStore.activeGroup).toBe("planning");
			expect(navigationStore.activeSubCategory).toBe("epics");
			expect(navigationStore.activeActivity).toBe("epics");
		});
	});

	describe("openArtifact / closeArtifact", () => {
		it("opens artifact with path and breadcrumbs", () => {
			navigationStore.openArtifact(".orqa/planning/epics/EPIC-001.md", ["Epics", "EPIC-001"]);
			expect(navigationStore.selectedArtifactPath).toBe(".orqa/planning/epics/EPIC-001.md");
			expect(navigationStore.explorerView).toBe("artifact-viewer");
			expect(navigationStore.breadcrumbs).toEqual(["Epics", "EPIC-001"]);
		});

		it("closes artifact and resets to list", () => {
			navigationStore.openArtifact("some/path.md", ["crumb"]);
			navigationStore.closeArtifact();
			expect(navigationStore.selectedArtifactPath).toBeNull();
			expect(navigationStore.explorerView).toBe("artifact-list");
			expect(navigationStore.breadcrumbs).toEqual([]);
		});
	});

	describe("getNavType", () => {
		it("returns null when navTree is not loaded", () => {
			expect(navigationStore.getNavType("epics")).toBeNull();
		});

		it("finds nav type by configured path", () => {
			(artifactStore as { navTree: NavTree | null }).navTree = sampleNavTree;
			const navType = navigationStore.getNavType("epics");
			expect(navType).not.toBeNull();
			expect(navType?.label).toBe("Epics");
		});

		it("returns null for unknown key", () => {
			(artifactStore as { navTree: NavTree | null }).navTree = sampleNavTree;
			expect(navigationStore.getNavType("nonexistent")).toBeNull();
		});
	});

	describe("getConfiguredPath", () => {
		it("returns path for direct type", () => {
			expect(navigationStore.getConfiguredPath("docs")).toBe(".orqa/documentation");
		});

		it("returns path for group child", () => {
			expect(navigationStore.getConfiguredPath("epics")).toBe(".orqa/planning/epics");
		});

		it("returns null for unknown key", () => {
			expect(navigationStore.getConfiguredPath("nope")).toBeNull();
		});
	});

	describe("navigateToArtifact", () => {
		it("resolves ID via SDK and navigates to path", () => {
			(artifactStore as { navTree: NavTree | null }).navTree = sampleNavTree;
			vi.mocked(artifactGraphSDK.resolve).mockReturnValue({
				path: ".orqa/planning/epics/EPIC-001.md",
				id: "EPIC-001",
				title: "First epic",
			});

			navigationStore.navigateToArtifact("EPIC-001");

			expect(artifactGraphSDK.resolve).toHaveBeenCalledWith("EPIC-001");
			expect(navigationStore.selectedArtifactPath).toBe(".orqa/planning/epics/EPIC-001.md");
			expect(navigationStore.activeActivity).toBe("epics");
			expect(navigationStore.activeGroup).toBe("planning");
			expect(navigationStore.activeSubCategory).toBe("epics");
		});

		it("warns and does nothing if ID cannot be resolved", () => {
			vi.mocked(artifactGraphSDK.resolve).mockReturnValue(null);
			const warnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});

			navigationStore.navigateToArtifact("NONEXISTENT-999");

			expect(warnSpy).toHaveBeenCalled();
			expect(navigationStore.selectedArtifactPath).toBeNull();
			warnSpy.mockRestore();
		});
	});

	describe("navigateToPath", () => {
		it("navigates to a grouped artifact path", () => {
			(artifactStore as { navTree: NavTree | null }).navTree = sampleNavTree;

			navigationStore.navigateToPath(".orqa/planning/epics/EPIC-001.md");

			expect(navigationStore.activeActivity).toBe("epics");
			expect(navigationStore.activeGroup).toBe("planning");
			expect(navigationStore.activeSubCategory).toBe("epics");
			expect(navigationStore.selectedArtifactPath).toBe(".orqa/planning/epics/EPIC-001.md");
			expect(navigationStore.explorerView).toBe("artifact-viewer");
		});

		it("navigates to a non-grouped artifact (tree children)", () => {
			(artifactStore as { navTree: NavTree | null }).navTree = sampleNavTree;

			navigationStore.navigateToPath(".orqa/documentation/architecture/overview.md");

			expect(navigationStore.activeActivity).toBe("docs");
			expect(navigationStore.activeGroup).toBeNull();
			expect(navigationStore.selectedArtifactPath).toBe(
				".orqa/documentation/architecture/overview.md",
			);
		});

		it("warns when navTree is null", () => {
			const warnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});

			navigationStore.navigateToPath(".orqa/planning/epics/EPIC-001.md");

			expect(warnSpy).toHaveBeenCalled();
			expect(navigationStore.selectedArtifactPath).toBeNull();
			warnSpy.mockRestore();
		});

		it("warns when path is not found in tree", () => {
			(artifactStore as { navTree: NavTree | null }).navTree = sampleNavTree;
			const warnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});

			navigationStore.navigateToPath(".orqa/nonexistent/file.md");

			expect(warnSpy).toHaveBeenCalled();
			warnSpy.mockRestore();
		});
	});

	describe("toggleNavPanel", () => {
		it("toggles collapsed state", () => {
			expect(navigationStore.navPanelCollapsed).toBe(false);
			navigationStore.toggleNavPanel();
			expect(navigationStore.navPanelCollapsed).toBe(true);
			navigationStore.toggleNavPanel();
			expect(navigationStore.navPanelCollapsed).toBe(false);
		});
	});

	describe("toggleSearch", () => {
		it("toggles search overlay", () => {
			expect(navigationStore.searchOverlayOpen).toBe(false);
			navigationStore.toggleSearch();
			expect(navigationStore.searchOverlayOpen).toBe(true);
			navigationStore.toggleSearch();
			expect(navigationStore.searchOverlayOpen).toBe(false);
		});
	});
});
