/**
 * Tests for ArtifactViewer.svelte.
 *
 * ArtifactViewer reads content from artifactStore and renders frontmatter
 * metadata plus markdown body. It also handles loading, error, and empty states.
 *
 * TooltipRoot/TooltipTrigger/TooltipContent are stubbed because ReferencesPanel
 * and ArtifactLink (connected) use Tooltip without a Provider context.
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { installMockStores, clearMockStores } from "../../shared/__tests__/mock-stores.js";

// Stub tooltip components to bypass bits-ui Provider context requirement
vi.mock("@orqastudio/svelte-components/pure", async (importActual) => {
	const actual = await importActual<typeof import("@orqastudio/svelte-components/pure")>();
	const { default: TooltipRoot } = await import("../../shared/__tests__/stubs/TooltipRoot.svelte");
	const { default: TooltipTrigger } = await import("../../shared/__tests__/stubs/TooltipTrigger.svelte");
	const { default: TooltipContent } = await import("../../shared/__tests__/stubs/TooltipContent.svelte");
	return { ...actual, TooltipRoot, TooltipTrigger, TooltipContent };
});

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn().mockResolvedValue(() => {}) }));

import ArtifactViewer from "../ArtifactViewer.svelte";

describe("ArtifactViewer", () => {
	beforeEach(() => {
		installMockStores();
	});

	afterEach(() => {
		clearMockStores();
	});

	it("renders loading spinner when content is loading", () => {
		installMockStores({
			artifactStore: {
				...({} as any),
				navTree: null,
				activeContent: null,
				activeContentLoading: true,
				activeContentError: null,
				loadNavTree: vi.fn(),
				loadContent: vi.fn(),
				invalidateNavTree: vi.fn(),
			} as any,
		});
		const { container } = render(ArtifactViewer);
		expect(container.querySelector(".flex")).toBeInTheDocument();
	});

	it("renders error message when content load failed", () => {
		installMockStores({
			artifactStore: {
				...({} as any),
				navTree: null,
				activeContent: null,
				activeContentLoading: false,
				activeContentError: "File not found",
				loadNavTree: vi.fn(),
				loadContent: vi.fn(),
				invalidateNavTree: vi.fn(),
			} as any,
		});
		render(ArtifactViewer);
		expect(screen.getByText("File not found")).toBeInTheDocument();
	});

	it("renders empty state prompt when no content is loaded", () => {
		installMockStores({
			artifactStore: {
				...({} as any),
				navTree: null,
				activeContent: null,
				activeContentLoading: false,
				activeContentError: null,
				loadNavTree: vi.fn(),
				loadContent: vi.fn(),
				invalidateNavTree: vi.fn(),
			} as any,
		});
		render(ArtifactViewer);
		expect(screen.getByText(/Select an artifact to view/i)).toBeInTheDocument();
	});

	it("renders markdown content when loaded", () => {
		installMockStores({
			artifactStore: {
				...({} as any),
				navTree: null,
				activeContent: "# Hello World\n\nThis is content.",
				activeContentLoading: false,
				activeContentError: null,
				loadNavTree: vi.fn(),
				loadContent: vi.fn(),
				invalidateNavTree: vi.fn(),
			} as any,
			navigationStore: {
				...({} as any),
				activeActivity: "discovery",
				activeGroup: null,
				selectedArtifactPath: ".orqa/discovery/DOC-001.md",
				breadcrumbs: ["Discovery", "DOC-001"],
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
			} as any,
		});
		render(ArtifactViewer);
		// The markdown renderer produces an h1 element
		expect(screen.getByRole("heading", { level: 1 })).toBeInTheDocument();
	});

	it("renders breadcrumbs when they are set", () => {
		installMockStores({
			artifactStore: {
				...({} as any),
				navTree: null,
				activeContent: "# Doc",
				activeContentLoading: false,
				activeContentError: null,
				loadNavTree: vi.fn(),
				loadContent: vi.fn(),
				invalidateNavTree: vi.fn(),
			} as any,
			navigationStore: {
				...({} as any),
				activeActivity: "discovery",
				activeGroup: null,
				selectedArtifactPath: ".orqa/discovery/DOC-001.md",
				breadcrumbs: ["Discovery", "Research"],
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
			} as any,
		});
		render(ArtifactViewer);
		expect(screen.getByText("Discovery")).toBeInTheDocument();
	});
});
