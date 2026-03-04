import { invoke } from "$lib/ipc/invoke";
import type {
	Project,
	ProjectSummary,
	ProjectSettings,
	ProjectScanResult,
} from "$lib/types";

class ProjectStore {
	activeProject = $state<Project | null>(null);
	projects = $state<ProjectSummary[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);

	// File-based project settings (.orqa/project.json)
	projectSettings = $state<ProjectSettings | null>(null);
	settingsLoaded = $state(false);
	scanning = $state(false);
	iconDataUrl = $state<string | null>(null);

	get hasProject(): boolean {
		return this.activeProject !== null;
	}

	get hasSettings(): boolean {
		return this.projectSettings !== null;
	}

	get projectPath(): string | null {
		return this.activeProject?.path ?? null;
	}

	get artifactCounts(): Record<string, number> {
		if (!this.activeProject) return {};
		const summary = this.projects.find((p) => p.id === this.activeProject?.id);
		return summary ? { total: summary.artifact_count } : {};
	}

	/** Try to restore the last active project on app startup. */
	async loadActiveProject() {
		this.loading = true;
		this.error = null;
		try {
			const project = await invoke<Project | null>("project_get_active");
			if (project) {
				this.activeProject = project;
				await this.loadProjectSettings(project.path);
			}
		} catch {
			// No active project — not an error, user just needs to open one
		} finally {
			this.loading = false;
		}
	}

	/** Open a project by its directory path. Creates a DB record if new. */
	async openProject(path: string) {
		this.loading = true;
		this.error = null;
		try {
			const project = await invoke<Project>("project_open", { path });
			this.activeProject = project;
			await this.loadProjects();
			await this.loadProjectSettings(path);
		} catch (err: unknown) {
			const message = err instanceof Error ? err.message : String(err);
			this.error = `Failed to open project: ${message}`;
		} finally {
			this.loading = false;
		}
	}

	/** Load all known projects. */
	async loadProjects() {
		try {
			const projects = await invoke<ProjectSummary[]>("project_list");
			this.projects = projects;
		} catch {
			// Non-critical
		}
	}

	/** Load project settings from .orqa/project.json */
	async loadProjectSettings(path: string) {
		this.settingsLoaded = false;
		try {
			const settings = await invoke<ProjectSettings | null>(
				"project_settings_read",
				{ path },
			);
			this.projectSettings = settings;
			if (settings?.icon) {
				await this.loadIcon();
			} else {
				this.iconDataUrl = null;
			}
		} catch {
			this.projectSettings = null;
		} finally {
			this.settingsLoaded = true;
		}
	}

	/** Save project settings to .orqa/project.json */
	async saveProjectSettings(path: string, settings: ProjectSettings) {
		try {
			const saved = await invoke<ProjectSettings>(
				"project_settings_write",
				{ path, settings },
			);
			this.projectSettings = saved;
		} catch (err: unknown) {
			const message = err instanceof Error ? err.message : String(err);
			this.error = `Failed to save project settings: ${message}`;
		}
	}

	/** Scan the project filesystem for stack and governance info */
	async scanProject(
		path: string,
		excludedPaths?: string[],
	): Promise<ProjectScanResult | null> {
		this.scanning = true;
		try {
			const result = await invoke<ProjectScanResult>("project_scan", {
				path,
				excluded_paths: excludedPaths ?? null,
			});
			return result;
		} catch (err: unknown) {
			const message = err instanceof Error ? err.message : String(err);
			this.error = `Failed to scan project: ${message}`;
			return null;
		} finally {
			this.scanning = false;
		}
	}

	/** Upload a project icon from a file path */
	async uploadIcon(sourcePath: string) {
		if (!this.projectPath || !this.projectSettings) {
			console.error("[uploadIcon] bail: projectPath=", this.projectPath, "settings=", !!this.projectSettings);
			return;
		}
		try {
			console.log("[uploadIcon] calling project_icon_upload", { project_path: this.projectPath, source_path: sourcePath });
			const filename = await invoke<string>("project_icon_upload", {
				project_path: this.projectPath,
				source_path: sourcePath,
			});
			console.log("[uploadIcon] got filename:", filename);
			this.projectSettings = { ...this.projectSettings, icon: filename };
			await this.saveProjectSettings(this.projectPath, this.projectSettings);
			console.log("[uploadIcon] settings saved, loading icon...");
			await this.loadIcon();
			console.log("[uploadIcon] done, iconDataUrl set:", !!this.iconDataUrl);
		} catch (err: unknown) {
			console.error("[uploadIcon] error:", err);
			const message = err instanceof Error ? err.message : String(err);
			this.error = `Failed to upload icon: ${message}`;
		}
	}

	/** Load the project icon as a data URL */
	async loadIcon() {
		if (!this.projectPath || !this.projectSettings?.icon) {
			console.log("[loadIcon] bail: projectPath=", this.projectPath, "icon=", this.projectSettings?.icon);
			this.iconDataUrl = null;
			return;
		}
		try {
			console.log("[loadIcon] calling project_icon_read", { project_path: this.projectPath, icon_filename: this.projectSettings.icon });
			const dataUrl = await invoke<string>("project_icon_read", {
				project_path: this.projectPath,
				icon_filename: this.projectSettings.icon,
			});
			console.log("[loadIcon] got dataUrl length:", dataUrl?.length);
			this.iconDataUrl = dataUrl;
		} catch (err) {
			console.error("[loadIcon] error:", err);
			this.iconDataUrl = null;
		}
	}

	/** Remove the project icon */
	async removeIcon() {
		if (!this.projectPath || !this.projectSettings) return;
		this.projectSettings = { ...this.projectSettings, icon: null };
		await this.saveProjectSettings(this.projectPath, this.projectSettings);
		this.iconDataUrl = null;
	}

	/** Close the current project, returning to the welcome screen. */
	closeProject() {
		this.activeProject = null;
		this.projectSettings = null;
		this.settingsLoaded = false;
		this.iconDataUrl = null;
		this.error = null;
	}

	setActiveProject(project: Project | null) {
		this.activeProject = project;
		this.error = null;
	}

	setProjects(projects: ProjectSummary[]) {
		this.projects = projects;
	}

	setLoading(loading: boolean) {
		this.loading = loading;
	}

	setError(error: string | null) {
		this.error = error;
		this.loading = false;
	}

	clear() {
		this.activeProject = null;
		this.projects = [];
		this.projectSettings = null;
		this.settingsLoaded = false;
		this.scanning = false;
		this.iconDataUrl = null;
		this.loading = false;
		this.error = null;
	}
}

export const projectStore = new ProjectStore();
