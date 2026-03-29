/**
 * Budget enforcement for agent spawning and session cost control.
 *
 * Enforces per-agent prompt size limits, per-agent total token limits,
 * and per-session total token/cost limits. Emits warnings at 75% and
 * 90% thresholds and hard-stops at 100%.
 * @module budget-enforcer
 */
// ---------------------------------------------------------------------------
// Model Tier Pricing
// ---------------------------------------------------------------------------
/** Cost per million tokens by model tier (USD). */
export const COST_PER_MTOK = {
    opus: { input: 15, output: 75, cached: 1.5 },
    sonnet: { input: 3, output: 15, cached: 0.3 },
    haiku: { input: 0.25, output: 1.25, cached: 0.025 },
};
/** All recognized model tiers, ordered from cheapest to most expensive. */
export const MODEL_TIERS = ["haiku", "sonnet", "opus"];
/** Sensible defaults from the research document. */
export const DEFAULT_BUDGETS = {
    perAgentPromptTokens: 4000,
    perAgentTotalTokens: 100000,
    perSessionTotalTokens: 500000,
    perSessionCostUsd: 5.0,
};
// ---------------------------------------------------------------------------
// Cost Estimation
// ---------------------------------------------------------------------------
/**
 * Estimate cost in USD for a given number of tokens at a model tier.
 * @param tier - Model tier (opus, sonnet, haiku)
 * @param inputTokens - Number of input tokens
 * @param outputTokens - Number of output tokens
 * @param cachedTokens - Number of cached input tokens (subtracted from input)
 * @returns Estimated cost in USD rounded to 6 decimal places.
 */
export function estimateCost(tier, inputTokens, outputTokens, cachedTokens = 0) {
    const pricing = COST_PER_MTOK[tier];
    const uncachedInput = Math.max(0, inputTokens - cachedTokens);
    const cost = (uncachedInput * pricing.input +
        outputTokens * pricing.output +
        cachedTokens * pricing.cached) /
        1_000_000;
    return Math.round(cost * 1_000_000) / 1_000_000;
}
/**
 * Infer model tier from a model string (e.g. "claude-opus-4-6" -> "opus").
 * Returns "sonnet" as default if the tier cannot be determined.
 * @param model - The model identifier string to parse.
 * @returns The inferred model tier.
 */
export function inferModelTier(model) {
    const lower = model.toLowerCase();
    if (lower.includes("opus"))
        return "opus";
    if (lower.includes("haiku"))
        return "haiku";
    return "sonnet";
}
/**
 * Suggest a cheaper model tier one step down from the current tier.
 * Returns undefined if already at the cheapest tier.
 * @param currentTier - The current model tier to downgrade from.
 * @returns The next cheaper tier, or undefined if already at haiku.
 */
export function suggestDowngrade(currentTier) {
    const idx = MODEL_TIERS.indexOf(currentTier);
    if (idx <= 0)
        return undefined;
    return MODEL_TIERS[idx - 1];
}
// ---------------------------------------------------------------------------
// Budget Enforcer
// ---------------------------------------------------------------------------
/**
 * Stateful budget enforcer for a session.
 *
 * Create one per session. Call `checkAgentSpawn` before spawning a new agent,
 * and `recordUsage` after each API response to keep running totals.
 */
