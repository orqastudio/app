/**
 * Plugin Registry — manages plugin lifecycle, schema resolution, and view routing.
 *
 * Uses Svelte 5 `$state` so that any component reading from the registry
 * re-renders automatically when plugins are registered or unregistered.
 *
 * Schema ownership is per-plugin. Views are looked up by (pluginSource, viewKey).
 * Conflicts are detected at registration time and surfaced to the caller.
 */

import { SvelteMap } from "svelte/reactivity";
import type {
	PluginManifest,
	ArtifactSchema,
	ViewRegistration,
	WidgetRegistration,
	RelationshipType,
	NavigationItem,
	SidecarRegistration,
	ToolRegistration,
	HookRegistration,
	ProviderConfig,
} from "@orqastudio/types";
import type { Component } from "svelte";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/** A registered plugin with its manifest and runtime component map. */
export interface RegisteredPlugin {
	manifest: PluginManifest;
	/** View components keyed by view key. */
	views: Map<string, Component>;
	/** Widget components keyed by widget key. */
	widgets: Map<string, Component>;
}

/** A schema conflict between two plugins. */
export interface SchemaConflict {
	key: string;
	existingPlugin: string;
	newPlugin: string;
}

// ---------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------

export class PluginRegistry {
	/** All registered plugins keyed by plugin name. */
	plugins = $state<SvelteMap<string, RegisteredPlugin>>(new SvelteMap());

	/** All artifact schemas keyed by type key → owning plugin name. */
	private schemaOwnership = $state<SvelteMap<string, string>>(new SvelteMap());

	/** Provider routing configuration — set from project.json at startup. */
	providerConfig = $state<ProviderConfig>({});

	// -----------------------------------------------------------------------
	// Registration
	// -----------------------------------------------------------------------

	/**
	 * Register a plugin with its manifest and component map.
	 *
	 * @param manifest - The plugin manifest.
	 * @param components - Map of component keys to Svelte components.
	 *                     Keys should match view/widget keys from the manifest.
	 * @throws If a required dependency is not loaded, or if there are unresolved schema conflicts.
	 */
	register(
		manifest: PluginManifest,
		components: Record<string, Component>,
	): void {
		// Check dependencies
		if (manifest.requires) {
			for (const dep of manifest.requires) {
				if (!this.plugins.has(dep)) {
					throw new Error(
						`[PluginRegistry] Cannot register "${manifest.name}": ` +
						`required plugin "${dep}" is not loaded.`,
					);
				}
			}
		}

		// Check sidecar requirements
		if (manifest.requiresSidecar) {
			const required = Array.isArray(manifest.requiresSidecar)
				? manifest.requiresSidecar
				: [manifest.requiresSidecar];
			const available = this.sidecarProviders.map((s) => s.key);
			const missing = required.filter((r) => !available.includes(r));
			if (missing.length > 0) {
				throw new Error(
					`[PluginRegistry] Cannot register "${manifest.name}": ` +
					`required sidecar(s) not available: ${missing.join(", ")}. ` +
					`Register a sidecar provider first.`,
				);
			}
		}

		// Check schema conflicts
		const conflicts = this.checkConflicts(manifest);
		if (conflicts.length > 0) {
			const msgs = conflicts.map(
				(c) => `schema key "${c.key}" already owned by "${c.existingPlugin}"`,
			);
			throw new Error(
				`[PluginRegistry] Cannot register "${manifest.name}": ${msgs.join(", ")}`,
			);
		}

		// Build component maps
		const viewMap = new Map<string, Component>();
		for (const view of manifest.provides.views) {
			const comp = components[view.key];
			if (comp) viewMap.set(view.key, comp);
		}

		const widgetMap = new Map<string, Component>();
		for (const widget of manifest.provides.widgets) {
			const comp = components[widget.key];
			if (comp) widgetMap.set(widget.key, comp);
		}

		// Register schemas
		for (const schema of manifest.provides.schemas) {
			this.schemaOwnership.set(schema.key, manifest.name);
		}

		// Store registration
		this.plugins.set(manifest.name, {
			manifest,
			views: viewMap,
			widgets: widgetMap,
		});
	}

