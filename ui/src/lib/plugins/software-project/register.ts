/**
 * Registration function for the Software Project plugin.
 *
 * The manifest is loaded from the canonical orqa-plugin.json via IPC —
 * no TypeScript copy to maintain. Only the Svelte component bindings
 * are statically imported (components can't be loaded from JSON).
 */

import type { PluginRegistry } from "@orqastudio/sdk";
import type { PluginManifest } from "@orqastudio/types";
import { invoke } from "@tauri-apps/api/core";

// Static imports of view components
import RoadmapView from "$lib/components/roadmap/RoadmapView.svelte";

export const PLUGIN_NAME = "@orqastudio/plugin-software-project";

/**
 * Register the Software Project plugin with the plugin registry.
 *
 * Loads the manifest from the Rust backend (which reads the canonical
 * orqa-plugin.json from disk), then registers with UI component bindings.
 */
export async function registerSoftwareProjectPlugin(registry: PluginRegistry): Promise<void> {
	const manifest = await invoke<PluginManifest>("plugin_get_manifest", {
		name: PLUGIN_NAME,
	});

	registry.register(manifest, {
		roadmap: RoadmapView,
	});
}
