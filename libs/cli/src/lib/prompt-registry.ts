/**
 * Prompt registry — scans installed plugins for knowledge declarations and
 * prompt sections, builds a lookup registry, and caches it to disk.
 *
 * Runs at `orqa plugin install` / `orqa plugin refresh` time, similar to
 * the workflow resolver. The runtime prompt pipeline reads only the cached
 * registry — never raw plugin manifests.
 *
 * Five-stage pipeline (this module covers Stage 1 — Plugin Registry):
 *   1. Plugin Registry (this) → 2. Schema Assembly → 3. Section Resolution
 *   → 4. Token Budgeting → 5. Prompt Output
 */

import * as fs from "node:fs";
import * as path from "node:path";
import type {
	KnowledgeDeclaration,
	KnowledgeInjectionTier,
	PromptPriority,
	PromptSection,
	PromptSectionType,
} from "@orqastudio/types";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/** Source priority for conflict resolution (lower number = higher priority). */
export type KnowledgeSource = "project-rules" | "project-knowledge" | "plugin" | "core";

/** A knowledge entry in the registry, enriched with source metadata. */
export interface RegistryKnowledgeEntry {
	/** Unique ID within its plugin (e.g. "rust-error-composition"). */
	id: string;
	/** Plugin name that contributed this entry. */
	plugin: string;
	/** Source category for conflict resolution. */
	source: KnowledgeSource;
	/** Injection tier. */
	tier: KnowledgeInjectionTier;
	/** Agent roles that receive this knowledge. */
	roles: string[];
	/** Workflow stages that trigger injection. */
	stages: string[];
	/** File path glob patterns that trigger injection. */
	paths: string[];
	/** Semantic tags for on-demand retrieval. */
	tags: string[];
	/** Priority level for token budgeting. */
	priority: PromptPriority;
	/** Compressed summary (100-150 tokens). */
	summary: string | null;
	/** Absolute path to the full content file. */
	content_file: string | null;
}

/** A prompt section entry in the registry, enriched with source metadata. */
export interface RegistryPromptSection {
	/** Unique ID within its plugin. */
	id: string;
	/** Plugin name that contributed this section. */
	plugin: string;
	/** Source category for conflict resolution. */
	source: KnowledgeSource;
	/** Section type. */
	type: PromptSectionType;
	/** Agent role this section applies to. */
	role: string | null;
	/** Workflow stage this section applies to. */
	stage: string | null;
	/** Priority level for token budgeting. */
	priority: PromptPriority;
	/** Absolute path to the content file. */
	content_file: string;
}

/** The cached prompt registry written to .orqa/prompt-registry.json. */
export interface PromptRegistry {
	/** Schema version for forward compatibility. */
	version: 1;
	/** Timestamp of registry build. */
	built_at: string;
	/** All knowledge entries from installed plugins. */
	knowledge: RegistryKnowledgeEntry[];
	/** All prompt section entries from installed plugins. */
	sections: RegistryPromptSection[];
	/** Plugins that contributed to this registry. */
	contributors: string[];
	/** Errors encountered during scanning. */
	errors: string[];
}

// ---------------------------------------------------------------------------
// Discovery
// ---------------------------------------------------------------------------

