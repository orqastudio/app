/**
 * Five-stage prompt generation pipeline.
 *
 * Assembles role-specific, token-budgeted, KV-cache-aware prompts from
 * the cached prompt registry (built by prompt-registry.ts at install time).
 *
 * Stages:
 *   1. Plugin Registry — query cached registry (prompt-registry.ts)
 *   2. Schema Assembly — collect sections for (role, stage, task), apply conflict resolution
 *   3. Section Resolution — resolve references, load content from disk
 *   4. Token Budgeting — trim by priority (P3 first, P0 never)
 *   5. Prompt Output — KV-cache-aware assembly (static top, dynamic bottom)
 */

import * as fs from "node:fs";
import type {
	PromptPriority,
	PromptSectionType,
} from "@orqastudio/types";
import type {
	KnowledgeSource,
	PromptRegistry,
	RegistryKnowledgeEntry,
	RegistryPromptSection,
} from "./prompt-registry.js";
import {
	queryKnowledge,
	querySections,
	readPromptRegistry,
} from "./prompt-registry.js";

// ---------------------------------------------------------------------------
// Public API Types
// ---------------------------------------------------------------------------

/** Options for the prompt generation pipeline. */
export interface PromptPipelineOptions {
	/** Agent role (e.g. "implementer", "reviewer", "orchestrator"). */
	role: string;
	/** Current workflow stage (e.g. "implement", "review"). */
	workflowStage?: string;
	/** Task context injected as dynamic content. */
	taskContext?: {
		description: string;
		files?: string[];
		acceptanceCriteria?: string[];
	};
	/** Total token budget for this prompt. Defaults to role-based budget. */
	tokenBudget?: number;
	/** Project root directory (used to read registry and content files). */
	projectPath: string;
}

/** A resolved section ready for output. */
export interface ResolvedSection {
	/** Origin ID (from knowledge entry or prompt section). */
	id: string;
	/** Source type for conflict resolution. */
	source: KnowledgeSource;
	/** Section category for ordering. */
	type: PromptSectionType | "knowledge" | "task-context";
	/** Priority level. */
	priority: PromptPriority;
	/** Resolved text content. */
	content: string;
	/** Estimated token count (chars / 4). */
	tokens: number;
}

/** Result of the prompt generation pipeline. */
export interface PromptResult {
	/** The assembled prompt string. */
	prompt: string;
	/** Total estimated token count. */
	totalTokens: number;
	/** Token budget that was applied. */
	budget: number;
	/** Sections included in the prompt (after budgeting). */
	includedSections: ResolvedSection[];
	/** Sections trimmed by the budget enforcer. */
	trimmedSections: ResolvedSection[];
	/** Errors encountered during generation. */
	errors: string[];
}

// ---------------------------------------------------------------------------
// Default Token Budgets (from research RES-d6e8ab11 section 6)
// ---------------------------------------------------------------------------

/** Default token budgets per agent role. */
export const DEFAULT_TOKEN_BUDGETS: Record<string, number> = {
	orchestrator: 2500,
	implementer: 2800,
	reviewer: 1900,
	researcher: 2100,
	writer: 1800,
	designer: 1800,
};

/** Fallback budget when role is not in the table. */
const FALLBACK_BUDGET = 2000;

// ---------------------------------------------------------------------------
// Source Priority (lower = higher priority)
// ---------------------------------------------------------------------------

const SOURCE_PRIORITY: Record<KnowledgeSource, number> = {
	"project-rules": 0,
	"project-knowledge": 1,
	plugin: 2,
	core: 3,
};

// ---------------------------------------------------------------------------
// Priority ordering for trim (P3 trimmed first)
// ---------------------------------------------------------------------------

const PRIORITY_TRIM_ORDER: Record<PromptPriority, number> = {
	P3: 0,
	P2: 1,
	P1: 2,
	P0: 3,
};

// ---------------------------------------------------------------------------
// Section type → output zone mapping for KV-cache-aware ordering
// ---------------------------------------------------------------------------

/** Output zones: lower number = top of prompt (static), higher = bottom (dynamic). */
const SECTION_ZONE: Record<string, number> = {
	"role-definition": 0,
	"safety-rule": 1,
	constraint: 2,
	"stage-instruction": 3,
	knowledge: 4,
	"task-template": 5,
	"task-context": 6,
};

// ---------------------------------------------------------------------------
// Stage 1 — Plugin Registry (delegates to prompt-registry.ts)
// ---------------------------------------------------------------------------

/**
 * Load the cached prompt registry from disk.
 * Returns null with an error message if not found.
 */
