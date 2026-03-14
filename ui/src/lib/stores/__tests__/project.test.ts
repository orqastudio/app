import { describe, it, expect, beforeEach } from "vitest";
import { mockInvoke } from "./setup";

import { projectStore } from "../project.svelte";
import type { Project, ProjectSummary, ProjectSettings, ProjectScanResult } from "$lib/types";

const fakeProject: Project = {
	id: 1,
	path: "/home/user/my-project",
	name: "My Project",
	description: null,
	detected_stack: null,
	created_at: "2026-01-01T00:00:00Z",
	updated_at: "2026-01-01T00:00:00Z",
};

const fakeSummary: ProjectSummary = {
	id: 1,
	name: "My Project",
	path: "/home/user/my-project",
	detected_stack: null,
	session_count: 0,
	artifact_count: 42,
	updated_at: "2026-01-01T00:00:00Z",
};

const fakeSettings: ProjectSettings = {
	name: "My Project",
	description: "A test project",
	default_model: "auto",
	excluded_paths: [],
	stack: null,
	governance: null,
	icon: null,
	show_thinking: false,
	custom_system_prompt: null,
	artifacts: [
		{ key: "docs", label: "Documentation", path: ".orqa/documentation" },
	],
};

beforeEach(() => {
	mockInvoke.mockReset();
	projectStore.clear();
});

