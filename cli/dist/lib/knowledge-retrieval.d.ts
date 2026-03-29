/**
 * On-demand knowledge retrieval — queries knowledge artifacts by tags, role,
 * or content for agents that need more context beyond what the prompt pipeline
 * pre-loads.
 *
 * Integration point: the MCP search tools (mcp__orqastudio__search_semantic)
 * are available at runtime. This module provides the disk-based fallback and
 * the preamble that tells agents how to access on-demand knowledge.
 */
import type { PromptRegistry, RegistryKnowledgeEntry } from "./prompt-registry.js";
/** Options for querying on-demand knowledge. */
export interface KnowledgeQueryOptions {
    /** Filter by semantic tags. */
    tags?: string[];
    /** Filter by agent role. */
    role?: string;
    /** Simple text search in content. */
    textQuery?: string;
    /** Maximum token budget for returned content. */
    tokenBudget?: number;
}
/** A retrieved knowledge artifact with its full content. */
export interface RetrievedKnowledge {
    /** Artifact ID. */
    id: string;
    /** Title from frontmatter. */
    title: string;
    /** Full text content (body, not frontmatter). */
    content: string;
    /** Estimated token count. */
    tokens: number;
    /** Tags from the registry entry. */
    tags: string[];
    /** Source plugin. */
    plugin: string;
}
/**
 * Generate the on-demand knowledge preamble text that instructs agents on
 * how to retrieve full knowledge text via semantic search.
 *
 * Included in generated prompts when on-demand knowledge entries exist.
 */
export declare function generateOnDemandPreamble(onDemandCount: number): string;
/**
 * Query the prompt registry for on-demand knowledge entries that match
 * the given criteria. Returns registry entries (not full content).
 */
export declare function queryOnDemandEntries(registry: PromptRegistry, options: KnowledgeQueryOptions): RegistryKnowledgeEntry[];
/**
 * Retrieve full knowledge artifact content from disk.
 *
 * Reads knowledge artifacts from .orqa/ directories, matching by tags, role,
 * or text content. Returns full artifact body text (not summary) within
 * the specified token budget.
 */
export declare function retrieveKnowledge(projectPath: string, options: KnowledgeQueryOptions): RetrievedKnowledge[];
/**
 * Count on-demand knowledge entries in the registry.
 */
export declare function countOnDemandEntries(registry: PromptRegistry): number;
//# sourceMappingURL=knowledge-retrieval.d.ts.map