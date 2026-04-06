/**
 * Tests for WelcomeScreen.svelte.
 *
 * WelcomeScreen renders when no project is open. It shows the welcome hero
 * and an "Open Project" button that triggers the Tauri file dialog.
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import { installMockStores, clearMockStores } from "../../shared/__tests__/mock-stores.js";

// vi.mock calls are hoisted — factories must not reference top-level variables.
// Use vi.mocked() after import to get typed references.
vi.mock("@tauri-apps/plugin-dialog", () => ({ open: vi.fn().mockResolvedValue(null) }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn().mockResolvedValue(() => {}) }));
vi.mock("$lib/assets/setup-background.png", () => ({ default: "setup-bg.png" }));

import WelcomeScreen from "../WelcomeScreen.svelte";
import { open } from "@tauri-apps/plugin-dialog";

describe("WelcomeScreen", () => {
	let stores: ReturnType<typeof installMockStores>;

	beforeEach(() => {
		stores = installMockStores();
	});

	afterEach(() => {
		clearMockStores();
		vi.clearAllMocks();
	});

	it("renders the welcome title", () => {
		render(WelcomeScreen);
		expect(screen.getByText("Welcome to OrqaStudio")).toBeInTheDocument();
	});

	it("renders the Open Project button", () => {
		render(WelcomeScreen);
		expect(screen.getByRole("button", { name: /Open Project/i })).toBeInTheDocument();
	});

	it("opens Tauri file dialog when Open Project is clicked", async () => {
		render(WelcomeScreen);
		const btn = screen.getByRole("button", { name: /Open Project/i });
		await fireEvent.click(btn);
		expect(vi.mocked(open)).toHaveBeenCalledWith(
			expect.objectContaining({ directory: true, multiple: false }),
		);
	});

	it("calls projectStore.openProject when a directory is selected", async () => {
		vi.mocked(open).mockResolvedValueOnce("/home/user/my-project" as unknown as null);
		render(WelcomeScreen);
		const btn = screen.getByRole("button", { name: /Open Project/i });
		await fireEvent.click(btn);
		// Wait for async handler
		await new Promise((r) => setTimeout(r, 0));
		expect(stores.projectStore.openProject).toHaveBeenCalledWith("/home/user/my-project");
	});

	it("does not call openProject when dialog is cancelled", async () => {
		vi.mocked(open).mockResolvedValueOnce(null);
		render(WelcomeScreen);
		const btn = screen.getByRole("button", { name: /Open Project/i });
		await fireEvent.click(btn);
		await new Promise((r) => setTimeout(r, 0));
		expect(stores.projectStore.openProject).not.toHaveBeenCalled();
	});

	it("renders an error message when projectStore.error is set", () => {
		installMockStores({
			projectStore: {
				hasProject: false,
				activeProject: null,
				projectPath: null,
				projectSettings: null,
				settingsLoaded: false,
				hasSettings: true,
				iconDataUrl: null,
				error: "Failed to open project",
				openProject: vi.fn(),
				closeProject: vi.fn(),
				loadActiveProject: vi.fn(),
				loadProjectSettings: vi.fn(),
				checkIsOrqaProject: vi.fn(),
			},
		});
		render(WelcomeScreen);
		expect(screen.getByText("Failed to open project")).toBeInTheDocument();
	});
});
