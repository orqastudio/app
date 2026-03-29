import { describe, it, expect, beforeEach, afterEach } from "vitest";
import * as fs from "node:fs";
import * as path from "node:path";
import * as os from "node:os";
import {
	TokenTracker,
	recordRequest,
	recordAgentComplete,
	recordSessionSummary,
	readMetricEvents,
	filterEvents,
	computeTrends,
	getMetricsPath,
	type RequestMetrics,
	type AgentMetrics,
	type SessionMetrics,
} from "../src/lib/token-tracker.js";

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

function createTmpProject(): string {
	const dir = fs.mkdtempSync(path.join(os.tmpdir(), "orqa-tracker-test-"));
	fs.mkdirSync(path.join(dir, ".state"), { recursive: true });
	return dir;
}

function rmrf(dir: string): void {
	if (fs.existsSync(dir)) {
		fs.rmSync(dir, { recursive: true, force: true });
	}
}

// ---------------------------------------------------------------------------
// getMetricsPath
// ---------------------------------------------------------------------------

describe("getMetricsPath", () => {
	it("returns .state/token-metrics.jsonl path", () => {
		const p = getMetricsPath("/project");
		expect(p).toContain(".state");
		expect(p).toContain("token-metrics.jsonl");
	});
});

// ---------------------------------------------------------------------------
// Metric Writers + Readers
// ---------------------------------------------------------------------------

describe("metric writers and readers", () => {
	let tmpDir: string;

	beforeEach(() => {
		tmpDir = createTmpProject();
	});

	afterEach(() => {
		rmrf(tmpDir);
	});

	it("writes and reads request events", () => {
		const metrics: RequestMetrics = {
			timestamp: new Date().toISOString(),
			agentId: "agent-1",
			taskId: "task-1",
			inputTokens: 1000,
			outputTokens: 500,
			cacheHitTokens: 200,
			reasoningTokens: 100,
			model: "claude-sonnet-4-6",
			latencyMs: 1500,
		};

		recordRequest(tmpDir, metrics);

		const events = readMetricEvents(tmpDir);
		expect(events).toHaveLength(1);
		expect(events[0]._type).toBe("request");
		expect(events[0].data).toEqual(metrics);
	});

	it("writes and reads agent_complete events", () => {
		const metrics: AgentMetrics = {
			agentId: "agent-1",
			role: "implementer",
			model: "claude-sonnet-4-6",
			totalInputTokens: 5000,
			totalOutputTokens: 3000,
			contextUtilization: 0.45,
			requestCount: 12,
			lifetimeMs: 60000,
		};

		recordAgentComplete(tmpDir, metrics);

		const events = readMetricEvents(tmpDir);
		expect(events).toHaveLength(1);
		expect(events[0]._type).toBe("agent_complete");
	});

	it("writes and reads session_summary events", () => {
		const metrics: SessionMetrics = {
			sessionId: "sess-1",
			startTime: new Date().toISOString(),
			totalTokens: 50000,
			totalCost: 1.25,
			agentSpawns: 5,
			overheadRatio: 0.15,
			teamSpawnCost: 7500,
		};

		recordSessionSummary(tmpDir, metrics);

		const events = readMetricEvents(tmpDir);
		expect(events).toHaveLength(1);
		expect(events[0]._type).toBe("session_summary");
	});

	it("handles multiple event types", () => {
		recordRequest(tmpDir, {
			timestamp: new Date().toISOString(),
			agentId: "a1",
			inputTokens: 100,
			outputTokens: 50,
			cacheHitTokens: 0,
			reasoningTokens: 0,
			model: "claude-haiku-4-5",
			latencyMs: 200,
		});
		recordAgentComplete(tmpDir, {
			agentId: "a1",
			role: "reviewer",
			model: "claude-haiku-4-5",
			totalInputTokens: 100,
			totalOutputTokens: 50,
			contextUtilization: 0.1,
			requestCount: 1,
			lifetimeMs: 5000,
		});

		const events = readMetricEvents(tmpDir);
		expect(events).toHaveLength(2);

		const requests = filterEvents(events, "request");
		expect(requests).toHaveLength(1);

		const agents = filterEvents(events, "agent_complete");
		expect(agents).toHaveLength(1);
	});

	it("returns empty array for non-existent file", () => {
		const events = readMetricEvents("/nonexistent/path");
		expect(events).toEqual([]);
	});

	it("skips malformed lines", () => {
		const metricsPath = getMetricsPath(tmpDir);
		fs.appendFileSync(metricsPath, "not valid json\n", "utf-8");
		fs.appendFileSync(
			metricsPath,
			JSON.stringify({ _type: "request", data: { agentId: "a1" } }) + "\n",
			"utf-8",
		);

		const events = readMetricEvents(tmpDir);
		expect(events).toHaveLength(1);
	});
});

// ---------------------------------------------------------------------------
// TokenTracker class
// ---------------------------------------------------------------------------

