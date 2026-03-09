import { invoke, extractErrorMessage } from "$lib/ipc/invoke";
import type { NavTree } from "$lib/types/nav-tree";

class ArtifactStore {
	// The full navigation tree — loaded once, refreshed by file watcher
	navTree = $state<NavTree | null>(null);
	navTreeLoading = $state(false);
	navTreeError = $state<string | null>(null);

	// Cache for full artifact content when viewing.
	// Capped at MAX_VIEWER_CACHE entries; oldest entry is evicted on overflow.
	private static readonly MAX_VIEWER_CACHE = 50;
	private viewerCache = new Map<string, string>();

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

	/** Load artifact content for viewing */
	async loadContent(path: string) {
		// Check cache first
		const cached = this.viewerCache.get(path);
		if (cached !== undefined) {
			this.activeContent = cached;
			return;
		}

		this.activeContentLoading = true;
		this.activeContentError = null;
		try {
			const result = await invoke<{ content: string }>("read_artifact", { relPath: path });
			if (this.viewerCache.size >= ArtifactStore.MAX_VIEWER_CACHE) {
				// Evict the oldest entry (Map preserves insertion order).
				const oldestKey = this.viewerCache.keys().next().value;
				if (oldestKey !== undefined) this.viewerCache.delete(oldestKey);
			}
			this.viewerCache.set(path, result.content);
			this.activeContent = result.content;
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

	/** Invalidate a specific viewer cache entry */
	invalidateContent(path: string) {
		this.viewerCache.delete(path);
	}

	clear() {
		this.navTree = null;
		this.navTreeLoading = false;
		this.navTreeError = null;
		this.viewerCache.clear();
		this.activeContent = null;
		this.activeContentLoading = false;
		this.activeContentError = null;
	}
}

export const artifactStore = new ArtifactStore();
