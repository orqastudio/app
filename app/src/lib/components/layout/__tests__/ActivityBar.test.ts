/**
 * Tests for ActivityBar.svelte.
 *
 * ActivityBar renders navigation items from navigationStore.topLevelNavItems
 * and handles click events to update the active activity/group.
 *
 * TooltipRoot/TooltipTrigger/TooltipContent are stubbed to avoid the
 * bits-ui Tooltip.Provider context requirement in ActivityBarItem.svelte.
 * Because bits-ui's real TooltipTrigger injects `title` into button props,
 * tests verify presence via aria-label query or button count rather than title.
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
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

import ActivityBar from "../ActivityBar.svelte";

const mockNavItems = [
	{ key: "project", type: "activity" as const, label: "Project", icon: "layout-dashboard", hidden: false },
	{ key: "chat", type: "activity" as const, label: "Chat", icon: "message-square", hidden: false },
	{ key: "discovery", type: "group" as const, label: "Discovery", icon: "compass", hidden: false },
	{ key: "artifact-graph", type: "activity" as const, label: "Artifact Graph", icon: "git-graph", hidden: false },
	{ key: "plugins", type: "activity" as const, label: "Plugins", icon: "puzzle", hidden: false },
	{ key: "settings", type: "activity" as const, label: "Settings", icon: "settings", hidden: false },
];

describe("ActivityBar", () => {
	let stores: ReturnType<typeof installMockStores>;

	beforeEach(() => {
		stores = installMockStores({
			navigationStore: {
				...({} as any),
				activeActivity: "project",
				activeGroup: null,
				selectedArtifactPath: null,
				breadcrumbs: [],
				topLevelNavItems: mockNavItems,
				showNavPanel: false,
				groupSubCategories: {},
				isArtifactActivity: false,
				searchOpen: false,
				setActivity: vi.fn(),
				setGroup: vi.fn(),
				toggleSearch: vi.fn(),
				navigateToArtifact: vi.fn(),
				openArtifact: vi.fn(),
				getNavType: vi.fn().mockReturnValue(null),
			} as any,
		});
	});

	afterEach(() => {
		clearMockStores();
	});

	it("renders activity bar buttons for nav items", () => {
		const { container } = render(ActivityBar);
		// Each nav item renders one button via ActivityBarItem
		// project, chat, discovery (main), artifact-graph, search (fixed), plugins, settings = 7 buttons
		const buttons = container.querySelectorAll("button");
		expect(buttons.length).toBeGreaterThanOrEqual(4);
	});

	it("renders the activity bar container", () => {
		const { container } = render(ActivityBar);
		// The outer div with the sidebar styling should be present
		expect(container.querySelector(".flex.w-12")).toBeInTheDocument();
	});

	it("calls setActivity when first main nav item button is clicked", async () => {
		const { container } = render(ActivityBar);
		// The first button is the "project" item (first non-hidden, non-bottom item)
		const buttons = container.querySelectorAll("button");
		expect(buttons.length).toBeGreaterThan(0);
		// Click the first button (project)
		await fireEvent.click(buttons[0]);
		expect(stores.navigationStore.setActivity).toHaveBeenCalledWith("project");
	});

	it("calls setGroup when group nav item button is clicked", async () => {
		const { container } = render(ActivityBar);
		// Buttons are: project(0), chat(1), discovery(2), artifact-graph(3), search(4), plugins(5), settings(6)
		const buttons = container.querySelectorAll("button");
		// "discovery" is at index 2 (0=project, 1=chat, 2=discovery)
		await fireEvent.click(buttons[2]);
		expect(stores.navigationStore.setGroup).toHaveBeenCalledWith("discovery");
	});

	it("calls toggleSearch when search button is clicked", async () => {
		const { container } = render(ActivityBar);
		// search button is at index 4 (0=project, 1=chat, 2=discovery, 3=artifact-graph, 4=search)
		const buttons = container.querySelectorAll("button");
		await fireEvent.click(buttons[4]);
		expect(stores.navigationStore.toggleSearch).toHaveBeenCalled();
	});

	it("does not render hidden nav items", () => {
		installMockStores({
			navigationStore: {
				...({} as any),
				activeActivity: "project",
				activeGroup: null,
				topLevelNavItems: [
					{ key: "project", type: "activity", label: "Project", icon: "layout-dashboard", hidden: false },
					{ key: "hidden-item", type: "activity", label: "Hidden", icon: "eye-off", hidden: true },
					{ key: "settings", type: "activity", label: "Settings", icon: "settings", hidden: false },
				],
				showNavPanel: false,
				groupSubCategories: {},
				isArtifactActivity: false,
				setActivity: vi.fn(),
				setGroup: vi.fn(),
				toggleSearch: vi.fn(),
				navigateToArtifact: vi.fn(),
				openArtifact: vi.fn(),
				getNavType: vi.fn().mockReturnValue(null),
			} as any,
		});
		const { container } = render(ActivityBar);
		// With hidden-item excluded: project(0), search(1), settings(2) = 3 buttons
		// (artifact-graph and plugins are not in the nav items list, so only 3 buttons total)
		const buttons = container.querySelectorAll("button");
		// project + search + settings = 3 buttons (hidden item is excluded)
		expect(buttons.length).toBe(3);
	});

	it("renders correct number of nav buttons (no hidden items)", () => {
		const { container } = render(ActivityBar);
		const buttons = container.querySelectorAll("button");
		// All 6 items present: project, chat, discovery, artifact-graph, search, plugins, settings = 7
		expect(buttons.length).toBe(7);
	});
});
