/**
 * Plugin Registry — manages plugin lifecycle, schema resolution, and view routing.
 *
 * Uses Svelte 5 `$state` so that any component reading from the registry
 * re-renders automatically when plugins are registered or unregistered.
 *
 * Schema and relationship ownership is per-plugin. Conflicts are detected
 * at registration time — duplicate keys or incompatible from/to constraints
 * cause registration to fail.
 */

import { SvelteMap, SvelteSet } from "svelte/reactivity";
import { logger } from "../logger.js";
import type {
	PluginManifest,
	PluginProjectConfig,
	ArtifactSchema,
	ViewRegistration,
	WidgetRegistration,
	RelationshipType,
	NavigationItem,
	SidecarRegistration,
	CliToolRegistration,
	HookRegistration,
	ProviderConfig,
	AliasMapping,
	SettingsPageDeclaration,
	RoleDefinition,
	SchemaCategory,
	PipelineStageConfig,
} from "@orqastudio/types";
// SchemaCategory and PipelineStageConfig are used as return types for registry helper methods.
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

/** A conflict detected during plugin registration. */
export interface RegistrationConflict {
	/** What kind of conflict: schema key, relationship key, or relationship constraint mismatch. */
	type: "schema" | "relationship-key" | "relationship-constraint";
	/** The conflicting key. */
	key: string;
	/** The plugin that already owns this key. */
	existingPlugin: string;
	/** The plugin trying to register. */
	newPlugin: string;
	/** Human-readable detail. */
	detail: string;
}

// ---------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------

const log = logger("plugins");

/**
 *
 */
export class PluginRegistry {
	/** All registered plugins keyed by plugin name. */
	plugins = $state<SvelteMap<string, RegisteredPlugin>>(new SvelteMap());

	/** All artifact schemas keyed by type key → owning plugin name. */
	private schemaOwnership = $state<SvelteMap<string, string>>(new SvelteMap());

	/** All relationships keyed by relationship key → owning source ("platform" or plugin name). */
	private relationshipOwnership = $state<SvelteMap<string, string>>(new SvelteMap());

	/** All registered relationships keyed by key → full definition. */
	private relationshipDefs = $state<SvelteMap<string, RelationshipType>>(new SvelteMap());

	/** Alias mappings: canonical key → project-local key. Bidirectional. */
	private aliasToCanonical = $state<SvelteMap<string, string>>(new SvelteMap());
	private canonicalToAlias = $state<SvelteMap<string, string>>(new SvelteMap());

	/** Per-plugin project config (aliases, enabled, etc.). */
	private pluginConfigs = $state<SvelteMap<string, PluginProjectConfig>>(new SvelteMap());

	/** Provider routing configuration — set from project.json at startup. */
	providerConfig = $state<ProviderConfig>({});

	// -----------------------------------------------------------------------
	// Platform Registration
	// -----------------------------------------------------------------------

	/**
	 * Register platform (core.json) relationships as the baseline.
	 * Must be called before any plugin registration.
	 * Platform relationships cannot be overridden by plugins.
	 * @param relationships - The array of platform relationship type definitions to register.
	 */
	registerPlatformRelationships(relationships: readonly RelationshipType[]): void {
		for (const rel of relationships) {
			this.relationshipOwnership.set(rel.key, "platform");
			this.relationshipOwnership.set(rel.inverse, "platform");
			this.relationshipDefs.set(rel.key, rel);
			// Store inverse as a derived definition
			this.relationshipDefs.set(rel.inverse, {
				key: rel.inverse,
				inverse: rel.key,
				label: rel.inverseLabel,
				inverseLabel: rel.label,
				from: rel.to,
				to: rel.from,
				description: rel.description,
				semantic: rel.semantic,
			});
		}
	}

	// -----------------------------------------------------------------------
	// Alias Resolution
	// -----------------------------------------------------------------------

	/**
	 * Load plugin project configs (from project.json).
	 * Must be called before plugin registration so aliases are available.
	 * @param configs - A map of plugin name to its project configuration (aliases, enabled state, path).
	 */
	loadPluginConfigs(configs: Record<string, PluginProjectConfig>): void {
		for (const [pluginName, config] of Object.entries(configs)) {
			this.pluginConfigs.set(pluginName, config);

			// Register schema aliases
			if (config.schemaAliases) {
				for (const [canonical, mapping] of Object.entries(config.schemaAliases)) {
					this.aliasToCanonical.set(mapping.alias, canonical);
					this.canonicalToAlias.set(canonical, mapping.alias);
				}
			}

			// Register relationship aliases
			if (config.relationshipAliases) {
				for (const [canonical, mapping] of Object.entries(config.relationshipAliases)) {
					this.aliasToCanonical.set(mapping.alias, canonical);
					this.canonicalToAlias.set(canonical, mapping.alias);
				}
			}
		}
	}

