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
import type { KnowledgeInjectionTier, PromptPriority, PromptSectionType } from "@orqastudio/types";
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
/** Scans installed plugins for knowledge declarations and prompt sections. */
export declare function scanPluginPromptContributions(projectRoot: string): {
    knowledge: RegistryKnowledgeEntry[];
    sections: RegistryPromptSection[];
    contributors: string[];
    errors: string[];
};
/** Build the prompt registry from all installed plugins and write to disk. */
export declare function buildPromptRegistry(projectRoot: string): PromptRegistry;
/** Read the cached prompt registry from disk. Returns null if not found. */
export declare function readPromptRegistry(projectRoot: string): PromptRegistry | null;
/**
 * Query the registry for knowledge entries matching a (role, stage, paths) tuple.
 *
 * This is the main lookup used by Stage 2 (Schema Assembly) of the pipeline.
 */
export declare function queryKnowledge(registry: PromptRegistry, query: {
    role?: string;
    stage?: string;
    filePaths?: string[];
}): RegistryKnowledgeEntry[];
/** Query the registry for prompt sections matching a (role, stage) tuple. */
export declare function querySections(registry: PromptRegistry, query: {
    role?: string;
    stage?: string;
}): RegistryPromptSection[];
/**
 * Run prompt registry build and print results.
 *
 * Called from `cmdPluginSync` in install.ts and plugin refresh commands.
 */
export declare function runPromptRegistryBuild(projectRoot: string): void;
//# sourceMappingURL=prompt-registry.d.ts.map