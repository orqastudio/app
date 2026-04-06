/**
 * Tests for ArtifactMasterDetail.svelte.
 *
 * ArtifactMasterDetail renders a two-panel layout: ArtifactNav on the left
 * and ArtifactViewer on the right. The right panel shows a placeholder when
 * nothing is selected, or the viewer when selectedArtifactPath is set.
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { installMockStores, clearMockStores } from "../../shared/__tests__/mock-stores.js";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn().mockResolvedValue(() => {}) }));

import ArtifactMasterDetail from "../ArtifactMasterDetail.svelte";

describe("ArtifactMasterDetail", () => {
	beforeEach(() => {
		installMockStores();
	});

	afterEach(() => {
		clearMockStores();
	});

	it("shows placeholder text when no artifact is selected", () => {
		installMockStores({
			navigationStore: {
				...({} as unknown),
				activeActivity: "discovery",
				activeGroup: null,
				selectedArtifactPath: null,
				breadcrumbs: [],
				topLevelNavItems: [],
				showNavPanel: false,
				groupSubCategories: {},
				isArtifactActivity: true,
				searchOpen: false,
				setActivity: vi.fn(),
				setGroup: vi.fn(),
				toggleSearch: vi.fn(),
				navigateToArtifact: vi.fn(),
				openArtifact: vi.fn(),
				getNavType: vi.fn().mockReturnValue(null),
				getLabelForKey: vi.fn().mockReturnValue("Discovery"),
			} as unknown,
		});
		render(ArtifactMasterDetail, { props: { activity: "discovery" } });
		expect(screen.getByText(/Select an item to view it/i)).toBeInTheDocument();
	});
});