function loadRegistry(
	projectPath: string,
): { registry: PromptRegistry | null; error: string | null } {
	const registry = readPromptRegistry(projectPath);
	if (!registry) {
		return {
			registry: null,
			error: "Prompt registry not found. Run `orqa plugin install` to build it.",
		};
	}
	return { registry, error: null };
}

// ---------------------------------------------------------------------------
// Stage 2 — Schema Assembly
// ---------------------------------------------------------------------------

/** An assembled section before content resolution. */
interface AssembledSection {
	id: string;
	source: KnowledgeSource;
	type: PromptSectionType | "knowledge" | "task-context";
	priority: PromptPriority;
	/** Path to content file (needs resolution in stage 3). */
	contentFile: string | null;
	/** Inline content (summaries, task context — no file resolution needed). */
	inlineContent: string | null;
}

/**
 * Stage 2: Collect applicable sections for the given role/stage/task tuple.
 *
 * Applies conflict resolution: when two sections from different sources
 * cover the same domain (same id), the higher-priority source wins.
 */
function assembleSchema(
	registry: PromptRegistry,
	options: PromptPipelineOptions,
): AssembledSection[] {
	const assembled: AssembledSection[] = [];
	const seenIds = new Map<string, KnowledgeSource>();

	// Query knowledge entries from registry
	const knowledgeEntries = queryKnowledge(registry, {
		role: options.role,
		stage: options.workflowStage,
		filePaths: options.taskContext?.files,
	});

	// Apply conflict resolution on knowledge entries
	const resolvedKnowledge = resolveConflicts(knowledgeEntries);

	for (const entry of resolvedKnowledge) {
		assembled.push({
			id: entry.id,
			source: entry.source,
			type: "knowledge",
			priority: entry.priority,
			contentFile: entry.content_file,
			inlineContent: entry.summary,
		});
		seenIds.set(entry.id, entry.source);
	}

	// Query prompt sections from registry
	const promptSections = querySections(registry, {
		role: options.role,
		stage: options.workflowStage,
	});

	// Apply conflict resolution on prompt sections
	const resolvedSections = resolvePromptSectionConflicts(promptSections);

	for (const section of resolvedSections) {
		if (seenIds.has(section.id)) {
			const existingSource = seenIds.get(section.id)!;
			if (SOURCE_PRIORITY[section.source] >= SOURCE_PRIORITY[existingSource]) {
				continue; // Lower priority source, skip
			}
		}

		assembled.push({
			id: section.id,
			source: section.source,
			type: section.type,
			priority: section.priority,
			contentFile: section.content_file,
			inlineContent: null,
		});
		seenIds.set(section.id, section.source);
	}

	// Add task context as a dynamic section if provided
	if (options.taskContext) {
		assembled.push({
			id: "__task-context__",
			source: "project-knowledge",
			type: "task-context",
			priority: "P1",
			contentFile: null,
			inlineContent: formatTaskContext(options.taskContext),
		});
	}

	return assembled;
}

/**
 * Resolve knowledge entry conflicts: when multiple entries share the same id,
 * keep only the one from the highest-priority source.
 */
function resolveConflicts(
	entries: RegistryKnowledgeEntry[],
): RegistryKnowledgeEntry[] {
	const byId = new Map<string, RegistryKnowledgeEntry>();

	for (const entry of entries) {
		const existing = byId.get(entry.id);
		if (!existing || SOURCE_PRIORITY[entry.source] < SOURCE_PRIORITY[existing.source]) {
			byId.set(entry.id, entry);
		}
	}

	return Array.from(byId.values());
}

/**
 * Resolve prompt section conflicts: when multiple sections share the same id,
 * keep only the one from the highest-priority source.
 */
function resolvePromptSectionConflicts(
	sections: RegistryPromptSection[],
): RegistryPromptSection[] {
	const byId = new Map<string, RegistryPromptSection>();

	for (const section of sections) {
		const existing = byId.get(section.id);
		if (!existing || SOURCE_PRIORITY[section.source] < SOURCE_PRIORITY[existing.source]) {
			byId.set(section.id, section);
		}
	}

	return Array.from(byId.values());
}

/** Format task context into a prompt-friendly string. */
function formatTaskContext(ctx: NonNullable<PromptPipelineOptions["taskContext"]>): string {
	const parts: string[] = [];
	parts.push(`<task-description>\n${ctx.description}\n</task-description>`);

	if (ctx.files && ctx.files.length > 0) {
		parts.push(`<relevant-files>\n${ctx.files.join("\n")}\n</relevant-files>`);
	}

	if (ctx.acceptanceCriteria && ctx.acceptanceCriteria.length > 0) {
		const criteria = ctx.acceptanceCriteria.map((c, i) => `${i + 1}. ${c}`).join("\n");
		parts.push(`<acceptance-criteria>\n${criteria}\n</acceptance-criteria>`);
	}

	return parts.join("\n\n");
}

