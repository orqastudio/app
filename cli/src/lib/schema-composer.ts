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
import type { ArtifactSchema } from "@orqastudio/types";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

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
function buildIdPattern(idPrefix: string): string {
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
function deriveInitialStatus(transitions: Record<string, string[]>): string {
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
function buildStateCategoriesFromWorkflows(
  projectRoot: string,
  artifactKey: string,
): Record<string, string[]> {
  const workflowsDir = path.join(projectRoot, ".orqa", "workflows");
  if (!fs.existsSync(workflowsDir)) {
    return {};
  }

  // Look for a resolved workflow that covers this artifact type.
  // Resolved workflows are JSON files. Stage files embed artifact types under
  // artifact_types[key].state_machine.states. Per-type standalone files have
  // artifact_type and states at the top level.
  for (const file of fs.readdirSync(workflowsDir)) {
    if (!file.endsWith(".resolved.json")) continue;
    try {
      const content = fs.readFileSync(path.join(workflowsDir, file), "utf-8");
      const workflow = JSON.parse(content) as Record<string, unknown>;

      // Find state machines that belong to this artifact type.
      // Stage files: artifact_types[artifactKey].state_machine.states
      // Standalone files: artifact_type === artifactKey && states at top level
      let states: Record<string, { category?: string }> | null = null;

      const artifactTypes = workflow["artifact_types"] as Record<string, unknown> | undefined;
      if (artifactTypes && typeof artifactTypes === "object" && artifactKey in artifactTypes) {
        const entry = artifactTypes[artifactKey] as Record<string, unknown> | undefined;
        const sm = entry?.["state_machine"] as Record<string, unknown> | undefined;
        if (sm?.["states"] && typeof sm["states"] === "object") {
          states = sm["states"] as Record<string, { category?: string }>;
        }
      } else if (workflow["artifact_type"] === artifactKey && workflow["states"]) {
        states = workflow["states"] as Record<string, { category?: string }>;
      }

      if (!states) continue;

      // Build a map of category → [statuses] from the state definitions.
      const categories: Record<string, string[]> = {};
      for (const [stateName, stateDef] of Object.entries(states)) {
        const cat = stateDef?.category;
        if (typeof cat === "string") {
          if (!categories[cat]) categories[cat] = [];
          categories[cat].push(stateName);
        }
      }

      if (Object.keys(categories).length > 0) {
        return categories;
      }
    } catch {
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
function splitFields(schema: ArtifactSchema): {
  required: Record<string, unknown>;
  optional: Record<string, unknown>;
} {
  const requiredKeys = new Set(schema.frontmatter.required ?? []);
  const properties = schema.frontmatter.properties ?? {};
  const required: Record<string, unknown> = {};
  const optional: Record<string, unknown> = {};

  for (const [key, def] of Object.entries(properties)) {
    if (requiredKeys.has(key)) {
      required[key] = def;
    } else {
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
export function composeSchema(projectRoot: string): ComposedSchema {
  const artifactTypes: Record<string, ComposedArtifactType> = {};
  const relationshipTypes: RelationshipTypeSummary[] = [];
  const seenRelKeys = new Set<string>();

  for (const plugin of listInstalledPlugins(projectRoot)) {
    let manifest;
    try {
      manifest = readManifest(plugin.path);
    } catch {
      continue;
    }

    // Collect artifact type schemas.
    for (const schema of manifest.provides?.schemas ?? []) {
      const statuses = Object.keys(schema.statusTransitions ?? {});
      const { required, optional } = splitFields(schema);
      const stateCategories = buildStateCategoriesFromWorkflows(
        projectRoot,
        schema.key,
      );

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
      if (seenRelKeys.has(rel.key)) continue;
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
    description:
      "Composed schema generated from all installed plugins. Artifact types, relationship types, and validation constraints sourced from plugin manifests.",
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
export function writeComposedSchema(projectRoot: string): string {
  const composed = composeSchema(projectRoot);
  const outputPath = path.join(projectRoot, ".orqa", "schema.composed.json");
  const outputDir = path.dirname(outputPath);

  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  fs.writeFileSync(outputPath, JSON.stringify(composed, null, 2) + "\n", "utf-8");
  return outputPath;
}
