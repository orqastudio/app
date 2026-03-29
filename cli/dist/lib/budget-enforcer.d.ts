/**
 * Budget enforcement for agent spawning and session cost control.
 *
 * Enforces per-agent prompt size limits, per-agent total token limits,
 * and per-session total token/cost limits. Emits warnings at 75% and
 * 90% thresholds and hard-stops at 100%.
 * @module budget-enforcer
 */
import type { ModelTier } from "./agent-spawner.js";
export type { ModelTier } from "./agent-spawner.js";
/** Cost per million tokens by model tier (USD). */
export declare const COST_PER_MTOK: Record<ModelTier, {
    input: number;
    output: number;
    cached: number;
}>;
/** All recognized model tiers, ordered from cheapest to most expensive. */
export declare const MODEL_TIERS: ModelTier[];
/** Budget limits for agent and session spending. */
export interface BudgetConfig {
    /** Max tokens allowed in an agent's system prompt. */
    perAgentPromptTokens: number;
    /** Max total tokens (input + output) per agent lifetime. */
    perAgentTotalTokens: number;
    /** Max total tokens (all agents) per session. */
    perSessionTotalTokens: number;
    /** Max estimated cost in USD per session. */
    perSessionCostUsd: number;
}
/** Sensible defaults from the research document. */
export declare const DEFAULT_BUDGETS: BudgetConfig;
/** Severity of a budget check result. */
export type BudgetSeverity = "ok" | "warning" | "critical" | "blocked";
/** Result of a budget check. */
export interface BudgetCheckResult {
    allowed: boolean;
    severity: BudgetSeverity;
    message: string;
    /** If the budget is under pressure, suggest a cheaper model tier. */
    suggestedModelDowngrade?: ModelTier;
    /** Current usage as a ratio of the limit (0.0 to 1.0+). */
    usageRatio: number;
}
/**
 * Estimate cost in USD for a given number of tokens at a model tier.
 * @param tier - Model tier (opus, sonnet, haiku)
 * @param inputTokens - Number of input tokens
 * @param outputTokens - Number of output tokens
 * @param cachedTokens - Number of cached input tokens (subtracted from input)
 * @returns Estimated cost in USD rounded to 6 decimal places.
 */
export declare function estimateCost(tier: ModelTier, inputTokens: number, outputTokens: number, cachedTokens?: number): number;
/**
 * Infer model tier from a model string (e.g. "claude-opus-4-6" -> "opus").
 * Returns "sonnet" as default if the tier cannot be determined.
 * @param model - The model identifier string to parse.
 * @returns The inferred model tier.
 */
export declare function inferModelTier(model: string): ModelTier;
/**
 * Suggest a cheaper model tier one step down from the current tier.
 * Returns undefined if already at the cheapest tier.
 * @param currentTier - The current model tier to downgrade from.
 * @returns The next cheaper tier, or undefined if already at haiku.
 */
export declare function suggestDowngrade(currentTier: ModelTier): ModelTier | undefined;
/**
 * Stateful budget enforcer for a session.
 *
 * Create one per session. Call `checkAgentSpawn` before spawning a new agent,
 * and `recordUsage` after each API response to keep running totals.
 */
export declare class BudgetEnforcer {
    private readonly config;
    private sessionTokens;
    private sessionCost;
    private readonly agentTokens;
    /**
     * Create a new BudgetEnforcer with optional config overrides.
     * @param config - Partial budget config to override defaults.
     */
    constructor(config?: Partial<BudgetConfig>);
    /**
     * Get the active budget configuration.
     * @returns The active budget configuration.
     */
    getConfig(): Readonly<BudgetConfig>;
    /**
     * Get current session-level usage.
     * @returns Session usage totals including token and cost ratios.
     */
    getSessionUsage(): {
        totalTokens: number;
        totalCost: number;
        tokenRatio: number;
        costRatio: number;
    };
    /**
     * Get token usage for a specific agent.
     * @param agentId - The agent's unique identifier.
     * @returns Total tokens used by this agent.
     */
    getAgentUsage(agentId: string): number;
    /**
     * Check whether a new agent spawn is allowed under budget.
     * @param promptTokens - Estimated tokens in the agent's system prompt
     * @param modelTier - The model tier intended for this agent
     * @returns Budget check result indicating whether the spawn is allowed.
     */
    checkAgentSpawn(promptTokens: number, modelTier: ModelTier): BudgetCheckResult;
    /**
     * Check whether an agent can continue operating within its per-agent budget.
     * @param agentId - The agent's unique identifier
     * @param modelTier - Current model tier for downgrade suggestions
     * @returns Budget check result for this agent's continuation.
     */
    checkAgentContinue(agentId: string, modelTier: ModelTier): BudgetCheckResult;
    /**
     * Record token usage from an API response.
     * Updates both session-level and agent-level running totals.
     * @param agentId - The agent's unique identifier.
     * @param inputTokens - Number of input tokens used.
     * @param outputTokens - Number of output tokens used.
     * @param cachedTokens - Number of cached input tokens used.
     * @param modelTier - The model tier used for this response.
     */
    recordUsage(agentId: string, inputTokens: number, outputTokens: number, cachedTokens: number, modelTier: ModelTier): void;
    /** Reset all counters (for testing). */
    reset(): void;
}
//# sourceMappingURL=budget-enforcer.d.ts.map