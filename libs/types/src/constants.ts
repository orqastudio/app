import type { RelationshipType } from "./plugin.js";

/**
 * Build a bidirectional inverse map from an array of relationship definitions.
 *
 * This replaces the hardcoded INVERSE_MAP constant. Both platform relationships
 * and project/plugin relationships use the same `RelationshipType` shape, so
 * callers merge them before calling this function.
 * @param relationships - Array of relationship definitions with key and inverse fields.
 * @returns Immutable map from each relationship key to its inverse key.
 */
export function buildInverseMap(
	relationships: ReadonlyArray<Pick<RelationshipType, "key" | "inverse">>,
): ReadonlyMap<string, string> {
	return new Map(
		relationships.flatMap((rel) =>
			rel.inverse !== rel.key
				? [
						[rel.key, rel.inverse],
						[rel.inverse, rel.key],
					]
				: [[rel.key, rel.inverse]],
		),
	);
}

/**
 * Check whether a relationship key has a given semantic in the semantics map.
 *
 * Usage: `hasSemanticRole(semantics, "evolves-into", "lineage")` → true
 * This allows checks to query intent ("is this a lineage relationship?")
 * without hardcoding specific relationship keys.
 * @param semantics - Map of semantic category names to arrays of relationship keys.
 * @param relationshipKey - The relationship key to test for membership.
 * @param semanticName - The semantic category name to look up.
 * @returns True if the relationship key belongs to the named semantic category.
 */
export function hasSemantic(
	semantics: Readonly<Record<string, { readonly keys: readonly string[] }>>,
	relationshipKey: string,
	semanticName: string,
): boolean {
	return semantics[semanticName]?.keys.includes(relationshipKey) ?? false;
}

/**
 * Get all relationship keys for a given semantic category.
 *
 * Usage: `keysForSemantic(semantics, "lineage")` → ["evolves-into", "evolves-from", "merged-into", "merged-from"]
 * @param semantics - Map of semantic category names to arrays of relationship keys.
 * @param semanticName - The semantic category name whose keys to return.
 * @returns Array of relationship key strings belonging to the category, or empty array if not found.
 */
export function keysForSemantic(
	semantics: Readonly<Record<string, { readonly keys: readonly string[] }>>,
	semanticName: string,
): readonly string[] {
	return semantics[semanticName]?.keys ?? [];
}
