/**
 * Tests for Toolbar.svelte.
 *
 * Toolbar renders the menu bar, window controls, and handles file open/new
 * project actions. Tauri IPC and dialog APIs are fully mocked.
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import { installMockStores, clearMockStores } from "../../shared/__tests__/mock-stores.js";

// vi.mock calls are hoisted — factories must not reference top-level variables.
// Use vi.mocked() after import to get typed references.
vi.mock("@tauri-apps/plugin-dialog", () => ({ open: vi.fn().mockResolvedValue(null) }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn().mockResolvedValue(undefined) }));
vi.mock("@tauri-apps/api/window", () => ({
	getCurrentWindow: vi.fn(() => ({
		startDragging: vi.fn(),
		isMaximized: vi.fn().mockResolvedValue(false),
		maximize: vi.fn(),
		unmaximize: vi.fn(),
		close: vi.fn(),
	})),
}));
vi.mock("@tauri-apps/api/app", () => ({
	getVersion: vi.fn().mockResolvedValue("0.1.0"),
	getName: vi.fn().mockResolvedValue("OrqaStudio"),
}));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn().mockResolvedValue(() => {}) }));
vi.mock("$lib/assets/logo-static.svg", () => ({ default: "logo-static.svg" }));

import Toolbar from "../Toolbar.svelte";
import { invoke } from "@tauri-apps/api/core";

describe("Toolbar", () => {
	beforeEach(() => {
		installMockStores();
	});

	afterEach(() => {
		clearMockStores();
		vi.clearAllMocks();
	});

	it("renders the DevTools button", () => {
		render(Toolbar);
		expect(screen.getByText("DevTools")).toBeInTheDocument();
	});

	it("invokes launch_devtools when DevTools button is clicked", async () => {
		render(Toolbar);
		const devBtn = screen.getByText("DevTools");
		await fireEvent.click(devBtn);
		expect(vi.mocked(invoke)).toHaveBeenCalledWith("launch_devtools");
	});

	it("renders File menu item", () => {
		render(Toolbar);
		expect(screen.getByText("File")).toBeInTheDocument();
	});

	it("renders project icon when iconDataUrl is set", () => {
		installMockStores({
			projectStore: {
				hasProject: true,
				activeProject: { id: 1, name: "My Project", path: "/home/user/my-project" },
				projectPath: "/home/user/my-project",
				projectSettings: null,
				settingsLoaded: true,
				hasSettings: true,
				iconDataUrl: "data:image/png;base64,abc",
				error: null,
				openProject: vi.fn(),
				closeProject: vi.fn(),
				loadActiveProject: vi.fn(),
				loadProjectSettings: vi.fn(),
				checkIsOrqaProject: vi.fn(),
			},
		});
		render(Toolbar);
		const img = screen.getByAltText("OrqaStudio");
		expect(img).toHaveAttribute("src", "data:image/png;base64,abc");
	});

	it("renders fallback logo when no icon is set", () => {
		render(Toolbar);
		const img = screen.getByAltText("OrqaStudio");
		expect(img).toHaveAttribute("src", "logo-static.svg");
	});
});