// ---------------------------------------------------------------------------
// Stage 3 — Section Resolution
// ---------------------------------------------------------------------------

/**
 * Stage 3: Resolve assembled sections by loading content from disk.
 *
 * For each section:
 * - If it has inline content (summary), use that
 * - If it has a content file, read from disk
 * - Follow cross-references to depth 1
 * - Detect and break circular references
 */
function resolveSections(
	assembled: AssembledSection[],
): { resolved: ResolvedSection[]; errors: string[] } {
	const resolved: ResolvedSection[] = [];
	const errors: string[] = [];
	const resolving = new Set<string>(); // Circular reference detection

	for (const section of assembled) {
		const result = resolveOneSection(section, resolving, errors);
		if (result) {
			resolved.push(result);
		}
	}

	return { resolved, errors };
}

/**
 * Resolve a single section's content.
 *
 * Priority:
 * 1. Content file on disk (full text)
 * 2. Inline content (summary)
 * 3. Skip with error if neither available
 */
function resolveOneSection(
	section: AssembledSection,
	resolving: Set<string>,
	errors: string[],
): ResolvedSection | null {
	// Circular reference guard
	if (resolving.has(section.id)) {
		errors.push(`Circular reference detected for section "${section.id}", skipping`);
		return null;
	}
	resolving.add(section.id);

	let content: string | null = null;

	// Try content file first
	if (section.contentFile) {
		content = readContentFile(section.contentFile, errors);
	}

	// Fall back to inline content (summary)
	if (!content && section.inlineContent) {
		content = section.inlineContent;
	}

	resolving.delete(section.id);

	if (!content) {
		errors.push(
			`No content available for section "${section.id}" (no file or summary)`,
		);
		return null;
	}

	// Follow cross-references at depth 1
	content = resolveCrossReferences(content, resolving, errors);

	const tokens = estimateTokens(content);

	return {
		id: section.id,
		source: section.source,
		type: section.type,
		priority: section.priority,
		content,
		tokens,
	};
}

/** Read content from a file path. Returns null on failure. */
function readContentFile(filePath: string, errors: string[]): string | null {
	try {
		if (!fs.existsSync(filePath)) {
			errors.push(`Content file not found: ${filePath}`);
			return null;
		}
		return fs.readFileSync(filePath, "utf-8").trim();
	} catch (err) {
		errors.push(
			`Failed to read ${filePath}: ${err instanceof Error ? err.message : String(err)}`,
		);
		return null;
	}
}

/**
 * Resolve cross-references in content text (depth 1 only).
 *
 * Recognizes patterns like `{{ref:ARTIFACT-ID}}` and replaces them
 * with the referenced artifact's summary or a placeholder.
 */
function resolveCrossReferences(
	content: string,
	resolving: Set<string>,
	errors: string[],
): string {
	const refPattern = /\{\{ref:([A-Z]+-[a-f0-9]+)\}\}/g;
	return content.replace(refPattern, (_match, artifactId: string) => {
		if (resolving.has(artifactId)) {
			errors.push(`Circular cross-reference to "${artifactId}", skipping`);
			return `[circular ref: ${artifactId}]`;
		}
		// Depth 1: don't recursively resolve further
		return `[ref: ${artifactId}]`;
	});
}

// ---------------------------------------------------------------------------
// Stage 4 — Token Budgeting
// ---------------------------------------------------------------------------

/** Estimate token count from character length (chars / 4 approximation). */
export function estimateTokens(text: string): number {
	return Math.ceil(text.length / 4);
}

/**
 * Stage 4: Enforce the token budget by trimming low-priority sections.
 *
 * Trim order: P3 first, then P2, then P1. P0 is NEVER trimmed.
 * Within the same priority, trim the largest sections first.
 */
