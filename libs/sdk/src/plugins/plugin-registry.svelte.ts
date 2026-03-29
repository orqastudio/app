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

import { SvelteMap } from "svelte/reactivity";
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
	 */
	registerPlatformRelationships(relationships: RelationshipType[]): void {
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
	 */
	resolveKey(key: string): string {
		return this.aliasToCanonical.get(key) ?? key;
	}

	/**
	 * Get the project-local alias for a canonical key, or the key itself if no alias.
	 */
	getAlias(canonicalKey: string): string {
		return this.canonicalToAlias.get(canonicalKey) ?? canonicalKey;
	}

	/**
	 * Set an alias for a conflict resolution. Updates the maps immediately.
	 * Caller is responsible for persisting to project.json.
	 */
	setAlias(pluginName: string, type: "schema" | "relationship", canonicalKey: string, alias: string, label?: string): void {
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
	 */
	getPluginConfig(pluginName: string): PluginProjectConfig | null {
		return this.pluginConfigs.get(pluginName) ?? null;
	}

	/**
	 * Get all plugin configs for serialization.
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
	 *
	 * @param manifest - The plugin manifest.
	 * @param components - Map of component keys to Svelte components.
	 * @throws If dependencies unmet, schemas conflict, or relationships conflict.
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

		// Check all conflicts (schemas + relationships)
		const conflicts = this.checkConflicts(manifest);
		if (conflicts.length > 0) {
			const msgs = conflicts.map((c) => c.detail);
			throw new Error(
				`[PluginRegistry] Cannot register "${manifest.name}": ${msgs.join("; ")}`,
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

		// Register relationships
		for (const rel of manifest.provides.relationships) {
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
	}

	/**
	 * Unregister a plugin and remove its schema/relationship ownership.
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

		// Remove relationship ownership
		for (const rel of plugin.manifest.provides.relationships) {
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
	 * Get all relationship types across platform + all plugins.
	 */
	get allRelationships(): RelationshipType[] {
		const seen = new Set<string>();
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
	 */
	getRelationship(key: string): RelationshipType | null {
		return this.relationshipDefs.get(key) ?? null;
	}

	/**
	 * Get the owner of a relationship key ("platform" or plugin name).
	 */
	getRelationshipOwner(key: string): string | null {
		return this.relationshipOwnership.get(key) ?? null;
	}

	/**
	 * Validate that a relationship between two artifact types is allowed.
	 * Returns null if valid, or an error message if invalid.
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
	 * Get all CLI tool registrations across all plugins.
	 */
	get allCliTools(): CliToolRegistration[] {
		const tools: CliToolRegistration[] = [];
		for (const [, plugin] of this.plugins) {
			if (plugin.manifest.provides.cliTools) {
				tools.push(...plugin.manifest.provides.cliTools);
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
	 * Check for all conflicts: schema keys, relationship keys, and
	 * relationship constraint mismatches.
	 */
	checkConflicts(manifest: PluginManifest): RegistrationConflict[] {
		const conflicts: RegistrationConflict[] = [];

		// Schema key conflicts
		for (const schema of manifest.provides.schemas) {
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
		for (const rel of manifest.provides.relationships) {
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
						conflicts.push({
							type: "relationship-constraint",
							key: rel.key,
							existingPlugin: forwardOwner,
							newPlugin: manifest.name,
							detail: `relationship "${rel.key}" has conflicting type constraints — ` +
								`existing: from [${existing.from}] to [${existing.to}], ` +
								`new: from [${rel.from}] to [${rel.to}]`,
						});
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
	 * Resolve a navigation item's label and icon from plugin registrations.
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

	get activeSidecarKey(): string | null {
		if (this.providerConfig.activeSidecar) {
			return this.providerConfig.activeSidecar;
		}
		const first = this.sidecarProviders[0];
		return first ? first.key : null;
	}

	get activeSidecar(): SidecarRegistration | null {
		const key = this.activeSidecarKey;
		if (!key) return null;
		return this.sidecarProviders.find((s) => s.key === key) ?? null;
	}

	setActiveSidecar(key: string): void {
		this.providerConfig = {
			...this.providerConfig,
			activeSidecar: key,
		};
	}

	isSidecarSatisfied(manifest: PluginManifest): boolean {
		if (!manifest.requiresSidecar) return true;
		const required = Array.isArray(manifest.requiresSidecar)
			? manifest.requiresSidecar
			: [manifest.requiresSidecar];
		const available = this.sidecarProviders.map((s) => s.key);
		return required.every((r) => available.includes(r));
	}

	get blockedPlugins(): PluginManifest[] {
		return [];
	}

	// -----------------------------------------------------------------------
	// Settings Pages
	// -----------------------------------------------------------------------

	/**
	 * Get all settings page declarations across all registered plugins.
	 * Used by SettingsCategoryNav to render plugin-contributed settings sections.
	 */
	getSettingsPages(): Array<SettingsPageDeclaration & { pluginName: string }> {
		const pages: Array<SettingsPageDeclaration & { pluginName: string }> = [];
		for (const [name, plugin] of this.plugins) {
			if (plugin.manifest.provides.settings_pages) {
				for (const page of plugin.manifest.provides.settings_pages) {
					pages.push({ ...page, pluginName: name });
				}
			}
		}
		return pages;
	}

	// -----------------------------------------------------------------------
	// Role Definitions
	// -----------------------------------------------------------------------

	/**
	 * Get all role definitions across all registered plugins.
	 * Later registrations override earlier ones for the same role key.
	 */
	get allRoleDefinitions(): RoleDefinition[] {
		const byRole = new Map<string, RoleDefinition>();
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
