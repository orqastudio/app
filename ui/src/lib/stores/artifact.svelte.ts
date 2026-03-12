import { invoke, extractErrorMessage } from "$lib/ipc/invoke";
import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
import type { NavTree } from "$lib/types/nav-tree";

class ArtifactStore {
	// The full navigation tree — loaded once, refreshed by file watcher
	navTree = $state<NavTree | null>(null);
	navTreeLoading = $state(false);
	navTreeError = $state<string | null>(null);

	// Active viewer state
	activeContent = $state<string | null>(null);
	activeContentLoading = $state(false);
	activeContentError = $state<string | null>(null);

	/** Load the full navigation tree from the backend */
	async loadNavTree() {
		if (this.navTreeLoading) return;
		this.navTreeLoading = true;
		this.navTreeError = null;
		try {
			const tree = await invoke<NavTree>("artifact_scan_tree");
			this.navTree = tree;
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.navTreeError = `Failed to load navigation tree: ${message}`;
		} finally {
			this.navTreeLoading = false;
		}
	}

	/** Load artifact content for viewing. Delegates to the SDK which reads from disk each time. */
	async loadContent(path: string) {
		this.activeContentLoading = true;
		this.activeContentError = null;
		try {
			const content = await artifactGraphSDK.readContent(path);
			this.activeContent = content;
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.activeContentError = `Failed to load content: ${message}`;
			this.activeContent = null;
		} finally {
			this.activeContentLoading = false;
		}
	}

	/** Invalidate nav tree cache (called by file watcher) */
	invalidateNavTree() {
		this.navTree = null;
		this.loadNavTree();
	}

	clear() {
		this.navTree = null;
		this.navTreeLoading = false;
		this.navTreeError = null;
		this.activeContent = null;
		this.activeContentLoading = false;
		this.activeContentError = null;
	}
}

export const artifactStore = new ArtifactStore();
