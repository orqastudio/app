/* eslint-disable */
// THIS FILE IS AUTO-GENERATED — DO NOT EDIT BY HAND.
// Source: libs/types/src/platform/*.schema.json
// Regenerate: node scripts/generate-types.mjs

/**
 * Schema for how plugins extend the OrqaStudio core type system. Defines provides.schemas, provides.relationships, and validation rules for plugin manifests.
 */
export type PluginExtensionTypes =
  | PluginManifest
  | PluginArtifactSchema
  | PluginRelationshipExtension
  | PluginManifestProvides
  | PluginInstallValidationResult;

/**
 * The full orqa-plugin.json manifest for an OrqaStudio plugin.
 */
export interface PluginManifest {
  /**
   * JSON Schema reference for editor autocompletion.
   */
  $schema?: string;
  /**
   * Package name. Scoped (@org/name) or unscoped (name).
   */
  name: string;
  /**
   * Semver version string.
   */
  version: string;
  /**
   * Human-readable name shown in the plugin manager UI.
   */
  displayName?: string | null;
  /**
   * Short description of what the plugin does.
   */
  description?: string | null;
  author?: {
    name?: string;
    url?: string | null;
    email?: string | null;
  } | null;
  /**
   * SPDX license identifier.
   */
  license?: string | null;
  /**
   * Plugin dependencies — names of plugins that must be loaded first.
   */
  requires?: string[];
  /**
   * Minimum versions required for this plugin to function.
   */
  compatibility?: {
    app?: string | null;
    sdk?: string | null;
    types?: string | null;
  } | null;
  provides: PluginManifestProvides;
}
/**
 * The 'provides' block of a plugin manifest. Declares all capabilities the plugin registers with the app at install time.
 */
export interface PluginManifestProvides {
  /**
   * Artifact type schemas this plugin introduces. Each schema extends the core type system.
   */
  schemas?: PluginArtifactSchema[];
  /**
   * Relationship types this plugin introduces. Each relationship extends the core relationship vocabulary.
   */
  relationships?: PluginRelationshipExtension[];
  /**
   * Knowledge artifact paths contributed by this plugin.
   */
  knowledge?: string[];
  /**
   * Agent artifact paths contributed by this plugin.
   */
  agents?: string[];
  /**
   * Custom view registrations contributed by this plugin.
   */
  views?: {
    key: string;
    label: string;
    icon: string;
  }[];
  /**
   * Dashboard widget registrations contributed by this plugin.
   */
  widgets?: {
    key: string;
    label: string;
    icon: string;
  }[];
  /**
   * Hook registrations contributed by this plugin.
   */
  hooks?: string[];
}
/**
 * An artifact type schema contributed by a plugin. Plugin schemas must declare key, label, icon, idPrefix, and frontmatter. They may use $ref to reference core types in core.json.
 */
export interface PluginArtifactSchema {
  /**
   * Unique artifact type key (lowercase, hyphenated). MUST NOT collide with core artifact type keys.
   */
  key: string;
  /**
   * Singular display label.
   */
  label: string;
  /**
   * Plural display label. Defaults to label + 's' if omitted.
   */
  plural?: string | null;
  /**
   * Lucide icon name.
   */
  icon: string;
  /**
   * Default relative path within .orqa/ for storing artifacts of this type.
   */
  defaultPath: string;
  /**
   * ID prefix for auto-generated IDs (e.g. 'EPIC', 'TASK'). MUST NOT collide with core ID prefixes.
   */
  idPrefix: string;
  /**
   * Frontmatter field declarations for this artifact type.
   */
  frontmatter: {
    /**
     * Fields that must be present in every artifact of this type.
     */
    required: string[];
    /**
     * Fields that may optionally appear in artifacts of this type.
     */
    optional: string[];
  };
  /**
   * Status transition rules specific to this artifact type.
   */
  statusTransitions?: {
    [k: string]: string[] | undefined;
  } | null;
}
/**
 * A relationship type contributed by a plugin. Relationships MUST include key, inverse, from, to, and semantic to integrate correctly with the validation engine.
 */
export interface PluginRelationshipExtension {
  /**
   * Relationship type key (lowercase, hyphenated). MUST NOT collide with core relationship keys.
   */
  key: string;
  /**
   * Inverse relationship key. May be the same as key for symmetric relationships.
   */
  inverse: string;
  /**
   * Human-readable label for the forward direction.
   */
  label: string;
  /**
   * Human-readable label for the inverse direction.
   */
  inverseLabel: string;
  /**
   * Artifact type keys that are valid sources for this relationship.
   *
   * @minItems 1
   */
  from: [string, ...string[]];
  /**
   * Artifact type keys that are valid targets for this relationship.
   *
   * @minItems 1
   */
  to: [string, ...string[]];
  /**
   * Human-readable explanation of what this relationship means.
   */
  description: string;
  /**
   * The semantic category this relationship belongs to. Determines how the validation engine treats it.
   */
  semantic:
    | "foundation"
    | "lineage"
    | "governance"
    | "knowledge-flow"
    | "agency"
    | "synchronisation"
    | "dependency"
    | "delivery";
  /**
   * Optional validation constraints for this relationship.
   */
  constraints?: {
    /**
     * Whether artifacts of the 'from' types must have at least one of this relationship.
     */
    required?: boolean | null;
    /**
     * Minimum number of this relationship required.
     */
    minCount?: number | null;
    /**
     * Maximum number of this relationship allowed.
     */
    maxCount?: number | null;
    /**
     * Whether the inverse edge must also be present.
     */
    requireInverse?: boolean | null;
  } | null;
}
/**
 * Result of validating a plugin manifest at install time.
 */
export interface PluginInstallValidationResult {
  /**
   * Whether the plugin manifest passed all validation checks.
   */
  valid: boolean;
  /**
   * Blocking validation errors that must be resolved before installation.
   */
  errors: string[];
  /**
   * Non-blocking warnings about the plugin manifest.
   */
  warnings: string[];
  /**
   * Schema or relationship keys that conflict with existing definitions.
   */
  key_collisions?: string[];
}