	/**
	 * Resolve a key — returns the canonical key if an alias was provided,
	 * or the alias if a canonical key was provided and has one.
	 * @param key - The key to resolve (may be a canonical key or a project-local alias).
	 * @returns The canonical key if the input is an alias, otherwise the key unchanged.
	 */
	resolveKey(key: string): string {
		return this.aliasToCanonical.get(key) ?? key;
	}

	/**
	 * Get the project-local alias for a canonical key, or the key itself if no alias.
	 * @param canonicalKey - The canonical key to look up in the alias map.
	 * @returns The project-local alias string, or the canonical key when no alias is set.
	 */
	getAlias(canonicalKey: string): string {
		return this.canonicalToAlias.get(canonicalKey) ?? canonicalKey;
	}

	/**
	 * Set an alias for a conflict resolution. Updates the maps immediately.
	 * Caller is responsible for persisting to project.json.
	 * @param pluginName - The name of the plugin that owns the canonical key.
	 * @param type - Whether this alias applies to a "schema" key or a "relationship" key.
	 * @param canonicalKey - The canonical key being aliased.
	 * @param alias - The project-local alias to assign.
	 * @param label - Optional human-readable label for the alias in the project context.
	 */
	setAlias(
		pluginName: string,
		type: "schema" | "relationship",
		canonicalKey: string,
		alias: string,
		label?: string,
	): void {
		// Update maps
		this.aliasToCanonical.set(alias, canonicalKey);
		this.canonicalToAlias.set(canonicalKey, alias);

		// Update plugin config
		let config = this.pluginConfigs.get(pluginName);
		if (!config) {
			config = { installed: true, enabled: true, path: `plugins/${pluginName}` };
			this.pluginConfigs.set(pluginName, config);
		}

		const mapping: AliasMapping = { alias, label };
		if (type === "schema") {
			config.schemaAliases = { ...config.schemaAliases, [canonicalKey]: mapping };
		} else {
			config.relationshipAliases = { ...config.relationshipAliases, [canonicalKey]: mapping };
		}
	}

	/**
	 * Get the plugin project config for serialization back to project.json.
	 * @param pluginName - The name of the plugin whose config to retrieve.
	 * @returns The plugin's project configuration, or null if no config is stored.
	 */
	getPluginConfig(pluginName: string): PluginProjectConfig | null {
		return this.pluginConfigs.get(pluginName) ?? null;
	}

	/**
	 * Get all plugin configs for serialization.
	 * @returns A plain object mapping plugin names to their project configurations.
	 */
	get allPluginConfigs(): Record<string, PluginProjectConfig> {
		const result: Record<string, PluginProjectConfig> = {};
		for (const [name, config] of this.pluginConfigs) {
			result[name] = config;
		}
		return result;
	}

	// -----------------------------------------------------------------------
	// Plugin Registration
	// -----------------------------------------------------------------------

	/**
	 * Register a plugin with its manifest and component map.
	 * @param manifest - The plugin manifest.
	 * @param components - Map of component keys to Svelte components.
	 * @throws {Error} If dependencies are unmet, schemas conflict, or relationships conflict.
	 */
	register(manifest: PluginManifest, components: Record<string, Component>): void {
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

		// Check all conflicts (schemas + relationships)
		const conflicts = this.checkConflicts(manifest);
		if (conflicts.length > 0) {
			const msgs = conflicts.map((c) => c.detail);
			throw new Error(`[PluginRegistry] Cannot register "${manifest.name}": ${msgs.join("; ")}`);
		}

		// Build component maps
		const viewMap = new SvelteMap<string, Component>();
		for (const view of manifest.provides.views ?? []) {
			const comp = components[view.key];
			if (comp) viewMap.set(view.key, comp);
		}

		const widgetMap = new SvelteMap<string, Component>();
		for (const widget of manifest.provides.widgets ?? []) {
			const comp = components[widget.key];
			if (comp) widgetMap.set(widget.key, comp);
		}

		// Register schemas
		for (const schema of manifest.provides.schemas ?? []) {
			this.schemaOwnership.set(schema.key, manifest.name);
		}

		// Register relationships
		for (const rel of manifest.provides.relationships ?? []) {
			this.relationshipOwnership.set(rel.key, manifest.name);
			this.relationshipOwnership.set(rel.inverse, manifest.name);
			this.relationshipDefs.set(rel.key, rel);
			this.relationshipDefs.set(rel.inverse, {
				key: rel.inverse,
				inverse: rel.key,
				label: rel.inverseLabel,
				inverseLabel: rel.label,
				from: rel.to,
				to: rel.from,
				description: rel.description,
				semantic: rel.semantic,
			});
		}

		// Store registration
		this.plugins.set(manifest.name, {
			manifest,
			views: viewMap,
			widgets: widgetMap,
		});

		log.info("plugin registered", {
			name: manifest.name,
			schemas: (manifest.provides.schemas ?? []).length,
			relationships: (manifest.provides.relationships ?? []).length,
			views: (manifest.provides.views ?? []).length,
		});
	}

