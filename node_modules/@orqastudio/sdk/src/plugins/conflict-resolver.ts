/**
 * Conflict Resolver — builds AI prompts for resolving plugin conflicts.
 *
 * When two plugins register conflicting schema or relationship keys,
 * this module constructs a prompt for the sidecar to generate resolution
 * suggestions. The AI reads both plugin manifests, understands the
 * semantic difference, and proposes aliases that preserve intent.
 */

import type {
	PluginManifest,
	ConflictResolutionSuggestion,
} from "@orqastudio/types";
import { logger } from "../logger.js";
import type { RegistrationConflict } from "./plugin-registry.svelte.js";

const log = logger("conflict-resolver");

/**
 * Build a system prompt for the AI to resolve plugin conflicts.
 */
export function buildConflictResolutionPrompt(
	conflicts: RegistrationConflict[],
	existingManifest: PluginManifest,
	newManifest: PluginManifest,
	projectContext?: { vision?: string; pillars?: string[] },
): string {
	const conflictDescriptions = conflicts.map((c) => {
		switch (c.type) {
			case "schema":
				return `Schema conflict: both plugins register artifact type "${c.key}"`;
			case "relationship-key":
				return `Relationship conflict: both plugins register relationship "${c.key}"`;
			case "relationship-constraint":
				return `Relationship constraint conflict: "${c.key}" has different from/to types`;
		}
	});

	const contextSection = projectContext
		? `\nProject context:\n- Vision: ${projectContext.vision ?? "not set"}\n- Pillars: ${projectContext.pillars?.join(", ") ?? "not set"}\n`
		: "";

	return `You are resolving naming conflicts between two OrqaStudio plugins that want to register the same artifact type or relationship keys.

## Existing plugin
Name: ${existingManifest.name}
Display: ${existingManifest.displayName ?? existingManifest.name}
Description: ${existingManifest.description ?? "none"}

Schemas: ${existingManifest.provides.schemas.map((s) => `${s.key} (${s.label})`).join(", ") || "none"}
Relationships: ${existingManifest.provides.relationships.map((r) => `${r.key} — ${r.description}`).join("; ") || "none"}

## New plugin being installed
Name: ${newManifest.name}
Display: ${newManifest.displayName ?? newManifest.name}
Description: ${newManifest.description ?? "none"}

Schemas: ${newManifest.provides.schemas.map((s) => `${s.key} (${s.label})`).join(", ") || "none"}
Relationships: ${newManifest.provides.relationships.map((r) => `${r.key} — ${r.description}`).join("; ") || "none"}
${contextSection}
## Conflicts to resolve
${conflictDescriptions.map((d, i) => `${i + 1}. ${d}`).join("\n")}

## Instructions

For each conflict, suggest 2-3 alias options. Each suggestion should:
1. Preserve the semantic meaning of both plugins' definitions
2. Use natural language that reads well in artifact frontmatter
3. Prefer renaming the new plugin's key (less disruption to existing artifacts)
4. Explain why the suggestion fits

Respond with a JSON array of ConflictResolutionSuggestion objects:
\`\`\`json
[
  {
    "key": "the conflicting key",
    "strategy": "rename-new" | "rename-existing" | "rename-both",
    "existingAlias": "suggested alias for existing (if renaming)",
    "newAlias": "suggested alias for new (if renaming)",
    "rationale": "why this works"
  }
]
\`\`\``;
}

/**
 * Parse the AI's response into typed suggestions.
 */
export function parseConflictResolutionResponse(
	response: string,
): ConflictResolutionSuggestion[] {
	// Extract JSON from response (may be wrapped in markdown code block)
	const jsonMatch = response.match(/```json\s*([\s\S]*?)\s*```/) ??
		response.match(/\[[\s\S]*\]/);

	if (!jsonMatch) return [];

	try {
		const json = jsonMatch[1] ?? jsonMatch[0];
		const parsed = JSON.parse(json);
		if (!Array.isArray(parsed)) return [];

		return parsed.filter(
			(s): s is ConflictResolutionSuggestion =>
				typeof s === "object" &&
				s !== null &&
				typeof s.key === "string" &&
				typeof s.strategy === "string" &&
				typeof s.rationale === "string",
		);
	} catch (err: unknown) {
		log.warn("failed to parse conflict resolution response", err);
		return [];
	}
}
