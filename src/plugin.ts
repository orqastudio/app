/**
 * Plugin manifest types for the OrqaStudio plugin system.
 *
 * Plugins declare schemas (artifact type definitions) and views (UI components)
 * separately. A plugin can provide schemas only, views only, or both.
 * View-only plugins declare `requires` on schema plugins.
 */

// ---------------------------------------------------------------------------
// Artifact Schema
// ---------------------------------------------------------------------------

/** Frontmatter field specification for an artifact schema. */
export interface ArtifactSchemaFrontmatter {
	/** Fields that must be present in the frontmatter. */
	required: string[];
	/** Fields that may be present in the frontmatter. */
	optional: string[];
}

/** An artifact type schema provided by a plugin. */
export interface ArtifactSchema {
	/** Unique artifact type key (lowercase, hyphenated). */
	key: string;
	/** Singular display label. */
	label: string;
	/** Plural display label. */
	plural?: string;
	/** Lucide icon name. */
	icon: string;
	/** Default relative path within `.orqa/` where artifacts live. */
	defaultPath: string;
	/** ID prefix for this type (e.g. "EPIC", "TASK"). */
	idPrefix: string;
	/** Frontmatter field specification. */
	frontmatter: ArtifactSchemaFrontmatter;
	/** Status transition rules specific to this type. */
	statusTransitions: Record<string, string[]>;
}

// ---------------------------------------------------------------------------
// View & Widget Registrations
// ---------------------------------------------------------------------------

/** A view registration provided by a plugin. */
export interface ViewRegistration {
	/** Unique view key used in navigation routing. */
	key: string;
	/** Display label for the view. */
	label: string;
	/** Lucide icon name. */
	icon: string;
}

/** A widget registration provided by a plugin. */
export interface WidgetRegistration {
	/** Unique widget key. */
	key: string;
	/** Display label for the widget. */
	label: string;
	/** Lucide icon name. */
	icon: string;
	/** Default dashboard position (row, col). */
	defaultPosition?: { row: number; col: number };
	/** Default grid span (rows, cols). */
	defaultSpan?: { rows: number; cols: number };
}

// ---------------------------------------------------------------------------
// Relationship Type
// ---------------------------------------------------------------------------

/**
 * A typed relationship that can be used in artifact frontmatter.
 *
 * Used by both platform (canonical) relationships and plugin relationships.
 * The integrity validator merges them into one vocabulary.
 */
export interface RelationshipType {
	/** Forward relationship key (e.g. "delivers"). */
	key: string;
	/** Inverse relationship key (e.g. "delivered-by"). */
	inverse: string;
	/** Forward display label (e.g. "Delivers"). */
	label: string;
	/** Inverse display label (e.g. "Delivered By"). */
	inverseLabel: string;
	/** Artifact type keys that can be the source (empty = any). */
	from: string[];
	/** Artifact type keys that can be the target (empty = any). */
	to: string[];
	/** Human-readable description of the relationship. */
	description: string;
}

// ---------------------------------------------------------------------------
// Settings Registration
// ---------------------------------------------------------------------------

/** Settings panel registration for a plugin. */
export interface SettingsRegistration {
	/** Relative path to the Svelte settings component. */
	entrypoint: string;
}

// ---------------------------------------------------------------------------
// Default Navigation
// ---------------------------------------------------------------------------

/** A recommended navigation tree addition from a plugin. */
export interface DefaultNavItem {
	/** Navigation item key. */
	key: string;
	/** Item type: "plugin" for plugin views, "group" for containers. */
	type: "plugin" | "group";
	/** Lucide icon name. */
	icon: string;
	/** Display label (required for groups). */
	label?: string;
	/** Plugin source identifier (required for plugin items). */
	pluginSource?: string;
	/** Children for group items. */
	children?: DefaultNavItem[];
}

// ---------------------------------------------------------------------------
// Plugin Manifest
// ---------------------------------------------------------------------------

/** The full plugin manifest (read from `orqa-plugin.json`). */
export interface PluginManifest {
	/** Package name (e.g. "@orqastudio/plugin-software-project"). */
	name: string;
	/** Semver version string. */
	version: string;
	/** Human-readable display name. */
	displayName?: string;
	/** Short description of what the plugin does. */
	description?: string;
	/** Plugin dependencies — other plugin names that must be loaded first. */
	requires?: string[];
	/** Capabilities this plugin provides. */
	provides: PluginProvides;
	/** Settings panel registration. */
	settings?: SettingsRegistration;
	/** Recommended navigation tree additions. */
	defaultNavigation?: DefaultNavItem[];
}

/** The `provides` block of a plugin manifest. */
export interface PluginProvides {
	/** Artifact type schemas this plugin introduces. */
	schemas: ArtifactSchema[];
	/** Views this plugin registers. */
	views: ViewRegistration[];
	/** Dashboard widgets this plugin registers. */
	widgets: WidgetRegistration[];
	/** Relationship types this plugin introduces. */
	relationships: RelationshipType[];
}

// ---------------------------------------------------------------------------
// Navigation Model (project.json)
// ---------------------------------------------------------------------------

/** Navigation item type. */
export type NavItemType = "builtin" | "plugin" | "group";

/** A single item in the navigation tree stored in project.json. */
export interface NavigationItem {
	/** Unique key for this nav item. */
	key: string;
	/** How this item is resolved. */
	type: NavItemType;
	/** Lucide icon name. */
	icon: string;
	/** Display label (required for groups, optional for others — resolved from registry). */
	label?: string;
	/** Plugin that provides this view (required when type === "plugin"). */
	pluginSource?: string;
	/** Children (only for type === "group"). */
	children?: NavigationItem[];
	/** Whether this item is hidden in the nav. */
	hidden?: boolean;
}

// ---------------------------------------------------------------------------
// Plugin Config (stored in project.json)
// ---------------------------------------------------------------------------

/** Per-plugin configuration stored in project.json under `plugins.<name>`. */
export interface PluginProjectConfig {
	/** Whether this plugin is enabled. */
	enabled: boolean;
	/** Per-relationship overrides (key → enabled). */
	relationships?: Record<string, boolean>;
	/** Plugin-specific settings. */
	config?: Record<string, unknown>;
}