/** Scans installed plugins for knowledge declarations and prompt sections. */
export function scanPluginPromptContributions(projectRoot: string): {
	knowledge: RegistryKnowledgeEntry[];
	sections: RegistryPromptSection[];
	contributors: string[];
	errors: string[];
} {
	const knowledge: RegistryKnowledgeEntry[] = [];
	const sections: RegistryPromptSection[] = [];
	const contributors: string[] = [];
	const errors: string[] = [];

	const containers = ["plugins", "connectors", "integrations"];

	for (const container of containers) {
		const containerDir = path.join(projectRoot, container);
		if (!fs.existsSync(containerDir)) continue;

		let entries: fs.Dirent[];
		try {
			entries = fs.readdirSync(containerDir, { withFileTypes: true });
		} catch {
			continue;
		}

		for (const entry of entries) {
			if (!entry.isDirectory() || entry.name.startsWith(".")) continue;

			const pluginDir = path.join(containerDir, entry.name);
			const manifestPath = path.join(pluginDir, "orqa-plugin.json");

			if (!fs.existsSync(manifestPath)) continue;

			let manifest: Record<string, unknown>;
			try {
				manifest = JSON.parse(fs.readFileSync(manifestPath, "utf-8"));
			} catch (err) {
				errors.push(
					`${manifestPath}: ${err instanceof Error ? err.message : String(err)}`,
				);
				continue;
			}

			const pluginName = (manifest.name as string) ?? entry.name;
			const provides = manifest.provides as Record<string, unknown> | undefined;
			if (!provides) continue;

			const source = classifySource(manifest);
			let contributed = false;

			// Scan knowledge_declarations
			const declarations = provides.knowledge_declarations as
				| KnowledgeDeclaration[]
				| undefined;
			if (Array.isArray(declarations)) {
				for (const decl of declarations) {
					const contentFile = decl.content_file
						? path.join(pluginDir, decl.content_file)
						: null;

					knowledge.push({
						id: decl.id,
						plugin: pluginName,
						source,
						tier: decl.tier,
						roles: decl.roles ?? [],
						stages: decl.stages ?? [],
						paths: decl.paths ?? [],
						tags: decl.tags ?? [],
						priority: decl.priority,
						summary: decl.summary ?? null,
						content_file: contentFile,
					});
					contributed = true;
				}
			}

			// Scan prompt_sections
			const sectionDecls = provides.prompt_sections as
				| PromptSection[]
				| undefined;
			if (Array.isArray(sectionDecls)) {
				for (const sec of sectionDecls) {
					sections.push({
						id: sec.id,
						plugin: pluginName,
						source,
						type: sec.type,
						role: sec.role ?? null,
						stage: sec.stage ?? null,
						priority: sec.priority,
						content_file: path.join(pluginDir, sec.content_file),
					});
					contributed = true;
				}
			}

			if (contributed) {
				contributors.push(pluginName);
			}
		}
	}

	return { knowledge, sections, contributors, errors };
}

// ---------------------------------------------------------------------------
// Source Classification
// ---------------------------------------------------------------------------

/**
 * Classify a plugin manifest into a knowledge source category.
 *
 * Priority order (highest first):
 *   1. project-rules — not from plugins, but reserved for .orqa/process/rules/
 *   2. project-knowledge — not from plugins, but reserved for .orqa/process/knowledge/
 *   3. plugin — most installed plugins
 *   4. core — the core framework plugin
 */
function classifySource(manifest: Record<string, unknown>): KnowledgeSource {
	const role = manifest.role as string | undefined;
	if (role === "core:framework") return "core";
	return "plugin";
}

// ---------------------------------------------------------------------------
// Registry Building
// ---------------------------------------------------------------------------

/** Build the prompt registry from all installed plugins and write to disk. */
export function buildPromptRegistry(projectRoot: string): PromptRegistry {
	const scan = scanPluginPromptContributions(projectRoot);

	const registry: PromptRegistry = {
		version: 1,
		built_at: new Date().toISOString(),
		knowledge: scan.knowledge,
		sections: scan.sections,
		contributors: scan.contributors,
		errors: scan.errors,
	};

	// Write to .orqa/prompt-registry.json
	const outputDir = path.join(projectRoot, ".orqa");
	if (!fs.existsSync(outputDir)) {
		fs.mkdirSync(outputDir, { recursive: true });
	}

	const outputPath = path.join(outputDir, "prompt-registry.json");
	fs.writeFileSync(
		outputPath,
		JSON.stringify(registry, null, 2) + "\n",
		"utf-8",
	);

	return registry;
}

// ---------------------------------------------------------------------------
// Registry Reading
// ---------------------------------------------------------------------------

/** Read the cached prompt registry from disk. Returns null if not found. */
export function readPromptRegistry(
	projectRoot: string,
): PromptRegistry | null {
	const registryPath = path.join(
		projectRoot,
		".orqa",
		"prompt-registry.json",
	);
	if (!fs.existsSync(registryPath)) return null;

	try {
		const content = fs.readFileSync(registryPath, "utf-8");
		return JSON.parse(content) as PromptRegistry;
	} catch {
		return null;
	}
}

// ---------------------------------------------------------------------------
// Lookup Helpers
// ---------------------------------------------------------------------------

/**
 * Query the registry for knowledge entries matching a (role, stage, paths) tuple.
 *
 * This is the main lookup used by Stage 2 (Schema Assembly) of the pipeline.
 */