function applyTokenBudget(
	sections: ResolvedSection[],
	budget: number,
): { included: ResolvedSection[]; trimmed: ResolvedSection[] } {
	const totalTokens = sections.reduce((sum, s) => sum + s.tokens, 0);

	if (totalTokens <= budget) {
		return { included: [...sections], trimmed: [] };
	}

	// Separate P0 (untouchable) from trimmable sections
	const p0Sections = sections.filter((s) => s.priority === "P0");
	const trimmable = sections
		.filter((s) => s.priority !== "P0")
		.sort((a, b) => {
			// Sort by trim order (P3 first), then by token count descending
			const orderDiff = PRIORITY_TRIM_ORDER[a.priority] - PRIORITY_TRIM_ORDER[b.priority];
			if (orderDiff !== 0) return orderDiff;
			return b.tokens - a.tokens;
		});

	const p0Tokens = p0Sections.reduce((sum, s) => sum + s.tokens, 0);
	let remainingBudget = budget - p0Tokens;

	const included: ResolvedSection[] = [...p0Sections];
	const trimmed: ResolvedSection[] = [];

	// Work from end of sorted list (highest priority / smallest first)
	// to front — we keep the most important sections
	const reversed = [...trimmable].reverse();
	const toKeep: ResolvedSection[] = [];

	for (const section of reversed) {
		if (section.tokens <= remainingBudget) {
			toKeep.push(section);
			remainingBudget -= section.tokens;
		} else {
			trimmed.push(section);
		}
	}

	included.push(...toKeep);

	return { included, trimmed };
}

// ---------------------------------------------------------------------------
// Stage 5 — Prompt Output
// ---------------------------------------------------------------------------

/**
 * Stage 5: Assemble the final prompt with KV-cache-aware ordering.
 *
 * Layout:
 *   1. Static core at TOP — role definition, safety rules, constraints
 *   2. Semi-static middle — stage instructions, knowledge
 *   3. Dynamic content at BOTTOM — task context, file paths
 *
 * Uses Claude XML tags for structure. Never reorder sections between turns.
 */
function assemblePrompt(sections: ResolvedSection[], role: string): string {
	// Sort by zone (static → dynamic), preserving relative order within zones
	const sorted = [...sections].sort((a, b) => {
		const zoneA = SECTION_ZONE[a.type] ?? 4;
		const zoneB = SECTION_ZONE[b.type] ?? 4;
		return zoneA - zoneB;
	});

	const parts: string[] = [];

	// Role header (always at top for KV-cache prefix stability)
	parts.push(`<role>${role}</role>`);

	// Group sections by zone
	let currentZone = -1;
	for (const section of sorted) {
		const zone = SECTION_ZONE[section.type] ?? 4;
		if (zone !== currentZone) {
			currentZone = zone;
		}

		const tag = sectionTypeToTag(section.type);
		parts.push(`<${tag} id="${section.id}" priority="${section.priority}">`);
		parts.push(section.content);
		parts.push(`</${tag}>`);
	}

	return parts.join("\n\n");
}

/** Map section type to an XML tag name. */
function sectionTypeToTag(type: string): string {
	switch (type) {
		case "role-definition":
			return "role-definition";
		case "safety-rule":
			return "safety-rule";
		case "constraint":
			return "constraint";
		case "stage-instruction":
			return "stage-instruction";
		case "knowledge":
			return "knowledge";
		case "task-template":
			return "task-template";
		case "task-context":
			return "task-context";
		default:
			return "section";
	}
}

// ---------------------------------------------------------------------------
// Main Pipeline Entry Point
// ---------------------------------------------------------------------------

/**
 * Generate a role-specific, token-budgeted, KV-cache-aware prompt.
 *
 * Runs all five stages of the pipeline:
 *   1. Load cached registry
 *   2. Assemble applicable sections
 *   3. Resolve content from disk
 *   4. Apply token budget
 *   5. Emit ordered prompt
 */
export function generatePrompt(options: PromptPipelineOptions): PromptResult {
	const errors: string[] = [];

	// Stage 1 — Load registry
	const { registry, error: registryError } = loadRegistry(options.projectPath);
	if (registryError) {
		errors.push(registryError);
	}
	if (!registry) {
		return {
			prompt: "",
			totalTokens: 0,
			budget: options.tokenBudget ?? DEFAULT_TOKEN_BUDGETS[options.role] ?? FALLBACK_BUDGET,
			includedSections: [],
			trimmedSections: [],
			errors,
		};
	}

	// Stage 2 — Schema Assembly
	const assembled = assembleSchema(registry, options);

	// Stage 3 — Section Resolution
	const { resolved, errors: resolveErrors } = resolveSections(assembled);
	errors.push(...resolveErrors);

	// Stage 4 — Token Budgeting
	const budget = options.tokenBudget ?? DEFAULT_TOKEN_BUDGETS[options.role] ?? FALLBACK_BUDGET;
	const { included, trimmed } = applyTokenBudget(resolved, budget);

	// Stage 5 — Prompt Output
	const prompt = assemblePrompt(included, options.role);
	const totalTokens = estimateTokens(prompt);

	return {
		prompt,
		totalTokens,
		budget,
		includedSections: included,
		trimmedSections: trimmed,
		errors,
	};
}