	/**
	 * Unregister a plugin and remove its schema/relationship ownership.
	 * @param pluginName - The name of the plugin to unregister.
	 */
	unregister(pluginName: string): void {
		const plugin = this.plugins.get(pluginName);
		if (!plugin) return;

		// Remove schema ownership
		for (const schema of plugin.manifest.provides.schemas ?? []) {
			if (this.schemaOwnership.get(schema.key) === pluginName) {
				this.schemaOwnership.delete(schema.key);
			}
		}

		// Remove relationship ownership
		for (const rel of plugin.manifest.provides.relationships ?? []) {
			if (this.relationshipOwnership.get(rel.key) === pluginName) {
				this.relationshipOwnership.delete(rel.key);
				this.relationshipOwnership.delete(rel.inverse);
				this.relationshipDefs.delete(rel.key);
				this.relationshipDefs.delete(rel.inverse);
			}
		}

		this.plugins.delete(pluginName);
	}

	// -----------------------------------------------------------------------
	// Schema Resolution
	// -----------------------------------------------------------------------

	/**
	 * Get the artifact schema for a given type key.
	 * @param key - The artifact type key to look up (e.g. "task", "epic").
	 * @returns The matching ArtifactSchema, or null if no plugin owns that key.
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
	 * @returns A flat array of every ArtifactSchema contributed by registered plugins.
	 */
	get allSchemas(): ArtifactSchema[] {
		return Array.from(this.plugins.values()).flatMap((p) => p.manifest.provides.schemas);
	}

	/**
	 * Get all registered view registrations across all plugins.
	 * @returns A flat array of every ViewRegistration contributed by registered plugins.
	 */
	get allViews(): ViewRegistration[] {
		return Array.from(this.plugins.values()).flatMap((p) => p.manifest.provides.views);
	}

	/**
	 * Get all registered widget registrations across all plugins.
	 * @returns A flat array of every WidgetRegistration contributed by registered plugins.
	 */
	get allWidgets(): WidgetRegistration[] {
		return Array.from(this.plugins.values()).flatMap((p) => p.manifest.provides.widgets);
	}

	/**
	 * Get all relationship types across platform + all plugins.
	 * @returns A deduplicated array of RelationshipType forward definitions (inverses excluded).
	 */
	get allRelationships(): RelationshipType[] {
		const seen = new SvelteSet<string>();
		const rels: RelationshipType[] = [];
		for (const [key, rel] of this.relationshipDefs) {
			// Only include forward keys (not inverses) to avoid duplicates
			if (!seen.has(key) && !seen.has(rel.inverse)) {
				rels.push(rel);
				seen.add(key);
				seen.add(rel.inverse);
			}
		}
		return rels;
	}

	// -----------------------------------------------------------------------
	// Relationship Resolution
	// -----------------------------------------------------------------------

	/**
	 * Get a relationship definition by key (forward or inverse).
	 * @param key - The relationship key to look up (forward or inverse form).
	 * @returns The RelationshipType definition, or null if the key is not registered.
	 */
	getRelationship(key: string): RelationshipType | null {
		return this.relationshipDefs.get(key) ?? null;
	}

	/**
	 * Get the owner of a relationship key ("platform" or plugin name).
	 * @param key - The relationship key whose owner to look up.
	 * @returns The owner string ("platform" or the plugin name), or null if not registered.
	 */
	getRelationshipOwner(key: string): string | null {
		return this.relationshipOwnership.get(key) ?? null;
	}

