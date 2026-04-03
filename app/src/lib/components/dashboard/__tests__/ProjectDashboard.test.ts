/**
 * Tests for ProjectDashboard.svelte.
 *
 * ProjectDashboard renders the project header and widget grid when a project
 * is active. It renders an empty state when no project is open.
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { installMockStores, clearMockStores } from "../../shared/__tests__/mock-stores.js";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn().mockResolvedValue(() => {}) }));

import ProjectDashboard from "../ProjectDashboard.svelte";

describe("ProjectDashboard", () => {
	beforeEach(() => {
		installMockStores();
	});

	afterEach(() => {
		clearMockStores();
	});

	it("renders empty state when no project is open", () => {
		installMockStores({
			projectStore: {
				...({} as any),
				hasProject: false,
				activeProject: null,
				projectPath: null,
				projectSettings: null,
				settingsLoaded: false,
				hasSettings: true,
				iconDataUrl: null,
				error: null,
				openProject: vi.fn(),
				closeProject: vi.fn(),
				loadActiveProject: vi.fn(),
				loadProjectSettings: vi.fn(),
				checkIsOrqaProject: vi.fn(),
			} as any,
		});
		render(ProjectDashboard);
		expect(screen.getByText("No project open")).toBeInTheDocument();
	});

	it("renders project name in header when project is active", () => {
		installMockStores({
			projectStore: {
				...({} as any),
				hasProject: true,
				activeProject: { id: 1, name: "OrqaStudio Dev", path: "/code/orqastudio-dev" },
				projectPath: "/code/orqastudio-dev",
				projectSettings: { name: "OrqaStudio Dev", description: "The dev project" },
				settingsLoaded: true,
				hasSettings: true,
				iconDataUrl: null,
				error: null,
				openProject: vi.fn(),
				closeProject: vi.fn(),
				loadActiveProject: vi.fn(),
				loadProjectSettings: vi.fn(),
				checkIsOrqaProject: vi.fn(),
			} as any,
		});
		render(ProjectDashboard);
		expect(screen.getByText("OrqaStudio Dev")).toBeInTheDocument();
	});

	it("renders project description when set", () => {
		installMockStores({
			projectStore: {
				...({} as any),
				hasProject: true,
				activeProject: { id: 1, name: "My Project", path: "/code/my-project" },
				projectPath: "/code/my-project",
				projectSettings: { name: "My Project", description: "A test project description" },
				settingsLoaded: true,
				hasSettings: true,
				iconDataUrl: null,
				error: null,
				openProject: vi.fn(),
				closeProject: vi.fn(),
				loadActiveProject: vi.fn(),
				loadProjectSettings: vi.fn(),
				checkIsOrqaProject: vi.fn(),
			} as any,
		});
		render(ProjectDashboard);
		expect(screen.getByText("A test project description")).toBeInTheDocument();
	});

	it("renders project path when no description is set", () => {
		installMockStores({
			projectStore: {
				...({} as any),
				hasProject: true,
				activeProject: { id: 1, name: "My Project", path: "/code/my-project" },
				projectPath: "/code/my-project",
				projectSettings: { name: "My Project", description: null },
				settingsLoaded: true,
				hasSettings: true,
				iconDataUrl: null,
				error: null,
				openProject: vi.fn(),
				closeProject: vi.fn(),
				loadActiveProject: vi.fn(),
				loadProjectSettings: vi.fn(),
				checkIsOrqaProject: vi.fn(),
			} as any,
		});
		render(ProjectDashboard);
		expect(screen.getByText("/code/my-project")).toBeInTheDocument();
	});

	it("renders project icon when iconDataUrl is set", () => {
		installMockStores({
			projectStore: {
				...({} as any),
				hasProject: true,
				activeProject: { id: 1, name: "Iconic Project", path: "/code/iconic" },
				projectPath: "/code/iconic",
				projectSettings: { name: "Iconic Project", description: null },
				settingsLoaded: true,
				hasSettings: true,
				iconDataUrl: "data:image/png;base64,icon",
				error: null,
				openProject: vi.fn(),
				closeProject: vi.fn(),
				loadActiveProject: vi.fn(),
				loadProjectSettings: vi.fn(),
				checkIsOrqaProject: vi.fn(),
			} as any,
		});
		render(ProjectDashboard);
		const img = screen.getByAltText("Iconic Project");
		expect(img).toHaveAttribute("src", "data:image/png;base64,icon");
	});
});