	/**
	 * Unregister a plugin and remove its schema ownership.
	 */
	unregister(pluginName: string): void {
		const plugin = this.plugins.get(pluginName);
		if (!plugin) return;

		// Remove schema ownership
		for (const schema of plugin.manifest.provides.schemas) {
			if (this.schemaOwnership.get(schema.key) === pluginName) {
				this.schemaOwnership.delete(schema.key);
			}
		}

		this.plugins.delete(pluginName);
	}

	// -----------------------------------------------------------------------
	// Schema Resolution
	// -----------------------------------------------------------------------

	/**
	 * Get the artifact schema for a given type key.
	 * Searches across all registered plugins.
	 */
	getSchema(key: string): ArtifactSchema | null {
		const owner = this.schemaOwnership.get(key);
		if (!owner) return null;

		const plugin = this.plugins.get(owner);
		if (!plugin) return null;

		return plugin.manifest.provides.schemas.find((s) => s.key === key) ?? null;
	}

	/**
	 * Get all registered artifact schemas across all plugins.
	 */
	get allSchemas(): ArtifactSchema[] {
		const schemas: ArtifactSchema[] = [];
		for (const [, plugin] of this.plugins) {
			schemas.push(...plugin.manifest.provides.schemas);
		}
		return schemas;
	}

	/**
	 * Get all registered view registrations across all plugins.
	 */
	get allViews(): ViewRegistration[] {
		const views: ViewRegistration[] = [];
		for (const [, plugin] of this.plugins) {
			views.push(...plugin.manifest.provides.views);
		}
		return views;
	}

	/**
	 * Get all registered widget registrations across all plugins.
	 */
	get allWidgets(): WidgetRegistration[] {
		const widgets: WidgetRegistration[] = [];
		for (const [, plugin] of this.plugins) {
			widgets.push(...plugin.manifest.provides.widgets);
		}
		return widgets;
	}

	/**
	 * Get all relationship types across all plugins.
	 */
	get allRelationships(): RelationshipType[] {
		const rels: RelationshipType[] = [];
		for (const [, plugin] of this.plugins) {
			rels.push(...plugin.manifest.provides.relationships);
		}
		return rels;
	}

	// -----------------------------------------------------------------------
	// Backend Capability Accessors
	// -----------------------------------------------------------------------

	/**
	 * Get all sidecar registrations across all plugins.
	 */
	get sidecarProviders(): SidecarRegistration[] {
		const sidecars: SidecarRegistration[] = [];
		for (const [, plugin] of this.plugins) {
			if (plugin.manifest.provides.sidecar) {
				sidecars.push(plugin.manifest.provides.sidecar);
			}
		}
		return sidecars;
	}

	/**
	 * Get all tool registrations across all plugins.
	 */
	get allTools(): ToolRegistration[] {
		const tools: ToolRegistration[] = [];
		for (const [, plugin] of this.plugins) {
			if (plugin.manifest.provides.tools) {
				tools.push(...plugin.manifest.provides.tools);
			}
		}
		return tools;
	}

	/**
	 * Get all hook registrations across all plugins.
	 */
	get allHooks(): HookRegistration[] {
		const hooks: HookRegistration[] = [];
		for (const [, plugin] of this.plugins) {
			if (plugin.manifest.provides.hooks) {
				hooks.push(...plugin.manifest.provides.hooks);
			}
		}
		return hooks;
	}

	// -----------------------------------------------------------------------
	// Component Resolution
	// -----------------------------------------------------------------------

	/**
	 * Resolve a view component from a specific plugin.
	 */
	getViewComponent(pluginSource: string, viewKey: string): Component | null {
		const plugin = this.plugins.get(pluginSource);
		if (!plugin) return null;
		return plugin.views.get(viewKey) ?? null;
	}

	/**
	 * Resolve a widget component from a specific plugin.
	 */
	getWidgetComponent(pluginSource: string, widgetKey: string): Component | null {
		const plugin = this.plugins.get(pluginSource);
		if (!plugin) return null;
		return plugin.widgets.get(widgetKey) ?? null;
	}

	// -----------------------------------------------------------------------
	// Conflict Detection
	// -----------------------------------------------------------------------