export class BudgetEnforcer {
    config;
    sessionTokens = 0;
    sessionCost = 0;
    agentTokens = new Map();
    /**
     * Create a new BudgetEnforcer with optional config overrides.
     * @param config - Partial budget config to override defaults.
     */
    constructor(config) {
        this.config = { ...DEFAULT_BUDGETS, ...config };
    }
    /**
     * Get the active budget configuration.
     * @returns The active budget configuration.
     */
    getConfig() {
        return this.config;
    }
    /**
     * Get current session-level usage.
     * @returns Session usage totals including token and cost ratios.
     */
    getSessionUsage() {
        return {
            totalTokens: this.sessionTokens,
            totalCost: Math.round(this.sessionCost * 1_000_000) / 1_000_000,
            tokenRatio: this.sessionTokens / this.config.perSessionTotalTokens,
            costRatio: this.sessionCost / this.config.perSessionCostUsd,
        };
    }
    /**
     * Get token usage for a specific agent.
     * @param agentId - The agent's unique identifier.
     * @returns Total tokens used by this agent.
     */
    getAgentUsage(agentId) {
        return this.agentTokens.get(agentId) ?? 0;
    }
    /**
     * Check whether a new agent spawn is allowed under budget.
     * @param promptTokens - Estimated tokens in the agent's system prompt
     * @param modelTier - The model tier intended for this agent
     * @returns Budget check result indicating whether the spawn is allowed.
     */
    checkAgentSpawn(promptTokens, modelTier) {
        // Check per-agent prompt budget
        if (promptTokens > this.config.perAgentPromptTokens) {
            return {
                allowed: false,
                severity: "blocked",
                message: `Agent prompt (${promptTokens} tokens) exceeds per-agent prompt budget (${this.config.perAgentPromptTokens})`,
                usageRatio: promptTokens / this.config.perAgentPromptTokens,
            };
        }
        // Check session token budget
        const sessionRatio = this.sessionTokens / this.config.perSessionTotalTokens;
        if (sessionRatio >= 1.0) {
            return {
                allowed: false,
                severity: "blocked",
                message: `Session token budget exhausted (${this.sessionTokens}/${this.config.perSessionTotalTokens})`,
                usageRatio: sessionRatio,
            };
        }
        // Check session cost budget
        const costRatio = this.sessionCost / this.config.perSessionCostUsd;
        if (costRatio >= 1.0) {
            return {
                allowed: false,
                severity: "blocked",
                message: `Session cost budget exhausted ($${this.sessionCost.toFixed(2)}/$${this.config.perSessionCostUsd.toFixed(2)})`,
                usageRatio: costRatio,
            };
        }
        // Warning thresholds
        const maxRatio = Math.max(sessionRatio, costRatio);
        if (maxRatio >= 0.9) {
            const downgrade = suggestDowngrade(modelTier);
            return {
                allowed: true,
                severity: "critical",
                message: `Session budget at ${(maxRatio * 100).toFixed(0)}% — approaching limit`,
                suggestedModelDowngrade: downgrade,
                usageRatio: maxRatio,
            };
        }
        if (maxRatio >= 0.75) {
            const downgrade = suggestDowngrade(modelTier);
            return {
                allowed: true,
                severity: "warning",
                message: `Session budget at ${(maxRatio * 100).toFixed(0)}%`,
                suggestedModelDowngrade: downgrade,
                usageRatio: maxRatio,
            };
        }
        return {
            allowed: true,
            severity: "ok",
            message: `Session budget at ${(maxRatio * 100).toFixed(0)}%`,
            usageRatio: maxRatio,
        };
    }
    /**
     * Check whether an agent can continue operating within its per-agent budget.
     * @param agentId - The agent's unique identifier
     * @param modelTier - Current model tier for downgrade suggestions
     * @returns Budget check result for this agent's continuation.
     */
    checkAgentContinue(agentId, modelTier) {
        const used = this.agentTokens.get(agentId) ?? 0;
        const ratio = used / this.config.perAgentTotalTokens;
        if (ratio >= 1.0) {
            return {
                allowed: false,
                severity: "blocked",
                message: `Agent ${agentId} exceeded per-agent token budget (${used}/${this.config.perAgentTotalTokens})`,
                usageRatio: ratio,
            };
        }
        if (ratio >= 0.9) {
            return {
                allowed: true,
                severity: "critical",
                message: `Agent ${agentId} at ${(ratio * 100).toFixed(0)}% of token budget`,
                suggestedModelDowngrade: suggestDowngrade(modelTier),
                usageRatio: ratio,
            };
        }
        if (ratio >= 0.75) {
            return {
                allowed: true,
                severity: "warning",
                message: `Agent ${agentId} at ${(ratio * 100).toFixed(0)}% of token budget`,
                suggestedModelDowngrade: suggestDowngrade(modelTier),
                usageRatio: ratio,
            };
        }
        return {
            allowed: true,
            severity: "ok",
            message: `Agent ${agentId} at ${(ratio * 100).toFixed(0)}% of token budget`,
            usageRatio: ratio,
        };
    }
    /**
     * Record token usage from an API response.
     * Updates both session-level and agent-level running totals.
     * @param agentId - The agent's unique identifier.
     * @param inputTokens - Number of input tokens used.
     * @param outputTokens - Number of output tokens used.
     * @param cachedTokens - Number of cached input tokens used.
     * @param modelTier - The model tier used for this response.
     */
    recordUsage(agentId, inputTokens, outputTokens, cachedTokens, modelTier) {
        const total = inputTokens + outputTokens;
        this.sessionTokens += total;
        this.sessionCost += estimateCost(modelTier, inputTokens, outputTokens, cachedTokens);
        const prev = this.agentTokens.get(agentId) ?? 0;
        this.agentTokens.set(agentId, prev + total);
    }
    /** Reset all counters (for testing). */
    reset() {
        this.sessionTokens = 0;
        this.sessionCost = 0;
        this.agentTokens.clear();
    }
}
//# sourceMappingURL=budget-enforcer.js.map