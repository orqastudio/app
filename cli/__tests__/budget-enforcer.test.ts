import { describe, it, expect, beforeEach } from "vitest";
import {
	BudgetEnforcer,
	estimateCost,
	inferModelTier,
	suggestDowngrade,
	DEFAULT_BUDGETS,
	COST_PER_MTOK,
	MODEL_TIERS,
} from "../src/lib/budget-enforcer.js";

// ---------------------------------------------------------------------------
// estimateCost
// ---------------------------------------------------------------------------

describe("estimateCost", () => {
	it("calculates opus cost correctly", () => {
		// 1M input + 1M output = $15 + $75 = $90
		const cost = estimateCost("opus", 1_000_000, 1_000_000, 0);
		expect(cost).toBe(90);
	});

	it("subtracts cached tokens from input cost", () => {
		// 1M input, 500k cached, 1M output
		// Uncached input: 500k * $15/M = $7.50
		// Cached: 500k * $1.50/M = $0.75
		// Output: 1M * $75/M = $75
		// Total: $83.25
		const cost = estimateCost("opus", 1_000_000, 1_000_000, 500_000);
		expect(cost).toBe(83.25);
	});

	it("handles haiku tier pricing", () => {
		// 1M input, 1M output = $0.25 + $1.25 = $1.50
		const cost = estimateCost("haiku", 1_000_000, 1_000_000, 0);
		expect(cost).toBe(1.5);
	});

	it("handles zero tokens", () => {
		expect(estimateCost("sonnet", 0, 0, 0)).toBe(0);
	});

	it("handles cached tokens exceeding input (clamped to 0)", () => {
		// Should not go negative
		const cost = estimateCost("sonnet", 100, 200, 500);
		expect(cost).toBeGreaterThanOrEqual(0);
	});
});

// ---------------------------------------------------------------------------
// inferModelTier
// ---------------------------------------------------------------------------

describe("inferModelTier", () => {
	it("infers opus from model strings", () => {
		expect(inferModelTier("claude-opus-4-6")).toBe("opus");
		expect(inferModelTier("Claude-Opus-4")).toBe("opus");
	});

	it("infers haiku from model strings", () => {
		expect(inferModelTier("claude-haiku-4-5")).toBe("haiku");
	});

	it("defaults to sonnet for unknown models", () => {
		expect(inferModelTier("some-model")).toBe("sonnet");
		expect(inferModelTier("claude-sonnet-4-6")).toBe("sonnet");
	});
});

// ---------------------------------------------------------------------------
// suggestDowngrade
// ---------------------------------------------------------------------------

describe("suggestDowngrade", () => {
	it("suggests sonnet for opus", () => {
		expect(suggestDowngrade("opus")).toBe("sonnet");
	});

	it("suggests haiku for sonnet", () => {
		expect(suggestDowngrade("sonnet")).toBe("haiku");
	});

	it("returns undefined for haiku (cheapest)", () => {
		expect(suggestDowngrade("haiku")).toBeUndefined();
	});
});

// ---------------------------------------------------------------------------
// BudgetEnforcer — checkAgentSpawn
// ---------------------------------------------------------------------------

describe("BudgetEnforcer.checkAgentSpawn", () => {
	let enforcer: BudgetEnforcer;

	beforeEach(() => {
		enforcer = new BudgetEnforcer();
	});

	it("allows spawn within budget", () => {
		const result = enforcer.checkAgentSpawn(2000, "sonnet");
		expect(result.allowed).toBe(true);
		expect(result.severity).toBe("ok");
	});

	it("blocks spawn when prompt exceeds per-agent prompt budget", () => {
		const result = enforcer.checkAgentSpawn(5000, "sonnet");
		expect(result.allowed).toBe(false);
		expect(result.severity).toBe("blocked");
		expect(result.message).toContain("prompt");
	});

	it("blocks spawn when session tokens exhausted", () => {
		// Exhaust session budget
		enforcer.recordUsage("a1", 250_000, 250_000, 0, "sonnet");
		const result = enforcer.checkAgentSpawn(2000, "sonnet");
		expect(result.allowed).toBe(false);
		expect(result.severity).toBe("blocked");
		expect(result.message).toContain("token budget exhausted");
	});

	it("warns at 75% session budget", () => {
		// 75% of 500k = 375k tokens
		enforcer.recordUsage("a1", 200_000, 180_000, 0, "haiku");
		const result = enforcer.checkAgentSpawn(2000, "sonnet");
		expect(result.allowed).toBe(true);
		expect(result.severity).toBe("warning");
		expect(result.suggestedModelDowngrade).toBe("haiku");
	});

	it("warns critical at 90% session budget", () => {
		// 90% of 500k = 450k tokens
		enforcer.recordUsage("a1", 230_000, 220_000, 0, "haiku");
		const result = enforcer.checkAgentSpawn(2000, "opus");
		expect(result.allowed).toBe(true);
		expect(result.severity).toBe("critical");
		expect(result.suggestedModelDowngrade).toBe("sonnet");
	});
});

