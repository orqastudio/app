import { invoke, extractErrorMessage } from "$lib/ipc/invoke";
import type { Artifact, ArtifactSummary, ArtifactType, DocNode } from "$lib/types";

class ArtifactStore {
	artifacts = $state<ArtifactSummary[]>([]);
	activeArtifact = $state<Artifact | null>(null);
	docTree = $state<DocNode[]>([]);
	researchTree = $state<DocNode[]>([]);
	planTree = $state<DocNode[]>([]);
	docTreeLoading = $state(false);
	researchTreeLoading = $state(false);
	planTreeLoading = $state(false);
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

	async loadGovernanceList(artifactType: string) {
		this.loading = true;
		this.error = null;
		try {
			const results = await invoke<ArtifactSummary[]>("governance_list", {
				artifactType,
			});
			// Merge into artifacts (replace entries of this type, keep others)
			const other = this.artifacts.filter((a) => a.artifact_type !== artifactType);
			this.artifacts = [...other, ...results];
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load artifacts: ${message}`;
		} finally {
			this.loading = false;
		}
	}

	async loadGovernanceArtifact(relPath: string) {
		this.loading = true;
		this.error = null;
		try {
			const artifact = await invoke<Artifact>("governance_read", {
				relPath,
			});
			this.activeArtifact = artifact;
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load artifact: ${message}`;
			this.activeArtifact = null;
		} finally {
			this.loading = false;
		}
	}

	async loadDocTree() {
		this.docTreeLoading = true;
		try {
			this.docTree = await invoke<DocNode[]>("doc_tree_scan");
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load documentation tree: ${message}`;
			this.docTree = [];
		} finally {
			this.docTreeLoading = false;
		}
	}

	async loadResearchTree() {
		this.researchTreeLoading = true;
		try {
			this.researchTree = await invoke<DocNode[]>("research_tree_scan");
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load research tree: ${message}`;
			this.researchTree = [];
		} finally {
			this.researchTreeLoading = false;
		}
	}

	async loadPlanTree() {
		this.planTreeLoading = true;
		try {
			this.planTree = await invoke<DocNode[]>("plan_tree_scan");
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load plans tree: ${message}`;
			this.planTree = [];
		} finally {
			this.planTreeLoading = false;
		}
	}

	async loadPlan(relPath: string) {
		this.loading = true;
		this.error = null;
		try {
			const artifact = await invoke<Artifact>("plan_read", { relPath });
			this.activeArtifact = artifact;
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load plan document: ${message}`;
			this.activeArtifact = null;
		} finally {
			this.loading = false;
		}
	}

	async loadResearch(relPath: string) {
		this.loading = true;
		this.error = null;
		try {
			const artifact = await invoke<Artifact>("research_read", { relPath });
			this.activeArtifact = artifact;
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load research document: ${message}`;
			this.activeArtifact = null;
		} finally {
			this.loading = false;
		}
	}

	async loadDoc(relPath: string) {
		this.loading = true;
		this.error = null;
		try {
			const artifact = await invoke<Artifact>("doc_read", { relPath });
			this.activeArtifact = artifact;
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load document: ${message}`;
			this.activeArtifact = null;
		} finally {
			this.loading = false;
		}
	}

	clear() {
		this.artifacts = [];
		this.activeArtifact = null;
		this.docTree = [];
		this.researchTree = [];
		this.planTree = [];
		this.docTreeLoading = false;
		this.researchTreeLoading = false;
		this.planTreeLoading = false;
		this.loading = false;
		this.error = null;
		this.filterText = "";
	}
}

export const artifactStore = new ArtifactStore();
