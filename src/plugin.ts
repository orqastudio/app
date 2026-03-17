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
// Backend Capability Registrations
// ---------------------------------------------------------------------------

/** A system binary requirement for a plugin capability. */
export interface SystemRequirement {
	/** Binary name (e.g. "node", "bun", "cargo"). */
	binary: string;
	/** Minimum semver version, if any. */
	minVersion?: string;
}

/** A long-running sidecar process provided by a plugin. */
export interface SidecarRegistration {
	/** Unique sidecar key (e.g. "claude-agent"). */
	key: string;
	/** Display label. */
	label: string;
	/** Runtime used to execute the sidecar. */
	runtime: "node";
	/** Entrypoint path relative to the plugin root (e.g. "dist/sidecar.js"). */
	entrypoint: string;
	/** Additional CLI arguments. */
	args?: string[];
	/** System binaries required to run this sidecar. */
	requires?: SystemRequirement[];
}

/** A one-shot CLI tool (command) provided by a plugin. */
export interface CliToolRegistration {
	/** Unique tool key (e.g. "integrity-check", "eslint"). */
	key: string;
	/** Display label. */
	label: string;
	/** Lucide icon name. */
	icon: string;
	/** Runtime used to execute the tool. */
	runtime: "node" | "system";
	/** Entrypoint path relative to the plugin root. */
	entrypoint: string;
	/** Additional CLI arguments. */
	args?: string[];
	/** Tool category for grouping in the UI. */
	category: "integrity" | "lint" | "test" | "build" | "custom";
	/** Endpoint for AD-042 verification snapshots. */
	dataEndpoint?: string;
}

/** @deprecated Use `CliToolRegistration` instead. */
export type ToolRegistration = CliToolRegistration;

/** A hook provided by a plugin, triggered on specific events. */
export interface HookRegistration {
	/** Unique hook key. */
	key: string;
	/** Event that triggers this hook. */
	event: "pre-commit" | "post-commit" | "pre-push" | "artifact-change" | "session-start";
	/** Runtime used to execute the hook. */
	runtime: "node" | "system";
	/** Entrypoint path relative to the plugin root. */
	entrypoint: string;
	/** Additional CLI arguments. */
	args?: string[];
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
	/** Semantic category (e.g. "lineage", "hierarchy", "governance"). Used by checks to query relationship intent without hardcoding keys. */
	semantic?: string;
}

/** A platform artifact type declaration. */
export interface PlatformArtifactType {
	key: string;
	label: string;
	icon: string;
	idPrefix: string;
}

/** Semantic category grouping relationship keys by intent. */
export interface RelationshipSemantic {
	description: string;
	keys: string[];
}

