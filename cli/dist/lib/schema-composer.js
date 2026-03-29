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
import * as fs from "node:fs";
import * as path from "node:path";
import { listInstalledPlugins } from "./installer.js";
import { readManifest } from "./manifest.js";
// ---------------------------------------------------------------------------
// Composition
// ---------------------------------------------------------------------------
/**
 * Build the id_pattern regex from an idPrefix.
 *
 * Pattern format: ^{PREFIX}-[a-f0-9]{8}$
 * @param idPrefix - The artifact ID prefix (e.g. "TASK", "EPIC").
 * @returns The regex pattern string for validating artifact IDs.
 */
function buildIdPattern(idPrefix) {
    const escaped = idPrefix.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    return `^${escaped}-[a-f0-9]{8}$`;
}
/**
 * Derive the initial status from a statusTransitions map.
 *
 * Returns the first key whose values include itself or the first key overall.
 * Falls back to the first key in the map.
 * @param transitions - The status transitions map from the artifact schema.
 * @returns The initial status name.
 */
function deriveInitialStatus(transitions) {
    const keys = Object.keys(transitions);
    return keys[0] ?? "captured";
}
/**
 * Build stateCategories from a workflow's resolved yaml or from status names.
 *
 * Since the composed schema does not have direct access to the workflow
 * state machine categories (those are in the resolved workflow YAML), this
 * function reads resolved workflows from .orqa/workflows/ to enrich the schema.
 * Falls back to empty categories if no workflow is found for this type.
 * @param projectRoot - Absolute path to the project root.
 * @param artifactKey - The artifact type key (e.g. "task", "epic").
 * @returns Map of state category name to list of state names, or empty object if no workflow found.
 */
function buildStateCategoriesFromWorkflows(projectRoot, artifactKey) {
    const workflowsDir = path.join(projectRoot, ".orqa", "workflows");
    if (!fs.existsSync(workflowsDir)) {
        return {};
    }
    // Look for a resolved workflow that covers this artifact type.
    // Resolved workflows are JSON files. Stage files embed artifact types under
    // artifact_types[key].state_machine.states. Per-type standalone files have
    // artifact_type and states at the top level.
    for (const file of fs.readdirSync(workflowsDir)) {
        if (!file.endsWith(".resolved.json"))
            continue;
        try {
            const content = fs.readFileSync(path.join(workflowsDir, file), "utf-8");
            const workflow = JSON.parse(content);
            // Find state machines that belong to this artifact type.
            // Stage files: artifact_types[artifactKey].state_machine.states
            // Standalone files: artifact_type === artifactKey && states at top level
            let states = null;
            const artifactTypes = workflow["artifact_types"];
            if (artifactTypes && typeof artifactTypes === "object" && artifactKey in artifactTypes) {
                const entry = artifactTypes[artifactKey];
                const sm = entry?.["state_machine"];
                if (sm?.["states"] && typeof sm["states"] === "object") {
                    states = sm["states"];
                }
            }
            else if (workflow["artifact_type"] === artifactKey && workflow["states"]) {
                states = workflow["states"];
            }
            if (!states)
                continue;
            // Build a map of category → [statuses] from the state definitions.
            const categories = {};
            for (const [stateName, stateDef] of Object.entries(states)) {
                const cat = stateDef?.category;
                if (typeof cat === "string") {
                    if (!categories[cat])
                        categories[cat] = [];
                    categories[cat].push(stateName);
                }
            }
            if (Object.keys(categories).length > 0) {
                return categories;
            }
        }
        catch {
            // Skip unreadable or unparseable workflow files
        }
    }
    return {};
}
/**
 * Split frontmatter properties into required and optional based on
 * the schema's `required` array.
 * @param schema - The artifact schema to split fields from.
 * @returns Object with required and optional property maps.
 */
function splitFields(schema) {
    const requiredKeys = new Set(schema.frontmatter.required ?? []);
    const properties = schema.frontmatter.properties ?? {};
    const required = {};
    const optional = {};
    for (const [key, def] of Object.entries(properties)) {
        if (requiredKeys.has(key)) {
            required[key] = def;
        }
        else {
            optional[key] = def;
        }
    }
    return { required, optional };
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
export function composeSchema(projectRoot) {
    const artifactTypes = {};
    const relationshipTypes = [];
    const seenRelKeys = new Set();
    for (const plugin of listInstalledPlugins(projectRoot)) {
        let manifest;
        try {
            manifest = readManifest(plugin.path);
        }
        catch {
            continue;
        }
        // Collect artifact type schemas.
        for (const schema of manifest.provides?.schemas ?? []) {
            const statuses = Object.keys(schema.statusTransitions ?? {});
            const { required, optional } = splitFields(schema);
            const stateCategories = buildStateCategoriesFromWorkflows(projectRoot, schema.key);
            artifactTypes[schema.key] = {
                id_prefix: schema.idPrefix,
                label: schema.label,
                ...(schema.plural ? { plural: schema.plural } : {}),
                id_pattern: buildIdPattern(schema.idPrefix),
                default_path: schema.defaultPath.endsWith("/")
                    ? schema.defaultPath
                    : schema.defaultPath + "/",
                icon: schema.icon,
                source: manifest.name,
                fields: { required, optional },
                additionalProperties: schema.frontmatter.additionalProperties ?? true,
                statuses,
                initialStatus: deriveInitialStatus(schema.statusTransitions ?? {}),
                stateCategories,
            };
        }
        // Collect relationship types.
        for (const rel of manifest.provides?.relationships ?? []) {
            if (seenRelKeys.has(rel.key))
                continue;
            seenRelKeys.add(rel.key);
            relationshipTypes.push({
                key: rel.key,
                inverse: rel.inverse,
                label: rel.label,
                inverseLabel: rel.inverseLabel,
                from: rel.from,
                to: rel.to,
                description: rel.description,
                ...(rel.semantic ? { semantic: rel.semantic } : {}),
            });
        }
    }
    return {
        $schema: "https://json-schema.org/draft/2020-12/schema",
        version: "1.0.0",
        generated: true,
        generatedAt: new Date().toISOString(),
        description: "Composed schema generated from all installed plugins. Artifact types, relationship types, and validation constraints sourced from plugin manifests.",
        artifactTypes,
        relationshipTypes,
    };
}
/**
 * Write the composed schema to .orqa/schema.composed.json.
 *
 * Called by orqa plugin install/refresh after all plugin schemas are loaded.
 * Creates the .orqa/ directory if it does not exist.
 * @param projectRoot - Absolute path to the project root.
 * @returns The absolute path to the written schema file.
 */
export function writeComposedSchema(projectRoot) {
    const composed = composeSchema(projectRoot);
    const outputPath = path.join(projectRoot, ".orqa", "schema.composed.json");
    const outputDir = path.dirname(outputPath);
    if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
    }
    fs.writeFileSync(outputPath, JSON.stringify(composed, null, 2) + "\n", "utf-8");
    return outputPath;
}
//# sourceMappingURL=schema-composer.js.map