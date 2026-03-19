/**
 * Mock store factory for developing and testing components
 * without the OrqaStudio app or Tauri backend.
 *
 * Creates a full OrqaStores-compatible object with sensible defaults
 * and registers it on globalThis.__orqa_stores.
 */

import { FIXTURE_ARTIFACTS, FIXTURE_PROJECT_SETTINGS } from "./fixtures.js";

export interface MockStoreOverrides {
	projectStore?: Partial<{
		hasProject: boolean;
		projectName: string;
		projectPath: string;
		settings: typeof FIXTURE_PROJECT_SETTINGS;
	}>;
	settingsStore?: Partial<{
		theme: "light" | "dark" | "system";
		model: string;
	}>;
	artifacts?: typeof FIXTURE_ARTIFACTS;
	toasts?: Array<{ id: string; message: string; type: "info" | "error" | "success" | "warning" }>;
}

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
			return rels
				.map((r) => artifacts.find((a) => a.id === r.target))
				.filter(Boolean);
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
		invoke: async (_cmd: string, _args?: Record<string, unknown>) => null,
	};

	// Register on globalThis so connected components can find stores
	(globalThis as Record<string, unknown>).__orqa_stores = stores;

	return stores;
}

export function getStores() {
	const stores = (globalThis as Record<string, unknown>).__orqa_stores;
	if (!stores) {
		throw new Error(
			"OrqaStudio stores not found. Call createMockStores() first, or ensure the app has initialized stores.",
		);
	}
	return stores as ReturnType<typeof createMockStores>;
}