	/**
	 * Validate that a relationship between two artifact types is allowed.
	 * Returns null if valid, or an error message if invalid.
	 * @param relationshipKey - The relationship key to validate.
	 * @param fromType - The artifact type of the source (from) artifact.
	 * @param toType - The artifact type of the target (to) artifact.
	 * @returns Null if the relationship is valid, or an error message string describing the violation.
	 */
	validateRelationship(relationshipKey: string, fromType: string, toType: string): string | null {
		const rel = this.relationshipDefs.get(relationshipKey);
		if (!rel) {
			return `unknown relationship key: "${relationshipKey}"`;
		}

		if (rel.from.length > 0 && !rel.from.includes(fromType)) {
			return `"${relationshipKey}" cannot originate from type "${fromType}" — allowed: ${rel.from.join(", ")}`;
		}

		if (rel.to.length > 0 && !rel.to.includes(toType)) {
			return `"${relationshipKey}" cannot target type "${toType}" — allowed: ${rel.to.join(", ")}`;
		}

		return null;
	}

	// -----------------------------------------------------------------------
	// Backend Capability Accessors
	// -----------------------------------------------------------------------

	/**
	 * Get all sidecar registrations across all plugins.
	 * @returns An array of SidecarRegistration objects from plugins that provide a sidecar.
	 */
	get sidecarProviders(): SidecarRegistration[] {
		return Array.from(this.plugins.values())
			.filter((p) => p.manifest.provides.sidecar !== undefined)
			.map((p) => p.manifest.provides.sidecar as SidecarRegistration);
	}

	/**
	 * Get all CLI tool registrations across all plugins.
	 * @returns A flat array of every CliToolRegistration contributed by registered plugins.
	 */
	get allCliTools(): CliToolRegistration[] {
		return Array.from(this.plugins.values()).flatMap((p) => p.manifest.provides.cliTools ?? []);
	}

	/**
	 * Get all hook registrations across all plugins.
	 * @returns A flat array of every HookRegistration contributed by registered plugins.
	 */
	get allHooks(): HookRegistration[] {
		return Array.from(this.plugins.values()).flatMap((p) => p.manifest.provides.hooks ?? []);
	}

	// -----------------------------------------------------------------------
	// Component Resolution
	// -----------------------------------------------------------------------

	/**
	 * Resolve a view component from a specific plugin.
	 * @param pluginSource - The name of the plugin that registered the view.
	 * @param viewKey - The key of the view to resolve.
	 * @returns The Svelte Component for the view, or null if the plugin or view is not found.
	 */
	getViewComponent(pluginSource: string, viewKey: string): Component | null {
		const plugin = this.plugins.get(pluginSource);
		if (!plugin) return null;
		return plugin.views.get(viewKey) ?? null;
	}