	/**
	 * Check for schema key conflicts between an incoming manifest and existing registrations.
	 */
	checkConflicts(manifest: PluginManifest): SchemaConflict[] {
		const conflicts: SchemaConflict[] = [];
		for (const schema of manifest.provides.schemas) {
			const existingOwner = this.schemaOwnership.get(schema.key);
			if (existingOwner && existingOwner !== manifest.name) {
				conflicts.push({
					key: schema.key,
					existingPlugin: existingOwner,
					newPlugin: manifest.name,
				});
			}
		}
		return conflicts;
	}

	// -----------------------------------------------------------------------
	// Navigation Integration
	// -----------------------------------------------------------------------

	/**
	 * Get a specific plugin registration.
	 */
	getPlugin(pluginName: string): RegisteredPlugin | null {
		return this.plugins.get(pluginName) ?? null;
	}

	/**
	 * Check if a plugin is registered and enabled.
	 */
	isPluginActive(pluginName: string): boolean {
		return this.plugins.has(pluginName);
	}

	/**
	 * Get all plugin names in dependency order.
	 */
	get pluginNames(): string[] {
		return Array.from(this.plugins.keys());
	}

	/**
	 * Build merged artifact config from the navigation tree.
	 * For each nav item, resolves label/icon from platform types or plugin registrations.
	 */
	resolveNavigationItem(item: NavigationItem): {
		key: string;
		label: string;
		icon: string;
		type: NavigationItem["type"];
		pluginSource?: string;
	} {
		if (item.type === "plugin" && item.pluginSource) {
			// Try to resolve label from the plugin's view registration
			const plugin = this.plugins.get(item.pluginSource);
			if (plugin) {
				const view = plugin.manifest.provides.views.find((v) => v.key === item.key);
				if (view) {
					return {
						key: item.key,
						label: item.label ?? view.label,
						icon: item.icon ?? view.icon,
						type: item.type,
						pluginSource: item.pluginSource,
					};
				}
				// Also check schemas for artifact list views
				const schema = plugin.manifest.provides.schemas.find((s) => s.key === item.key);
				if (schema) {
					return {
						key: item.key,
						label: item.label ?? schema.label,
						icon: item.icon ?? schema.icon,
						type: item.type,
						pluginSource: item.pluginSource,
					};
				}
			}
		}

		return {
			key: item.key,
			label: item.label ?? item.key,
			icon: item.icon,
			type: item.type,
			pluginSource: item.pluginSource,
		};
	}

	// -----------------------------------------------------------------------
	// Provider Management
	// -----------------------------------------------------------------------

	/**
	 * Get the currently active sidecar provider key.
	 * Resolves from providerConfig, falling back to the first registered sidecar.
	 */
	get activeSidecarKey(): string | null {
		if (this.providerConfig.activeSidecar) {
			return this.providerConfig.activeSidecar;
		}
		const first = this.sidecarProviders[0];
		return first ? first.key : null;
	}

	/**
	 * Get the active sidecar registration (resolved from config or first available).
	 */
	get activeSidecar(): SidecarRegistration | null {
		const key = this.activeSidecarKey;
		if (!key) return null;
		return this.sidecarProviders.find((s) => s.key === key) ?? null;
	}

	/**
	 * Set the active sidecar provider key.
	 * Updates providerConfig — caller is responsible for persisting to project.json.
	 */
	setActiveSidecar(key: string): void {
		this.providerConfig = {
			...this.providerConfig,
			activeSidecar: key,
		};
	}

	/**
	 * Check if a plugin's sidecar requirements are satisfied by the active provider.
	 */
	isSidecarSatisfied(manifest: PluginManifest): boolean {
		if (!manifest.requiresSidecar) return true;
		const required = Array.isArray(manifest.requiresSidecar)
			? manifest.requiresSidecar
			: [manifest.requiresSidecar];
		const available = this.sidecarProviders.map((s) => s.key);
		return required.every((r) => available.includes(r));
	}

	/**
	 * Get plugins that are blocked due to unsatisfied sidecar requirements.
	 * Useful for showing which plugins need a specific provider to be installed.
	 */
	get blockedPlugins(): PluginManifest[] {
		// This checks manifests that tried to register but couldn't —
		// since failed registrations throw, this is for pre-checking manifests
		// before registration. The caller should use isSidecarSatisfied() directly.
		return [];
	}
}
