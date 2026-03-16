import type { RelationshipType } from "./plugin.js";

/**
 * Build a bidirectional inverse map from an array of relationship definitions.
 *
 * This replaces the hardcoded INVERSE_MAP constant. Both platform relationships
 * and project/plugin relationships use the same `RelationshipType` shape, so
 * callers merge them before calling this function.
 */
export function buildInverseMap(
	relationships: ReadonlyArray<Pick<RelationshipType, "key" | "inverse">>,
): ReadonlyMap<string, string> {
	const map = new Map<string, string>();
	for (const rel of relationships) {
		map.set(rel.key, rel.inverse);
		if (rel.inverse !== rel.key) {
			map.set(rel.inverse, rel.key);
		}
	}
	return map;
}

/**
 * Check whether a relationship key has a given semantic in the semantics map.
 *
 * Usage: `hasSemanticRole(semantics, "evolves-into", "lineage")` → true
 * This allows checks to query intent ("is this a lineage relationship?")
 * without hardcoding specific relationship keys.
 */
export function hasSemantic(
	semantics: Record<string, { keys: string[] }>,
	relationshipKey: string,
	semanticName: string,
): boolean {
	return semantics[semanticName]?.keys.includes(relationshipKey) ?? false;
}

/**
 * Get all relationship keys for a given semantic category.
 *
 * Usage: `keysForSemantic(semantics, "lineage")` → ["evolves-into", "evolves-from", "merged-into", "merged-from"]
 */
export function keysForSemantic(
	semantics: Record<string, { keys: string[] }>,
	semanticName: string,
): string[] {
	return semantics[semanticName]?.keys ?? [];
}

/**
 * @deprecated Use buildInverseMap() with platform config instead.
 * Kept temporarily for backwards compatibility during migration.
 */
export const INVERSE_MAP: ReadonlyMap<string, string> = new Map([
	["informs", "informed-by"],
	["informed-by", "informs"],
	["evolves-into", "evolves-from"],
	["evolves-from", "evolves-into"],
	["drives", "driven-by"],
	["driven-by", "drives"],
	["governs", "governed-by"],
	["governed-by", "governs"],
	["delivers", "delivered-by"],
	["delivered-by", "delivers"],
	["enforces", "enforced-by"],
	["enforced-by", "enforces"],
	["grounded", "grounded-by"],
	["grounded-by", "grounded"],
	["observes", "observed-by"],
	["observed-by", "observes"],
	["merged-into", "merged-from"],
	["merged-from", "merged-into"],
	["synchronised-with", "synchronised-with"],
]);

/**
 * @deprecated Standalone reference fields are removed in the graph-first model.
 * All references now use the `relationships` frontmatter array.
 */
export const SINGLE_REF_FIELDS = [] as const;

/**
 * @deprecated Standalone reference fields are removed in the graph-first model.
 * All references now use the `relationships` frontmatter array.
 */
export const ARRAY_REF_FIELDS = [] as const;