describe("TokenTracker", () => {
	let tmpDir: string;

	beforeEach(() => {
		tmpDir = createTmpProject();
	});

	afterEach(() => {
		rmrf(tmpDir);
	});

	it("tracks requests and accumulates totals", () => {
		const tracker = new TokenTracker(tmpDir, "test-session");

		tracker.trackRequest({
			timestamp: new Date().toISOString(),
			agentId: "a1",
			inputTokens: 1000,
			outputTokens: 500,
			cacheHitTokens: 200,
			reasoningTokens: 0,
			model: "claude-sonnet-4-6",
			latencyMs: 1000,
		});

		tracker.trackRequest({
			timestamp: new Date().toISOString(),
			agentId: "a1",
			inputTokens: 800,
			outputTokens: 400,
			cacheHitTokens: 100,
			reasoningTokens: 0,
			model: "claude-sonnet-4-6",
			latencyMs: 900,
		});

		const totals = tracker.getSessionTotals();
		expect(totals.totalTokens).toBe(2700); // 1000+500+800+400
		expect(totals.inputTokens).toBe(1800);
		expect(totals.outputTokens).toBe(900);
		expect(totals.cacheHitTokens).toBe(300);
		expect(totals.requestCount).toBe(2);
	});

	it("tracks agent completions and spawn count", () => {
		const tracker = new TokenTracker(tmpDir, "test-session");

		tracker.trackAgentComplete({
			agentId: "a1",
			role: "implementer",
			model: "claude-sonnet-4-6",
			totalInputTokens: 5000,
			totalOutputTokens: 3000,
			contextUtilization: 0.4,
			requestCount: 10,
			lifetimeMs: 30000,
		});

		const totals = tracker.getSessionTotals();
		expect(totals.agentSpawns).toBe(1);
	});

	it("finalize writes session summary and returns it", () => {
		const tracker = new TokenTracker(tmpDir, "test-session");

		tracker.trackRequest({
			timestamp: new Date().toISOString(),
			agentId: "a1",
			inputTokens: 1000,
			outputTokens: 500,
			cacheHitTokens: 0,
			reasoningTokens: 0,
			model: "claude-sonnet-4-6",
			latencyMs: 1000,
		});

		tracker.trackAgentComplete({
			agentId: "a1",
			role: "implementer",
			model: "claude-sonnet-4-6",
			totalInputTokens: 1000,
			totalOutputTokens: 500,
			contextUtilization: 0.3,
			requestCount: 1,
			lifetimeMs: 5000,
		});

		const summary = tracker.finalize(0.05);
		expect(summary.sessionId).toBe("test-session");
		expect(summary.totalTokens).toBe(1500);
		expect(summary.totalCost).toBe(0.05);
		expect(summary.agentSpawns).toBe(1);
		expect(summary.overheadRatio).toBeGreaterThan(0);

		// Verify it was written to JSONL
		const events = readMetricEvents(tmpDir);
		const sessionEvents = filterEvents(events, "session_summary");
		expect(sessionEvents).toHaveLength(1);
		expect(sessionEvents[0].data.sessionId).toBe("test-session");
	});
});

// ---------------------------------------------------------------------------
// computeTrends
// ---------------------------------------------------------------------------

describe("computeTrends", () => {
	let tmpDir: string;

	beforeEach(() => {
		tmpDir = createTmpProject();
	});

	afterEach(() => {
		rmrf(tmpDir);
	});

	it("computes trends from recent events", () => {
		// Write some request events with current timestamps
		const now = new Date().toISOString();
		recordRequest(tmpDir, {
			timestamp: now,
			agentId: "a1",
			inputTokens: 1000,
			outputTokens: 500,
			cacheHitTokens: 200,
			reasoningTokens: 0,
			model: "claude-sonnet-4-6",
			latencyMs: 1000,
		});
		recordRequest(tmpDir, {
			timestamp: now,
			agentId: "a2",
			inputTokens: 2000,
			outputTokens: 1000,
			cacheHitTokens: 500,
			reasoningTokens: 0,
			model: "claude-opus-4-6",
			latencyMs: 2000,
		});

		const trends = computeTrends(tmpDir, 7);
		expect(trends.periodDays).toBe(7);
		expect(trends.totalRequests).toBe(2);
		expect(trends.totalTokens).toBe(4500); // 1000+500+2000+1000
		expect(trends.avgCacheHitRate).toBeGreaterThan(0);
		expect(trends.modelDistribution["claude-sonnet-4-6"]).toBe(1);
		expect(trends.modelDistribution["claude-opus-4-6"]).toBe(1);
	});

	it("returns zeroes for empty data", () => {
		const trends = computeTrends(tmpDir, 7);
		expect(trends.totalTokens).toBe(0);
		expect(trends.totalRequests).toBe(0);
		expect(trends.totalSessions).toBe(0);
	});
});
