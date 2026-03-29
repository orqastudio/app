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
import type { PromptPriority, PromptSectionType } from "@orqastudio/types";
import type { KnowledgeSource } from "./prompt-registry.js";
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
/** Default token budgets per agent role. */
export declare const DEFAULT_TOKEN_BUDGETS: Record<string, number>;
/** Estimate token count from character length (chars / 4 approximation). */
export declare function estimateTokens(text: string): number;
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
export declare function generatePrompt(options: PromptPipelineOptions): PromptResult;
//# sourceMappingURL=prompt-pipeline.d.ts.map