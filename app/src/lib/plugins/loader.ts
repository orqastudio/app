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
	readonly name: string;
	readonly version: string;
	readonly display_name: string | null;
	readonly description: string | null;
	readonly path: string;
	readonly source: string;
}

/**
 * Discover and register all installed plugins.
 *
 * Called once during app startup. Discovers plugins via IPC (Rust scans
 * the plugins/ directory), loads manifests, and registers with the
 * plugin registry. View components are loaded on demand when the user
 * navigates to a plugin view route — not at registration time.
 * @param registry
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

	// Load all manifests first, then register in dependency order.
	const manifests = new Map<string, PluginManifest>();
	for (const plugin of plugins) {
		try {
			const manifest = await invoke<PluginManifest>("plugin_get_manifest", {
				name: plugin.name,
			});

			// Ensure provides exists with default empty arrays.
			if (!manifest.provides) {
				(manifest as unknown as Record<string, unknown>).provides = {};
			}
			const p = manifest.provides;
			if (!p.schemas) p.schemas = [];
			if (!p.relationships) p.relationships = [];
			if (!p.views) p.views = [];
			if (!p.widgets) p.widgets = [];
			if (!p.workflows) p.workflows = [];
			if (!p.hooks) p.hooks = [];
			if (!p.knowledge) p.knowledge = [];

			manifests.set(manifest.name, manifest);
		} catch (err) {
			const msg = err instanceof Error ? err.message : JSON.stringify(err);
			log.error(`Failed to load manifest for "${plugin.name}": ${msg}`);
		}
	}

	// Register in dependency order — retry unresolved deps up to N passes.
	const registered = new Set<string>();
	const remaining = new Set(manifests.keys());
	const maxPasses = remaining.size + 1;

	for (let pass = 0; pass < maxPasses && remaining.size > 0; pass++) {
		for (const name of [...remaining]) {
			const manifest = manifests.get(name)!;
			const deps = manifest.requires ?? [];
			const unmet = deps.filter((d) => !registered.has(d) && manifests.has(d));
			if (unmet.length > 0) continue; // retry next pass

			try {
				registry.register(manifest, {});
				registered.add(name);
				remaining.delete(name);
			} catch (err) {
				const msg = err instanceof Error ? err.message : JSON.stringify(err);
				log.error(`Failed to register plugin "${name}": ${msg}`);
				remaining.delete(name); // don't retry broken plugins
			}
		}
	}

	if (remaining.size > 0) {
		log.warn(
			`${remaining.size} plugin(s) could not be registered (unresolved deps): ${[...remaining].join(", ")}`,
		);
	}

	log.info(`Registered ${registered.size}/${manifests.size} plugin(s)`);
}
