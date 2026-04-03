// Tests for createMockStores — verifies the mock store shape matches what connected components expect.
import { describe, it, expect, beforeEach } from "vitest";
import { createMockStores, getStores } from "../../src/testing/mock-stores.js";

describe("createMockStores", () => {
	beforeEach(() => {
		// Reset global stores between tests
		(globalThis as Record<string, unknown>).__orqa_stores = undefined;
	});

	it("creates stores with default project name", () => {
		const stores = createMockStores();
		expect(stores.projectStore.projectName).toBe("OrqaStudio");
	});

	it("allows overriding the project name", () => {
		const stores = createMockStores({
			projectStore: { projectName: "My Project" },
		});
		expect(stores.projectStore.projectName).toBe("My Project");
	});

	it("registers stores on globalThis.__orqa_stores", () => {
		createMockStores();
		expect((globalThis as Record<string, unknown>).__orqa_stores).toBeDefined();
	});

	it("provides a projectStore with getStatusConfig method", () => {
		const stores = createMockStores();
		const config = stores.projectStore.getStatusConfig("captured");
		expect(config).not.toBeNull();
		expect(config!.key).toBe("captured");
	});

	it("getStatusConfig returns null for unknown status", () => {
		const stores = createMockStores();
		const config = stores.projectStore.getStatusConfig("totally-unknown");
		expect(config).toBeNull();
	});

	it("provides an artifactStore with getArtifact method", () => {
		const stores = createMockStores();
		const artifact = stores.artifactStore.getArtifact("EPIC-001");
		expect(artifact).not.toBeNull();
		expect(artifact!.id).toBe("EPIC-001");
	});

	it("getArtifact returns null for unknown id", () => {
		const stores = createMockStores();
		expect(stores.artifactStore.getArtifact("UNKNOWN-999")).toBeNull();
	});

	it("provides a toastStore with add and remove", () => {
		const stores = createMockStores();
		const id = stores.toastStore.add({ message: "Test", type: "info" });
		expect(typeof id).toBe("string");
		expect(stores.toastStore.toasts).toHaveLength(1);
		stores.toastStore.remove(id);
		expect(stores.toastStore.toasts).toHaveLength(0);
	});

	it("provides a navigationStore with navigate method", () => {
		const stores = createMockStores();
		stores.navigationStore.navigate("artifacts", "EPIC-001");
		expect(stores.navigationStore.currentView).toBe("artifacts");
		expect(stores.navigationStore.currentArtifactId).toBe("EPIC-001");
	});
});

describe("getStores", () => {
	beforeEach(() => {
		(globalThis as Record<string, unknown>).__orqa_stores = undefined;
	});

	it("throws when stores have not been initialized", () => {
		expect(() => getStores()).toThrow("OrqaStudio stores not found");
	});

	it("returns the stores created by createMockStores", () => {
		const created = createMockStores();
		const retrieved = getStores();
		expect(retrieved.projectStore.projectName).toBe(created.projectStore.projectName);
	});
});