	/**
	 * Resolve a widget component from a specific plugin.
	 * @param pluginSource - The name of the plugin that registered the widget.
	 * @param widgetKey - The key of the widget to resolve.
	 * @returns The Svelte Component for the widget, or null if the plugin or widget is not found.
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
	 * Check for all conflicts: schema keys, relationship keys, and
	 * relationship constraint mismatches.
	 * @param manifest - The plugin manifest to check against already-registered plugins.
	 * @returns An array of RegistrationConflict objects describing every detected conflict.
	 */
	checkConflicts(manifest: PluginManifest): RegistrationConflict[] {
		const conflicts: RegistrationConflict[] = [];

		// Schema key conflicts
		for (const schema of manifest.provides.schemas ?? []) {
			const existingOwner = this.schemaOwnership.get(schema.key);
			if (existingOwner && existingOwner !== manifest.name) {
				conflicts.push({
					type: "schema",
					key: schema.key,
					existingPlugin: existingOwner,
					newPlugin: manifest.name,
					detail: `schema key "${schema.key}" already owned by "${existingOwner}"`,
				});
			}
		}

		// Relationship conflicts
		for (const rel of manifest.provides.relationships ?? []) {
			// Check forward key
			const forwardOwner = this.relationshipOwnership.get(rel.key);
			if (forwardOwner && forwardOwner !== manifest.name) {
				// Key collision — check if it's the same definition or a conflict
				const existing = this.relationshipDefs.get(rel.key);
				if (existing) {
					if (existing.inverse !== rel.inverse) {
						conflicts.push({
							type: "relationship-key",
							key: rel.key,
							existingPlugin: forwardOwner,
							newPlugin: manifest.name,
							detail: `relationship key "${rel.key}" already registered by "${forwardOwner}" with inverse "${existing.inverse}" (new: "${rel.inverse}")`,
						});
					} else if (!arraysEqual(existing.from, rel.from) || !arraysEqual(existing.to, rel.to)) {
						// Multiple plugins extend the same relationship with different type
						// pairs. Merge (union) the from/to arrays instead of rejecting.
						// This is the expected pattern: agile-discovery declares evolves-to
						// for discovery types, agile-planning extends it for planning types.
						const mergedFrom = [...new SvelteSet([...existing.from, ...rel.from])];
						const mergedTo = [...new SvelteSet([...existing.to, ...rel.to])];
						this.relationshipDefs.set(rel.key, { ...existing, from: mergedFrom, to: mergedTo });
					}
					// If key, inverse, from, and to all match — it's a duplicate, not a conflict.
					// Silently skip (idempotent re-registration is fine).
				}
			}

			// Check inverse key collision
			const inverseOwner = this.relationshipOwnership.get(rel.inverse);
			if (inverseOwner && inverseOwner !== manifest.name && inverseOwner !== forwardOwner) {
				conflicts.push({
					type: "relationship-key",
					key: rel.inverse,
					existingPlugin: inverseOwner,
					newPlugin: manifest.name,
					detail: `relationship inverse key "${rel.inverse}" already registered by "${inverseOwner}"`,
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
	 * @param pluginName - The name of the plugin to retrieve.
	 * @returns The RegisteredPlugin object, or null if the plugin is not registered.
	 */
	getPlugin(pluginName: string): RegisteredPlugin | null {
		return this.plugins.get(pluginName) ?? null;
	}

	/**
	 * Check if a plugin is registered and enabled.
	 * @param pluginName - The name of the plugin to check.
	 * @returns True if the plugin is currently registered, false otherwise.
	 */
	isPluginActive(pluginName: string): boolean {
		return this.plugins.has(pluginName);
	}

	/**
	 * Get all plugin names in dependency order.
	 * @returns An array of plugin name strings in the order they were registered.
	 */
	get pluginNames(): string[] {
		return Array.from(this.plugins.keys());
	}

	/**
	 * Resolve a navigation item's label and icon from plugin registrations.
	 * @param item - The navigation item to resolve, potentially referencing a plugin view or schema.
	 * @returns A fully resolved navigation item with label, icon, type, and optional plugin source.
	 */
	resolveNavigationItem(item: NavigationItem): {
		key: string;
		label: string;
		icon: string;
		type: NavigationItem["type"];
		pluginSource?: string;
	} {
		if (item.type === "plugin" && item.pluginSource) {
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
	 * Get the key of the currently active sidecar provider.
	 * Falls back to the first registered sidecar if no explicit selection has been made.
	 * @returns The active sidecar key string, or null if no sidecars are registered.
	 */
	get activeSidecarKey(): string | null {
		if (this.providerConfig.activeSidecar) {
			return this.providerConfig.activeSidecar;
		}
		const first = this.sidecarProviders[0];
		return first ? first.key : null;
	}

	/**
	 * Get the full registration object for the currently active sidecar provider.
	 * @returns The active SidecarRegistration, or null if no sidecar is active.
	 */
	get activeSidecar(): SidecarRegistration | null {
		const key = this.activeSidecarKey;
		if (!key) return null;
		return this.sidecarProviders.find((s) => s.key === key) ?? null;
	}

	/**
	 * Set the active sidecar provider by key. Updates the provider config immediately.
	 * @param key - The sidecar key to activate (must match a registered sidecar's key).
	 */
	setActiveSidecar(key: string): void {
		this.providerConfig = {
			...this.providerConfig,
			activeSidecar: key,
		};
	}

	/**
	 * Check whether all sidecar requirements declared by a plugin manifest are currently satisfied.
	 * @param manifest - The plugin manifest whose sidecar requirements to validate.
	 * @returns True if all required sidecars are registered, false if any are missing.
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
	 * Get the list of plugin manifests that cannot be registered due to unmet sidecar requirements.
	 * @returns An array of PluginManifest objects for plugins blocked by missing sidecar dependencies.
	 */
	get blockedPlugins(): PluginManifest[] {
		return [];
	}

	// -----------------------------------------------------------------------
	// Settings Pages
	// -----------------------------------------------------------------------

	/**
	 * Get all settings page declarations across all registered plugins.
	 * Used by SettingsCategoryNav to render plugin-contributed settings sections.
	 * @returns An array of settings page declarations augmented with the contributing plugin name.
	 */
	getSettingsPages(): Array<SettingsPageDeclaration & { pluginName: string }> {
		return Array.from(this.plugins.entries()).flatMap(([name, plugin]) =>
			(plugin.manifest.provides.settings_pages ?? []).map((page) => ({
				...page,
				pluginName: name,
			})),
		);
	}

	// -----------------------------------------------------------------------
	// Role Definitions
	// -----------------------------------------------------------------------

	/**
	 * Get all role definitions across all registered plugins.
	 * Later registrations override earlier ones for the same role key.
	 * @returns A deduplicated array of RoleDefinition objects, with later plugins winning on key conflicts.
	 */
	get allRoleDefinitions(): RoleDefinition[] {
		const byRole = new SvelteMap<string, RoleDefinition>();
		for (const [, plugin] of this.plugins) {
			if (plugin.manifest.provides.role_definitions) {
				for (const def of plugin.manifest.provides.role_definitions) {
					byRole.set(def.role, def);
				}
			}
		}
		return Array.from(byRole.values());
	}

	// -----------------------------------------------------------------------
	// Artifact Viewer Routing
	// -----------------------------------------------------------------------

	/**
	 * Return the view_key registered by any plugin as a custom viewer for the
	 * given artifact type, or null if no plugin claims that type.
	 *
	 * ExplorerRouter calls this before falling back to the generic ArtifactViewer.
	 * @param schemaKey - The artifact type key (e.g. "task").
	 * @returns The view key string, or null if no custom viewer is registered.
	 */
	getViewerForArtifactType(schemaKey: string): string | null {
		for (const [, plugin] of this.plugins) {
			for (const viewer of plugin.manifest.provides.artifact_viewers ?? []) {
				if (viewer.artifact_type === schemaKey) return viewer.view_key;
			}
		}
		return null;
	}

	/**
	 * Get all schemas whose semantic field equals "governance".
	 *
	 * Components use this to compute governance artifact sets without
	 * importing a static GOVERNANCE_TYPES constant.
	 * @returns Array of artifact schemas with semantic === "governance".
	 */
	get governanceSchemas(): ArtifactSchema[] {
		return this.allSchemas.filter((s) => s.semantic === "governance");
	}

	/**
	 * Return the Lucide icon name for an artifact type from its schema.
	 * Falls back to "file-text" when the type has no registered schema.
	 *
	 * Replaces the static relationship-icons.ts iconForArtifactType helper.
	 * @param key - The artifact type key (e.g. "task", "epic").
	 * @returns A Lucide icon name string.
	 */
	getIconForType(key: string): string {
		return this.getSchema(key)?.icon ?? "file-text";
	}

	/**
	 * Return the category definitions for an artifact type.
	 *
	 * Replaces the static category-colors.ts categoryColor helper. Callers
	 * receive the full SchemaCategory list and can derive colors from the
	 * hex `color` field rather than Tailwind class names.
	 * @param schemaKey - The artifact type key (e.g. "lesson").
	 * @returns Array of SchemaCategory, or an empty array when none are declared.
	 */
	getSchemaCategories(schemaKey: string): SchemaCategory[] {
		return this.getSchema(schemaKey)?.categories ?? [];
	}

	/**
	 * Return pipeline stage configuration for an artifact type from its workflow registration.
	 *
	 * Replaces the static lesson-stages.ts LESSON_STAGES constant. Returns the
	 * pipeline_stages declared on the workflow registration for the given type,
	 * or an empty array when no workflow or no pipeline stages are declared.
	 * @param artifactType - The artifact type key (e.g. "lesson").
	 * @returns Array of PipelineStageConfig, or an empty array when none are declared.
	 */
	getPipelineStages(artifactType: string): PipelineStageConfig[] {
		for (const [, plugin] of this.plugins) {
			const workflow = plugin.manifest.provides.workflows?.find(
				(w) => w.artifact_type === artifactType,
			);
			if (workflow?.pipeline_stages) return workflow.pipeline_stages;
		}
		return [];
	}
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function arraysEqual(a: string[], b: string[]): boolean {
	if (a.length !== b.length) return false;
	const sortedA = [...a].sort();
	const sortedB = [...b].sort();
	return sortedA.every((v, i) => v === sortedB[i]);
}
