import { describe, it, expect, vi, beforeEach } from "vitest";
import { mockInvoke } from "./setup";

// Mock the artifact-graph SDK before importing the store
vi.mock("$lib/sdk/artifact-graph.svelte", () => ({
	artifactGraphSDK: {
		readContent: vi.fn(),
		resolve: vi.fn(),
	},
}));

import { artifactStore } from "../artifact.svelte";
import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
import type { NavTree } from "$lib/types/nav-tree";

const fakeNavTree: NavTree = {
	groups: [
		{
			key: "planning",
			label: "Planning",
			icon: "target",
			types: [
				{
					label: "Epics",
					path: ".orqa/delivery/epics",
					icon: "rocket",
					description: "Epic artifacts",
					nodes: [
						{ label: "EPIC-001", path: ".orqa/delivery/epics/EPIC-001.md", description: "First epic" },
					],
				},
			],
		},
	],
};

beforeEach(() => {
	mockInvoke.mockReset();
	vi.mocked(artifactGraphSDK.readContent).mockReset();
	artifactStore.clear();
});

describe("ArtifactStore", () => {
	describe("initial state", () => {
		it("starts empty", () => {
			expect(artifactStore.navTree).toBeNull();
			expect(artifactStore.navTreeLoading).toBe(false);
			expect(artifactStore.navTreeError).toBeNull();
			expect(artifactStore.activeContent).toBeNull();
			expect(artifactStore.activeContentLoading).toBe(false);
			expect(artifactStore.activeContentError).toBeNull();
		});
	});

	describe("loadNavTree", () => {
		it("loads the nav tree from backend", async () => {
			mockInvoke.mockResolvedValueOnce(fakeNavTree);

			await artifactStore.loadNavTree();

			expect(mockInvoke).toHaveBeenCalledWith("artifact_scan_tree", undefined);
			expect(artifactStore.navTree).toEqual(fakeNavTree);
			expect(artifactStore.navTreeLoading).toBe(false);
			expect(artifactStore.navTreeError).toBeNull();
		});

		it("sets loading during fetch", async () => {
			let loadingDuringFetch = false;
			mockInvoke.mockImplementation(() => {
				loadingDuringFetch = artifactStore.navTreeLoading;
				return Promise.resolve(fakeNavTree);
			});

			await artifactStore.loadNavTree();

			expect(loadingDuringFetch).toBe(true);
		});

		it("sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Scan failed"));

			await artifactStore.loadNavTree();

			expect(artifactStore.navTreeError).toContain("Scan failed");
			expect(artifactStore.navTreeLoading).toBe(false);
		});

		it("skips if already loading", async () => {
			artifactStore.navTreeLoading = true;
			await artifactStore.loadNavTree();

			expect(mockInvoke).not.toHaveBeenCalled();
		});
	});

	describe("loadContent", () => {
		it("loads content via SDK", async () => {
			vi.mocked(artifactGraphSDK.readContent).mockResolvedValueOnce("# Hello\nContent here");

			await artifactStore.loadContent(".orqa/delivery/epics/EPIC-001.md");

			expect(artifactGraphSDK.readContent).toHaveBeenCalledWith(".orqa/delivery/epics/EPIC-001.md");
			expect(artifactStore.activeContent).toBe("# Hello\nContent here");
			expect(artifactStore.activeContentLoading).toBe(false);
		});

		it("sets error on failure", async () => {
			vi.mocked(artifactGraphSDK.readContent).mockRejectedValueOnce(new Error("File not found"));

			await artifactStore.loadContent("nonexistent.md");

			expect(artifactStore.activeContentError).toContain("File not found");
			expect(artifactStore.activeContent).toBeNull();
		});
	});

	describe("invalidateNavTree", () => {
		it("clears tree and triggers reload", async () => {
			artifactStore.navTree = fakeNavTree;
			mockInvoke.mockResolvedValueOnce(fakeNavTree);

			artifactStore.invalidateNavTree();

			// Tree is nulled immediately
			// loadNavTree is called (async) — we just verify invoke was called
			// Wait for the async call to complete
			await vi.waitFor(() => {
				expect(mockInvoke).toHaveBeenCalledWith("artifact_scan_tree", undefined);
			});
		});
	});

	describe("clear", () => {
		it("resets all state", () => {
			artifactStore.navTree = fakeNavTree;
			artifactStore.navTreeLoading = true;
			artifactStore.navTreeError = "error";
			artifactStore.activeContent = "content";
			artifactStore.activeContentLoading = true;
			artifactStore.activeContentError = "error";

			artifactStore.clear();

			expect(artifactStore.navTree).toBeNull();
			expect(artifactStore.navTreeLoading).toBe(false);
			expect(artifactStore.navTreeError).toBeNull();
			expect(artifactStore.activeContent).toBeNull();
			expect(artifactStore.activeContentLoading).toBe(false);
			expect(artifactStore.activeContentError).toBeNull();
		});
	});
});
