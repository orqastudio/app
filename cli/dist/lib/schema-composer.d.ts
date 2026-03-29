/**
 * Schema composer — generates .orqa/schema.composed.json from installed plugins.
 *
 * Reads provides.schemas from all installed plugin manifests and composes them
 * into a single .orqa/schema.composed.json file. This satisfies P7 (Resolved
 * Workflow Is a File) applied to schema: the composed schema exists as a
 * deterministic, diffable file on disk rather than only in memory.
 *
 * Called by orqa plugin install/refresh after plugins are synced.
 */
/** A single composed artifact type entry in schema.composed.json. */
interface ComposedArtifactType {
    /** ID prefix used in artifact identifiers. */
    id_prefix: string;
    /** Singular display label. */
    label: string;
    /** Plural display label (optional). */
    plural?: string;
    /** Regex pattern for valid IDs. */
    id_pattern: string;
    /** Default path within .orqa/ for artifacts of this type. */
    default_path: string;
    /** Lucide icon name. */
    icon: string;
    /** Plugin that provides this schema. */
    source: string;
    /** Frontmatter field schema split into required and optional. */
    fields: {
        required: Record<string, unknown>;
        optional: Record<string, unknown>;
    };
    /** Whether additional frontmatter properties are allowed. */
    additionalProperties: boolean;
    /** Valid status values for this type. */
    statuses: string[];
    /** Initial status when the artifact is created. */
    initialStatus: string;
    /** Map of state category → list of statuses in that category. */
    stateCategories: Record<string, string[]>;
}
/** The top-level schema.composed.json structure. */
interface ComposedSchema {
    $schema: string;
    version: string;
    generated: boolean;
    generatedAt: string;
    description: string;
    artifactTypes: Record<string, ComposedArtifactType>;
    relationshipTypes: RelationshipTypeSummary[];
}
/** A relationship type entry in the composed schema. */
interface RelationshipTypeSummary {
    key: string;
    inverse: string;
    label: string;
    inverseLabel: string;
    from: string[];
    to: string[];
    description: string;
    semantic?: string;
}
/**
 * Compose the schema from all installed plugin manifests.
 *
 * Iterates all installed plugins, collects provides.schemas entries,
 * and builds a unified ComposedSchema object. Later plugins win on key
 * collision (last-write wins, alphabetical plugin order from listInstalledPlugins).
 * @param projectRoot - Absolute path to the project root.
 * @returns The fully composed schema object.
 */
export declare function composeSchema(projectRoot: string): ComposedSchema;
/**
 * Write the composed schema to .orqa/schema.composed.json.
 *
 * Called by orqa plugin install/refresh after all plugin schemas are loaded.
 * Creates the .orqa/ directory if it does not exist.
 * @param projectRoot - Absolute path to the project root.
 * @returns The absolute path to the written schema file.
 */
export declare function writeComposedSchema(projectRoot: string): string;
export {};
//# sourceMappingURL=schema-composer.d.ts.map