/**
 * Plugin loader — discovers and registers installed plugins from project.json.
 *
 * Reads project.json to determine which plugins are installed, loads each
 * manifest via IPC, and registers with the plugin registry. Plugin views
 * are mounted at runtime from the plugin's pre-bundled JavaScript — no
 * compile-time knowledge of plugin components required.
 */

import type { PluginRegistry } from "@orqastudio/sdk";
import { logger } from "@orqastudio/sdk";
import type { PluginManifest } from "@orqastudio/types";
import { invoke } from "@tauri-apps/api/core";

const log = logger("plugins");

interface DiscoveredPlugin {
	name: string;
	version: string;
	display_name: string | null;
	description: string | null;
	path: string;
	source: string;
}

/**
 * Discover and register all installed plugins.
 *
 * Called once during app startup. Discovers plugins via IPC (Rust scans
 * the plugins/ directory), loads manifests, and registers with the
 * plugin registry. View components are loaded on demand when the user
 * navigates to a plugin view route — not at registration time.
 */
export async function registerInstalledPlugins(registry: PluginRegistry): Promise<void> {
	let plugins: DiscoveredPlugin[];
	try {
		plugins = await invoke<DiscoveredPlugin[]>("plugin_list_installed");
		log.info(`Discovered ${plugins.length} plugin(s)`);
	} catch (err) {
		const msg = err instanceof Error ? err.message : JSON.stringify(err);
		log.warn(`plugin_list_installed failed: ${msg}`);
		return;
	}

	for (const plugin of plugins) {
		try {
			const manifest = await invoke<PluginManifest>("plugin_get_manifest", {
				name: plugin.name,
			});
			if ((manifest as Record<string, unknown>).defaultNavigation) {
				log.info(`Plugin "${plugin.name}" has defaultNavigation with ${((manifest as Record<string, unknown>).defaultNavigation as unknown[]).length} items`);
			}

			// Ensure provides exists with default empty arrays — Tauri IPC
			// may omit fields that are empty/null in the Rust struct.
			if (!manifest.provides) {
				(manifest as Record<string, unknown>).provides = {};
			}
			const p = manifest.provides;
			if (!p.schemas) p.schemas = [];
			if (!p.relationships) p.relationships = [];
			if (!p.views) p.views = [];
			if (!p.widgets) p.widgets = [];
			if (!p.workflows) p.workflows = [];
			if (!p.hooks) p.hooks = [];
			if (!p.knowledge) p.knowledge = [];

			// Register the manifest with empty components — views are loaded
			// on demand via the plugin-view route, not compiled in.
			registry.register(manifest, {});
		} catch (err) {
			const msg = err instanceof Error ? err.message : JSON.stringify(err);
			log.error(`Failed to register plugin "${plugin.name}": ${msg}`);
		}
	}
}
