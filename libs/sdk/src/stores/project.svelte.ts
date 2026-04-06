import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import { logger } from "../logger.js";
import type {
	Project,
	ProjectSummary,
	ProjectSettings,
	ProjectScanResult,
	ArtifactEntry,
	NavigationItem,
} from "@orqastudio/types";

const log = logger("project");

/**
 * Reactive store managing the active project, project list, settings, icon, and scanning state.
 */
export class ProjectStore {
	activeProject = $state<Project | null>(null);
	projects = $state<ProjectSummary[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);

	// File-based project settings (.orqa/project.json)
	projectSettings = $state<ProjectSettings | null>(null);
	settingsLoaded = $state(false);
	scanning = $state(false);
	iconDataUrl = $state<string | null>(null);

	/**
	 * Returns true when a project is currently open.
	 * @returns Whether an active project is loaded.
	 */
	get hasProject(): boolean {
		return this.activeProject !== null;
	}

	/**
	 * Returns true when project settings have been loaded from disk.
	 * @returns Whether project settings are available.
	 */
	get hasSettings(): boolean {
		return this.projectSettings !== null;
	}

	/**
	 * Returns the filesystem path of the active project, or null if none is open.
	 * @returns The active project path, or null.
	 */
	get projectPath(): string | null {
		return this.activeProject?.path ?? null;
	}

	/**
	 * Returns a summary of artifact counts for the active project from the project list cache.
	 * @returns A record mapping count labels to their numeric values.
	 */
	get artifactCounts(): Record<string, number> {
		if (!this.activeProject) return {};
		const summary = this.projects.find((p) => p.id === this.activeProject?.id);
		return summary ? { total: summary.artifact_count } : {};
	}

	/**
	 * Returns the artifact configuration entries from the loaded project settings.
	 * @returns The array of artifact entries, or an empty array if settings are not loaded.
	 */
	get artifactConfig(): ArtifactEntry[] {
		return this.projectSettings?.artifacts ?? [];
	}

	/**
	 * Navigation tree from project.json. Null if not configured (legacy projects).
	 * @returns The navigation tree array, or null if the project has no explicit navigation config.
	 */
	get navigation(): NavigationItem[] | null {
		return this.projectSettings?.navigation ?? null;
	}

	/**
	 * Whether this project uses the new navigation model.
	 * @returns True when the project has an explicit navigation configuration.
	 */
	get hasNavigation(): boolean {
		return this.navigation !== null;
	}

	/** Try to restore the last active project on app startup. */
	async loadActiveProject() {
		log.info("loadActiveProject: restoring last active project");
		this.loading = true;
		this.error = null;
		try {
			const project = await invoke<Project | null>("project_get_active");
			if (project) {
				this.activeProject = project;
				log.info(`loadActiveProject: restored project at ${project.path}`);
				await this.loadProjectSettings(project.path);
			} else {
				log.info("loadActiveProject: no active project found");
			}
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load active project: ${message}`;
		} finally {
			this.loading = false;
		}
	}

	/**
	 * Open a project by its directory path. Creates a DB record if new.
	 * @param path - Absolute filesystem path to the project root directory.
	 */
	async openProject(path: string) {
		log.info(`openProject: opening ${path}`);
		this.loading = true;
		this.error = null;
		try {
			const project = await invoke<Project>("project_open", { path });
			this.activeProject = project;
			log.info(`openProject: opened project id=${project.id} at ${path}`);
			await this.loadProjects();
			await this.loadProjectSettings(path);
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
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
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load project list: ${message}`;
		}
	}

	/**
	 * Load project settings from .orqa/project.json
	 * @param path - Absolute filesystem path to the project root directory.
	 */
	async loadProjectSettings(path: string) {
		this.settingsLoaded = false;
		try {
			const settings = await invoke<ProjectSettings | null>("project_settings_read", { path });
			this.projectSettings = settings;
			if (settings?.icon) {
				await this.loadIcon();
			} else {
				this.iconDataUrl = null;
			}
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load project settings: ${message}`;
			this.projectSettings = null;
		} finally {
			this.settingsLoaded = true;
		}
	}

	/**
	 * Save project settings to .orqa/project.json
	 * @param path - Absolute filesystem path to the project root directory.
	 * @param settings - The complete project settings object to write.
	 */
	async saveProjectSettings(path: string, settings: ProjectSettings) {
		try {
			const saved = await invoke<ProjectSettings>("project_settings_write", { path, settings });
			this.projectSettings = saved;
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to save project settings: ${message}`;
		}
	}

	/**
	 * Scan the project filesystem for stack and governance info.
	 * @param path - Absolute filesystem path to the project root directory.
	 * @param excludedPaths - Optional list of paths to exclude from the scan.
	 * @returns The scan result, or null if the scan failed.
	 */
	async scanProject(path: string, excludedPaths?: string[]): Promise<ProjectScanResult | null> {
		this.scanning = true;
		try {
			const result = await invoke<ProjectScanResult>("project_scan", {
				path,
				excluded_paths: excludedPaths ?? null,
			});
			return result;
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to scan project: ${message}`;
			return null;
		} finally {
			this.scanning = false;
		}
	}

	/**
	 * Upload a project icon from a file path
	 * @param sourcePath - Absolute path to the image file to use as the project icon.
	 */
	async uploadIcon(sourcePath: string) {
		if (!this.projectPath || !this.projectSettings) {
			return;
		}
		try {
			const filename = await invoke<string>("project_icon_upload", {
				project_path: this.projectPath,
				source_path: sourcePath,
			});
			this.projectSettings = { ...this.projectSettings, icon: filename };
			await this.saveProjectSettings(this.projectPath, this.projectSettings);
			await this.loadIcon();
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to upload icon: ${message}`;
		}
	}

	/** Load the project icon as a data URL */
	async loadIcon() {
		if (!this.projectPath || !this.projectSettings?.icon) {
			this.iconDataUrl = null;
			return;
		}
		try {
			const dataUrl = await invoke<string>("project_icon_read", {
				project_path: this.projectPath,
				icon_filename: this.projectSettings.icon,
			});
			this.iconDataUrl = dataUrl;
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load project icon: ${message}`;
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

	/**
	 * Check whether a directory is an initialized Orqa project.
	 * @param path - Absolute filesystem path to the directory to check.
	 * @returns True if the directory contains valid Orqa project settings.
	 */
	async checkIsOrqaProject(path: string): Promise<boolean> {
		const settings = await invoke<ProjectSettings | null>("project_settings_read", { path });
		return settings !== null;
	}

	/** Close the current project, returning to the welcome screen. */
	closeProject() {
		this.activeProject = null;
		this.projectSettings = null;
		this.settingsLoaded = false;
		this.iconDataUrl = null;
		this.error = null;
	}

	/**
	 * Directly sets the active project and clears any previous error state.
	 * @param project - The project to activate, or null to clear the active project.
	 */
	setActiveProject(project: Project | null) {
		this.activeProject = project;
		this.error = null;
	}

	/**
	 * Directly replaces the full list of known projects.
	 * @param projects - The array of project summaries to set.
	 */
	setProjects(projects: ProjectSummary[]) {
		this.projects = projects;
	}

	/**
	 * Directly sets the loading state flag.
	 * @param loading - True to indicate an async operation is in progress.
	 */
	setLoading(loading: boolean) {
		this.loading = loading;
	}

	/**
	 * Sets an error message and clears the loading flag.
	 * @param error - The error message to display, or null to clear the error.
	 */
	setError(error: string | null) {
		this.error = error;
		this.loading = false;
	}

	/**
	 * Resets all project store state to its initial empty values.
	 */
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