/** Platform core configuration — loaded from platform/core.json. */
export interface PlatformConfig {
	artifactTypes: PlatformArtifactType[];
	relationships: RelationshipType[];
	semantics: Record<string, RelationshipSemantic>;
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
	/**
	 * Sidecar key(s) this plugin requires at runtime.
	 * If specified, the plugin is only loaded when a matching sidecar provider
	 * is registered and active. Used for capability routing.
	 */
	requiresSidecar?: string | string[];
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
	/** Long-running sidecar process (at most one per plugin). */
	sidecar?: SidecarRegistration;
	/** One-shot CLI tools (commands) this plugin provides. */
	cliTools?: CliToolRegistration[];
	/** @deprecated Use `cliTools` instead. */
	tools?: CliToolRegistration[];
	/** Hooks triggered on specific events. */
	hooks?: HookRegistration[];
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

/** AI provider routing configuration stored in project.json under `providers`. */
export interface ProviderConfig {
	/** The active sidecar provider key (matches a SidecarRegistration.key). */
	activeSidecar?: string;
	/**
	 * Per-capability routing overrides.
	 * Maps capability names to provider keys.
	 * project config > plugin defaults > app baseline.
	 */
	capabilities?: Record<string, string>;
}

// ---------------------------------------------------------------------------
// CLI Tool Runner Results (returned by Tauri commands)
// ---------------------------------------------------------------------------

/** Result of executing a plugin-registered CLI tool. */
export interface CliToolRunResult {
	/** Plugin that owns the tool. */
	plugin: string;
	/** Tool key. */
	tool_key: string;
	/** Exit code (0 = success). */
	exit_code: number;
	/** Captured stdout. */
	stdout: string;
	/** Captured stderr. */
	stderr: string;
	/** Execution duration in milliseconds. */
	duration_ms: number;
	/** Unix timestamp (seconds) when the run completed. */
	completed_at: number;
}

/** @deprecated Use `CliToolRunResult` instead. */
export type ToolRunResult = CliToolRunResult;

/** Status snapshot for a registered CLI tool. */
export interface CliToolRunStatus {
	/** Tool key. */
	tool_key: string;
	/** Plugin that owns the tool. */
	plugin: string;
	/** Display label. */
	label: string;
	/** Whether the last run succeeded. */
	success: boolean | null;
	/** Unix timestamp (seconds) of the last run. */
	last_run: number | null;
	/** Duration of the last run in milliseconds. */
	last_duration_ms: number | null;
	/** Human-readable summary. */
	summary: string | null;
}

/** @deprecated Use `CliToolRunStatus` instead. */
export type ToolRunStatus = CliToolRunStatus;

// ---------------------------------------------------------------------------
// Plugin Registry & Distribution Types
// ---------------------------------------------------------------------------

/** A plugin entry in a registry catalog (official or community). */
export interface RegistryEntry {
	/** Package name (e.g. "@orqastudio/plugin-claude"). */
	name: string;
	/** Human-readable display name. */
	displayName: string;
	/** Short description. */
	description: string;
	/** GitHub repo (owner/repo format). */
	repo: string;
	/** Plugin category for browsing. */
	category: string;
	/** Lucide icon name. */
	icon: string;
	/** Capabilities this plugin provides. */
	capabilities: string[];
	/** System requirements. */
	requires?: Record<string, string>;
}

/** A registry catalog fetched from GitHub. */
export interface RegistryCatalog {
	/** Schema version. */
	version: number;
	/** Source identifier ("official" or "community"). */
	source: string;
	/** Plugin entries. */
	plugins: RegistryEntry[];
}

/** A locked plugin version in plugins.lock.json. */
export interface PluginLockEntry {
	/** Plugin name. */
	name: string;
	/** Installed version. */
	version: string;
	/** GitHub repo source. */
	repo: string;
	/** SHA-256 hash of the installed archive. */
	sha256: string;
	/** Installation timestamp (ISO 8601). */
	installedAt: string;
}

/** Progress event during plugin installation. */
export interface PluginInstallProgress {
	/** Current phase. */
	phase: "downloading" | "extracting" | "validating" | "complete" | "error";
	/** Progress percentage (0-100). */
	percent: number;
	/** Human-readable message. */
	message: string;
}

/** Available update for an installed plugin. */
export interface PluginUpdate {
	/** Plugin name. */
	name: string;
	/** Currently installed version. */
	currentVersion: string;
	/** Latest available version. */
	latestVersion: string;
	/** GitHub repo source. */
	repo: string;
}

/** A discovered plugin from scanning the plugins/ directory. */
export interface DiscoveredPlugin {
	/** Plugin name from manifest. */
	name: string;
	/** Plugin version. */
	version: string;
	/** Display name. */
	displayName?: string;
	/** Description. */
	description?: string;
	/** Filesystem path to the plugin directory. */
	path: string;
	/** Whether this plugin is from a lock file (installed) or local. */
	source: "installed" | "local";
}

// ---------------------------------------------------------------------------
// Hook Generation Results (returned by Tauri commands)
// ---------------------------------------------------------------------------

/** Result of regenerating hook dispatcher scripts. */
export interface HookGenerationResult {
	/** Number of dispatcher scripts written. */
	dispatchers_written: number;
	/** Events that have dispatchers. */
	events: string[];
	/** Hook keys that were included (format: "plugin:hook_key"). */
	hooks: string[];
}
