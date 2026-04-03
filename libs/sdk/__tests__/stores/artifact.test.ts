import { describe, it, expect, vi, beforeEach } from "vitest";
import { mockInvoke } from "./setup";

// ArtifactStore.loadContent delegates to getStores().artifactGraphSDK.readContent
const mockReadContent = vi.fn();
vi.mock("../../src/registry.svelte.js", () => ({
	getStores: () => ({
		artifactGraphSDK: { readContent: mockReadContent, resolve: vi.fn() },
	}),
}));

import { ArtifactStore } from "../../src/stores/artifact.svelte";
import type { NavTree } from "@orqastudio/types";

const fakeNavTree: NavTree = {
	groups: [
		{
			key: "planning",
			label: "Planning",
			icon: "target",
			types: [
				{
					label: "Epics",
					path: ".orqa/implementation/epics",
					icon: "rocket",
					description: "Epic artifacts",
					nodes: [
						{ label: "EPIC-001", path: ".orqa/implementation/epics/EPIC-001.md", description: "First epic" },
					],
				},
			],
		},
	],
};

let artifactStore: ArtifactStore;

beforeEach(() => {
	mockInvoke.mockReset();
	mockReadContent.mockReset();
	artifactStore = new ArtifactStore();
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
			mockReadContent.mockResolvedValueOnce("# Hello\nContent here");

			await artifactStore.loadContent(".orqa/implementation/epics/EPIC-001.md");

			expect(mockReadContent).toHaveBeenCalledWith(".orqa/implementation/epics/EPIC-001.md");
			expect(artifactStore.activeContent).toBe("# Hello\nContent here");
			expect(artifactStore.activeContentLoading).toBe(false);
		});

		it("sets error on failure", async () => {
			mockReadContent.mockRejectedValueOnce(new Error("File not found"));

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
			expect(artifactStore.navTree).toBeNull();

			// loadNavTree is called async — wait for it to complete
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
