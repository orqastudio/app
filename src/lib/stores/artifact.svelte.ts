import type { Artifact, ArtifactSummary, ArtifactType } from "$lib/types";

class ArtifactStore {
	artifacts = $state<ArtifactSummary[]>([]);
	activeArtifact = $state<Artifact | null>(null);
	loading = $state(false);
	error = $state<string | null>(null);
	filterText = $state("");

	get filteredArtifacts(): ArtifactSummary[] {
		if (!this.filterText) return this.artifacts;
		const query = this.filterText.toLowerCase();
		return this.artifacts.filter(
			(a) =>
				a.name.toLowerCase().includes(query) ||
				(a.description?.toLowerCase().includes(query) ?? false),
		);
	}

	artifactsByType(type: ArtifactType): ArtifactSummary[] {
		return this.filteredArtifacts.filter((a) => a.artifact_type === type);
	}

	setArtifacts(artifacts: ArtifactSummary[]) {
		this.artifacts = artifacts;
	}

	setActiveArtifact(artifact: Artifact | null) {
		this.activeArtifact = artifact;
		this.error = null;
	}

	setFilter(text: string) {
		this.filterText = text;
	}

	setLoading(loading: boolean) {
		this.loading = loading;
	}

	setError(error: string | null) {
		this.error = error;
		this.loading = false;
	}

	clear() {
		this.artifacts = [];
		this.activeArtifact = null;
		this.loading = false;
		this.error = null;
		this.filterText = "";
	}
}

export const artifactStore = new ArtifactStore();
