import { invoke } from "$lib/ipc/invoke";
import type { Artifact, ArtifactSummary, ArtifactType, DocNode } from "$lib/types";

class ArtifactStore {
	artifacts = $state<ArtifactSummary[]>([]);
	activeArtifact = $state<Artifact | null>(null);
	docTree = $state<DocNode[]>([]);
	docTreeLoading = $state(false);
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
			const message = err instanceof Error ? err.message : String(err);
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
			const message = err instanceof Error ? err.message : String(err);
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
		} catch {
			this.docTree = [];
		} finally {
			this.docTreeLoading = false;
		}
	}

	async loadDoc(relPath: string) {
		this.loading = true;
		this.error = null;
		try {
			const artifact = await invoke<Artifact>("doc_read", { relPath });
			this.activeArtifact = artifact;
		} catch (err: unknown) {
			const message = err instanceof Error ? err.message : String(err);
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
		this.docTreeLoading = false;
		this.loading = false;
		this.error = null;
		this.filterText = "";
	}
}

export const artifactStore = new ArtifactStore();
