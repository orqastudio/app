import type { RelationshipType } from "./plugin.js";

/**
 * Build a bidirectional inverse map from an array of relationship definitions.
 *
 * This replaces the hardcoded INVERSE_MAP constant. Both platform relationships
 * and project/plugin relationships use the same `RelationshipType` shape, so
 * callers merge them before calling this function.
 * @param relationships
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
 * @param semantics
 * @param relationshipKey
 * @param semanticName
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
 * @param semantics
 * @param semanticName
 */
export function keysForSemantic(
	semantics: Readonly<Record<string, { readonly keys: readonly string[] }>>,
	semanticName: string,
): readonly string[] {
	return semantics[semanticName]?.keys ?? [];
}