// ---------------------------------------------------------------------------
// BudgetEnforcer — checkAgentContinue
// ---------------------------------------------------------------------------

describe("BudgetEnforcer.checkAgentContinue", () => {
	let enforcer: BudgetEnforcer;

	beforeEach(() => {
		enforcer = new BudgetEnforcer();
	});

	it("allows agent within per-agent budget", () => {
		enforcer.recordUsage("a1", 10_000, 5_000, 0, "sonnet");
		const result = enforcer.checkAgentContinue("a1", "sonnet");
		expect(result.allowed).toBe(true);
		expect(result.severity).toBe("ok");
	});

	it("blocks agent exceeding per-agent budget", () => {
		enforcer.recordUsage("a1", 60_000, 50_000, 0, "sonnet");
		const result = enforcer.checkAgentContinue("a1", "sonnet");
		expect(result.allowed).toBe(false);
		expect(result.severity).toBe("blocked");
	});

	it("warns at 75% of per-agent budget", () => {
		// 75% of 100k = 75k
		enforcer.recordUsage("a1", 40_000, 36_000, 0, "sonnet");
		const result = enforcer.checkAgentContinue("a1", "sonnet");
		expect(result.allowed).toBe(true);
		expect(result.severity).toBe("warning");
	});
});

// ---------------------------------------------------------------------------
// BudgetEnforcer — recordUsage + getSessionUsage
// ---------------------------------------------------------------------------

describe("BudgetEnforcer.recordUsage", () => {
	it("accumulates session tokens and cost", () => {
		const enforcer = new BudgetEnforcer();

		enforcer.recordUsage("a1", 1000, 500, 200, "sonnet");
		enforcer.recordUsage("a2", 2000, 1000, 0, "opus");

		const usage = enforcer.getSessionUsage();
		expect(usage.totalTokens).toBe(4500); // 1000+500+2000+1000
		expect(usage.totalCost).toBeGreaterThan(0);
		expect(usage.tokenRatio).toBe(4500 / DEFAULT_BUDGETS.perSessionTotalTokens);
	});

	it("tracks per-agent usage separately", () => {
		const enforcer = new BudgetEnforcer();

		enforcer.recordUsage("a1", 1000, 500, 0, "sonnet");
		enforcer.recordUsage("a2", 2000, 1000, 0, "sonnet");
		enforcer.recordUsage("a1", 500, 300, 0, "sonnet");

		expect(enforcer.getAgentUsage("a1")).toBe(2300); // 1500 + 800
		expect(enforcer.getAgentUsage("a2")).toBe(3000);
		expect(enforcer.getAgentUsage("a3")).toBe(0);
	});
});

// ---------------------------------------------------------------------------
// BudgetEnforcer — cost budget enforcement
// ---------------------------------------------------------------------------

describe("BudgetEnforcer cost budget", () => {
	it("blocks when cost budget is exceeded", () => {
		const enforcer = new BudgetEnforcer({ perSessionCostUsd: 0.01 });

		// A small number of opus tokens should exhaust a $0.01 budget
		enforcer.recordUsage("a1", 500, 200, 0, "opus");
		const result = enforcer.checkAgentSpawn(2000, "opus");
		expect(result.allowed).toBe(false);
		expect(result.severity).toBe("blocked");
		expect(result.message).toContain("cost budget exhausted");
	});
});

// ---------------------------------------------------------------------------
// BudgetEnforcer — reset
// ---------------------------------------------------------------------------

describe("BudgetEnforcer.reset", () => {
	it("resets all counters", () => {
		const enforcer = new BudgetEnforcer();
		enforcer.recordUsage("a1", 50_000, 50_000, 0, "sonnet");

		enforcer.reset();

		const usage = enforcer.getSessionUsage();
		expect(usage.totalTokens).toBe(0);
		expect(usage.totalCost).toBe(0);
		expect(enforcer.getAgentUsage("a1")).toBe(0);
	});
});

// ---------------------------------------------------------------------------
// Custom budget config
// ---------------------------------------------------------------------------

describe("BudgetEnforcer with custom config", () => {
	it("merges custom config with defaults", () => {
		const enforcer = new BudgetEnforcer({ perAgentPromptTokens: 8000 });
		const config = enforcer.getConfig();
		expect(config.perAgentPromptTokens).toBe(8000);
		expect(config.perSessionTotalTokens).toBe(DEFAULT_BUDGETS.perSessionTotalTokens);
	});
});