describe("ProjectStore", () => {
	describe("initial state", () => {
		it("starts with no active project", () => {
			expect(projectStore.activeProject).toBeNull();
			expect(projectStore.projects).toEqual([]);
			expect(projectStore.loading).toBe(false);
			expect(projectStore.error).toBeNull();
			expect(projectStore.projectSettings).toBeNull();
			expect(projectStore.settingsLoaded).toBe(false);
		});

		it("hasProject is false initially", () => {
			expect(projectStore.hasProject).toBe(false);
		});

		it("hasSettings is false initially", () => {
			expect(projectStore.hasSettings).toBe(false);
		});

		it("projectPath is null initially", () => {
			expect(projectStore.projectPath).toBeNull();
		});

		it("artifactCounts is empty initially", () => {
			expect(projectStore.artifactCounts).toEqual({});
		});

		it("artifactConfig is empty initially", () => {
			expect(projectStore.artifactConfig).toEqual([]);
		});
	});

	describe("loadActiveProject", () => {
		it("loads active project and its settings", async () => {
			mockInvoke
				.mockResolvedValueOnce(fakeProject) // project_get_active
				.mockResolvedValueOnce(fakeSettings); // project_settings_read

			await projectStore.loadActiveProject();

			expect(mockInvoke).toHaveBeenCalledWith("project_get_active", undefined);
			expect(projectStore.activeProject).toEqual(fakeProject);
			expect(projectStore.projectSettings).toEqual(fakeSettings);
			expect(projectStore.loading).toBe(false);
		});

		it("handles null active project", async () => {
			mockInvoke.mockResolvedValueOnce(null);

			await projectStore.loadActiveProject();

			expect(projectStore.activeProject).toBeNull();
			expect(projectStore.loading).toBe(false);
		});

		it("sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("DB error"));

			await projectStore.loadActiveProject();

			expect(projectStore.error).toContain("DB error");
			expect(projectStore.loading).toBe(false);
		});
	});

	describe("openProject", () => {
		it("opens project and loads settings", async () => {
			mockInvoke
				.mockResolvedValueOnce(fakeProject) // project_open
				.mockResolvedValueOnce([fakeSummary]) // project_list
				.mockResolvedValueOnce(fakeSettings); // project_settings_read

			await projectStore.openProject("/home/user/my-project");

			expect(mockInvoke).toHaveBeenCalledWith("project_open", { path: "/home/user/my-project" });
			expect(projectStore.activeProject).toEqual(fakeProject);
			expect(projectStore.projects).toEqual([fakeSummary]);
		});

		it("sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Not found"));

			await projectStore.openProject("/bad/path");

			expect(projectStore.error).toContain("Not found");
		});
	});

	describe("loadProjects", () => {
		it("loads project list", async () => {
			mockInvoke.mockResolvedValueOnce([fakeSummary]);

			await projectStore.loadProjects();

			expect(mockInvoke).toHaveBeenCalledWith("project_list", undefined);
			expect(projectStore.projects).toEqual([fakeSummary]);
		});
	});

	describe("loadProjectSettings", () => {
		it("loads settings from backend", async () => {
			mockInvoke.mockResolvedValueOnce(fakeSettings);

			await projectStore.loadProjectSettings("/home/user/my-project");

			expect(mockInvoke).toHaveBeenCalledWith("project_settings_read", { path: "/home/user/my-project" });
			expect(projectStore.projectSettings).toEqual(fakeSettings);
			expect(projectStore.settingsLoaded).toBe(true);
		});

		it("loads icon when settings have icon field", async () => {
			const settingsWithIcon = { ...fakeSettings, icon: "logo.png" };
			mockInvoke
				.mockResolvedValueOnce(settingsWithIcon) // project_settings_read
				.mockResolvedValueOnce("data:image/png;base64,abc"); // project_icon_read

			projectStore.activeProject = fakeProject;
			await projectStore.loadProjectSettings(fakeProject.path);

			expect(projectStore.iconDataUrl).toBe("data:image/png;base64,abc");
		});

		it("sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Read failed"));

			await projectStore.loadProjectSettings("/path");

			expect(projectStore.error).toContain("Read failed");
			expect(projectStore.projectSettings).toBeNull();
			expect(projectStore.settingsLoaded).toBe(true);
		});
	});

	describe("saveProjectSettings", () => {
		it("saves settings and updates local state", async () => {
			const updated = { ...fakeSettings, name: "Updated" };
			mockInvoke.mockResolvedValueOnce(updated);

			await projectStore.saveProjectSettings("/path", updated);

			expect(mockInvoke).toHaveBeenCalledWith("project_settings_write", { path: "/path", settings: updated });
			expect(projectStore.projectSettings).toEqual(updated);
		});
	});

	describe("scanProject", () => {
		it("returns scan result", async () => {
			const result: ProjectScanResult = {
				stack: { languages: ["rust", "typescript"], frameworks: [], package_manager: null, has_claude_config: true, has_design_tokens: false },
				governance: { docs: 5, agents: 2, rules: 10, skills: 3, hooks: 1, has_claude_config: true },
				scan_duration_ms: 42,
			};
			mockInvoke.mockResolvedValueOnce(result);

			const scanResult = await projectStore.scanProject("/path");

			expect(mockInvoke).toHaveBeenCalledWith("project_scan", { path: "/path", excluded_paths: null });
			expect(scanResult).toEqual(result);
			expect(projectStore.scanning).toBe(false);
		});

		it("returns null on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Scan failed"));

			const result = await projectStore.scanProject("/path");

			expect(result).toBeNull();
			expect(projectStore.error).toContain("Scan failed");
		});
	});

	describe("checkIsOrqaProject", () => {
		it("returns true when settings exist", async () => {
			mockInvoke.mockResolvedValueOnce(fakeSettings);

			const result = await projectStore.checkIsOrqaProject("/path");

			expect(result).toBe(true);
		});

		it("returns false when settings are null", async () => {
			mockInvoke.mockResolvedValueOnce(null);

			const result = await projectStore.checkIsOrqaProject("/path");

			expect(result).toBe(false);
		});
	});

	describe("closeProject", () => {
		it("clears project state", () => {
			projectStore.activeProject = fakeProject;
			projectStore.projectSettings = fakeSettings;
			projectStore.settingsLoaded = true;
			projectStore.iconDataUrl = "data:image/png;...";

			projectStore.closeProject();

			expect(projectStore.activeProject).toBeNull();
			expect(projectStore.projectSettings).toBeNull();
			expect(projectStore.settingsLoaded).toBe(false);
			expect(projectStore.iconDataUrl).toBeNull();
			expect(projectStore.error).toBeNull();
		});
	});

	describe("derived state", () => {
		it("hasProject reflects active project", () => {
			expect(projectStore.hasProject).toBe(false);
			projectStore.activeProject = fakeProject;
			expect(projectStore.hasProject).toBe(true);
		});

		it("projectPath reflects active project path", () => {
			expect(projectStore.projectPath).toBeNull();
			projectStore.activeProject = fakeProject;
			expect(projectStore.projectPath).toBe("/home/user/my-project");
		});

		it("artifactConfig comes from settings", () => {
			projectStore.projectSettings = fakeSettings;
			expect(projectStore.artifactConfig).toEqual(fakeSettings.artifacts);
		});

		it("artifactCounts comes from matching project summary", () => {
			projectStore.activeProject = fakeProject;
			projectStore.projects = [fakeSummary];
			expect(projectStore.artifactCounts).toEqual({ total: 42 });
		});
	});
});