export function queryKnowledge(
	registry: PromptRegistry,
	query: {
		role?: string;
		stage?: string;
		filePaths?: string[];
	},
): RegistryKnowledgeEntry[] {
	return registry.knowledge.filter((entry) => {
		// Tier-based filtering
		if (entry.tier === "always") {
			// Always-tier: match by role and/or paths
			const roleMatch =
				entry.roles.length === 0 ||
				(query.role !== undefined && entry.roles.includes(query.role));
			const pathMatch =
				entry.paths.length === 0 || matchesAnyPath(query.filePaths, entry.paths);
			return roleMatch || pathMatch;
		}

		if (entry.tier === "stage-triggered") {
			// Stage-triggered: must match stage, optionally role/paths
			if (query.stage === undefined) return false;
			const stageMatch = entry.stages.includes(query.stage);
			if (!stageMatch) return false;
			const roleMatch =
				entry.roles.length === 0 ||
				(query.role !== undefined && entry.roles.includes(query.role));
			return roleMatch;
		}

		// On-demand: not returned by query — retrieved via semantic search
		return false;
	});
}

/** Query the registry for prompt sections matching a (role, stage) tuple. */
export function querySections(
	registry: PromptRegistry,
	query: {
		role?: string;
		stage?: string;
	},
): RegistryPromptSection[] {
	return registry.sections.filter((section) => {
		const roleMatch =
			section.role === null ||
			(query.role !== undefined && section.role === query.role);
		const stageMatch =
			section.stage === null ||
			(query.stage !== undefined && section.stage === query.stage);
		return roleMatch && stageMatch;
	});
}

// ---------------------------------------------------------------------------
// Path Matching (simple glob support)
// ---------------------------------------------------------------------------

/**
 * Check if any of the file paths match any of the glob patterns.
 *
 * Supports simple glob patterns:
 * - `**` matches any number of path segments
 * - `*` matches any characters within a single path segment
 */
function matchesAnyPath(
	filePaths: string[] | undefined,
	patterns: string[],
): boolean {
	if (!filePaths || filePaths.length === 0) return false;

	for (const fp of filePaths) {
		const normalized = fp.replace(/\\/g, "/");
		for (const pattern of patterns) {
			if (simpleGlobMatch(normalized, pattern)) return true;
		}
	}

	return false;
}

/** Simple glob matcher — converts glob to regex. */
function simpleGlobMatch(filePath: string, pattern: string): boolean {
	// Escape regex special chars except * and **
	const regexStr = pattern
		.replace(/\\/g, "/")
		.replace(/[.+^${}()|[\]]/g, "\\$&")
		.replace(/\*\*/g, "\0")
		.replace(/\*/g, "[^/]*")
		.replace(/\0/g, ".*");

	try {
		return new RegExp(`^${regexStr}$`).test(filePath);
	} catch {
		return false;
	}
}

// ---------------------------------------------------------------------------
// CLI Integration
// ---------------------------------------------------------------------------

/**
 * Run prompt registry build and print results.
 *
 * Called from `cmdPluginSync` in install.ts and plugin refresh commands.
 */
export function runPromptRegistryBuild(projectRoot: string): void {
	const registry = buildPromptRegistry(projectRoot);

	const totalEntries =
		registry.knowledge.length + registry.sections.length;

	if (totalEntries === 0 && registry.errors.length === 0) {
		// No prompt contributions found — nothing to report
		return;
	}

	if (registry.errors.length > 0) {
		console.log("  Prompt registry warnings:");
		for (const err of registry.errors) {
			console.log(`    - ${err}`);
		}
	}

	if (registry.knowledge.length > 0) {
		const byTier = groupBy(registry.knowledge, (k) => k.tier);
		const tierSummary = Object.entries(byTier)
			.map(([tier, entries]) => `${entries.length} ${tier}`)
			.join(", ");
		console.log(
			`  Prompt registry: ${registry.knowledge.length} knowledge entries (${tierSummary})`,
		);
	}

	if (registry.sections.length > 0) {
		console.log(
			`  Prompt registry: ${registry.sections.length} prompt sections`,
		);
	}

	if (registry.contributors.length > 0) {
		const relPath = ".orqa/prompt-registry.json";
		console.log(
			`  Prompt registry cached → ${relPath} (${registry.contributors.length} plugin(s))`,
		);
	}
}

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------

function groupBy<T>(
	items: T[],
	keyFn: (item: T) => string,
): Record<string, T[]> {
	const result: Record<string, T[]> = {};
	for (const item of items) {
		const key = keyFn(item);
		if (!result[key]) result[key] = [];
		result[key].push(item);
	}
	return result;
}
