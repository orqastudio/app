import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import { getStores } from "../registry.svelte.js";
import { logger } from "../logger.js";
import type { NavTree } from "@orqastudio/types";

const log = logger("artifact");

/**
 *
 */
export class ArtifactStore {
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

	/**
	 * Load artifact content for viewing. Delegates to the SDK which reads from disk each time.
	 * @param path - Absolute file path of the artifact to load content for.
	 */
	async loadContent(path: string) {
		this.activeContentLoading = true;
		this.activeContentError = null;
		try {
			const content = await log.perfAsync(`loadContent ${path}`, () =>
				getStores().artifactGraphSDK.readContent(path),
			);
			this.activeContent = content;
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			log.error(`Failed to load content: ${message}`);
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

	/**
	 * Resets all artifact store state to its initial empty values, releasing any loaded content and nav tree.
	 */
	clear() {
		this.navTree = null;
		this.navTreeLoading = false;
		this.navTreeError = null;
		this.activeContent = null;
		this.activeContentLoading = false;
		this.activeContentError = null;
	}
}
