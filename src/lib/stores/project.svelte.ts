import type { Project, ProjectSummary } from "$lib/types";

class ProjectStore {
	activeProject = $state<Project | null>(null);
	projects = $state<ProjectSummary[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);

	get hasProject(): boolean {
		return this.activeProject !== null;
	}

	get artifactCounts(): Record<string, number> {
		if (!this.activeProject) return {};
		const summary = this.projects.find((p) => p.id === this.activeProject?.id);
		return summary ? { total: summary.artifact_count } : {};
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
		this.loading = false;
		this.error = null;
	}
}

export const projectStore = new ProjectStore();
