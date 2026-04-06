/**
 * Mock store factory for developing and testing components
 * without the OrqaStudio app or Tauri backend.
 *
 * Creates a full OrqaStores-compatible object with sensible defaults
 * and registers it on globalThis.__orqa_stores.
 */

import { FIXTURE_ARTIFACTS, FIXTURE_PROJECT_SETTINGS } from "./fixtures.js";

export interface MockStoreOverrides {
	readonly projectStore?: Readonly<
		Partial<{
			readonly hasProject: boolean;
			readonly projectName: string;
			readonly projectPath: string;
			readonly settings: typeof FIXTURE_PROJECT_SETTINGS;
		}>
	>;
	readonly settingsStore?: Readonly<
		Partial<{
			readonly theme: "light" | "dark" | "system";
			readonly model: string;
		}>
	>;
	readonly artifacts?: typeof FIXTURE_ARTIFACTS;
	readonly toasts?: ReadonlyArray<{
		readonly id: string;
		readonly message: string;
		readonly type: "info" | "error" | "success" | "warning";
	}>;
}

/**
 * Create a fully populated mock store object compatible with OrqaStores.
 * Registers the result on globalThis.__orqa_stores for component discovery.
 * @param overrides - Optional partial overrides for individual store slices.
 * @returns The constructed mock stores object.
 */
export function createMockStores(overrides?: MockStoreOverrides) {
	const artifacts = overrides?.artifacts ?? [...FIXTURE_ARTIFACTS];
	const settings = overrides?.projectStore?.settings ?? FIXTURE_PROJECT_SETTINGS;

	const projectStore = {
		hasProject: overrides?.projectStore?.hasProject ?? true,
		projectName: overrides?.projectStore?.projectName ?? "OrqaStudio",
		projectPath: overrides?.projectStore?.projectPath ?? "/mock/project",
		settings,
		statuses: settings.statuses,
		artifactTypes: settings.artifactTypes,
		getStatusConfig(key: string) {
			return settings.statuses.find((s) => s.key === key) ?? null;
		},
		getArtifactTypeConfig(key: string) {
			return settings.artifactTypes.find((t) => t.key === key) ?? null;
		},
	};

	const artifactStore = {
		artifacts,
		getArtifact(id: string) {
			return artifacts.find((a) => a.id === id) ?? null;
		},
		getArtifactsByType(type: string) {
			return artifacts.filter((a) => a.type === type);
		},
		getRelated(id: string, relationshipType?: string) {
			const artifact = artifacts.find((a) => a.id === id);
			if (!artifact) return [];
			const rels = relationshipType
				? artifact.relationships.filter((r) => r.type === relationshipType)
				: artifact.relationships;
			return rels.map((r) => artifacts.find((a) => a.id === r.target)).filter(Boolean);
		},
	};

	const settingsStore = {
		theme: overrides?.settingsStore?.theme ?? "dark",
		model: overrides?.settingsStore?.model ?? "claude-opus-4-6",
	};

	const toastStore = {
		toasts: overrides?.toasts ?? [],
		add(toast: { message: string; type: "info" | "error" | "success" | "warning" }) {
			const id = Math.random().toString(36).slice(2);
			toastStore.toasts = [...toastStore.toasts, { id, ...toast }];
			return id;
		},
		remove(id: string) {
			toastStore.toasts = toastStore.toasts.filter((t) => t.id !== id);
		},
	};

	const navigationStore = {
		currentView: "dashboard",
		currentArtifactId: null as string | null,
		navigate(view: string, artifactId?: string) {
			navigationStore.currentView = view;
			navigationStore.currentArtifactId = artifactId ?? null;
		},
	};

	const stores = {
		projectStore,
		artifactStore,
		settingsStore,
		toastStore,
		navigationStore,
		invoke: async () => null,
	};

	// Register on globalThis so connected components can find stores
	(globalThis as Record<string, unknown>).__orqa_stores = stores;

	return stores;
}

/**
 * Retrieve the mock stores registered on globalThis.__orqa_stores.
 * Throws if createMockStores() has not been called first.
 * @returns The previously registered mock stores object.
 */
export function getStores() {
	const stores = (globalThis as Record<string, unknown>).__orqa_stores;
	if (!stores) {
		throw new Error(
			"OrqaStudio stores not found. Call createMockStores() first, or ensure the app has initialized stores.",
		);
	}
	return stores as ReturnType<typeof createMockStores>;
}
