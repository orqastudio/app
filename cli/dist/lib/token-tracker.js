/**
 * Token tracking system — 4-level metrics capture.
 *
 * Captures token usage at per-request, per-agent, per-session, and
 * trend levels. All metrics are written to `.state/token-metrics.jsonl`
 * as newline-delimited JSON events.
 *
 * Levels:
 *   1. Per-Request — input/output tokens, cache hit rate, latency
 *   2. Per-Agent — total tokens, context utilization, lifetime
 *   3. Per-Session — tokens per deliverable, overhead ratio, cost
 *   4. Trends — 7-day/30-day aggregates (computed from historical data)
 * @module token-tracker
 */
import { appendFileSync, existsSync, mkdirSync, readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { randomUUID } from "node:crypto";
import { assertNever } from "@orqastudio/types";
// ---------------------------------------------------------------------------
// Metrics File Path
// ---------------------------------------------------------------------------
const METRICS_FILENAME = "token-metrics.jsonl";
/**
 * Get the path to the token metrics file for a project.
 * @param projectRoot - Absolute path to the project root.
 * @returns Absolute path to the token metrics JSONL file.
 */
export function getMetricsPath(projectRoot) {
    return join(projectRoot, ".state", METRICS_FILENAME);
}
// ---------------------------------------------------------------------------
// Metrics Writer
// ---------------------------------------------------------------------------
/**
 * Ensure the parent directory exists and append a line to the metrics file.
 * @param projectRoot - Absolute path to the project root.
 * @param line - The line to append to the metrics file.
 */
function appendMetricLine(projectRoot, line) {
    const metricsPath = getMetricsPath(projectRoot);
    const dir = dirname(metricsPath);
    if (!existsSync(dir)) {
        mkdirSync(dir, { recursive: true });
    }
    appendFileSync(metricsPath, line + "\n", "utf-8");
}
/**
 * Write a request-level metric event.
 * @param projectRoot - Absolute path to the project root.
 * @param metrics - The request metrics to record.
 */
export function recordRequest(projectRoot, metrics) {
    const event = { _type: "request", data: metrics };
    appendMetricLine(projectRoot, JSON.stringify(event));
}
/**
 * Write an agent-complete metric event.
 * @param projectRoot - Absolute path to the project root.
 * @param metrics - The agent metrics to record.
 */
export function recordAgentComplete(projectRoot, metrics) {
    const event = { _type: "agent_complete", data: metrics };
    appendMetricLine(projectRoot, JSON.stringify(event));
}
/**
 * Write a session-summary metric event.
 * @param projectRoot - Absolute path to the project root.
 * @param metrics - The session metrics to record.
 */
export function recordSessionSummary(projectRoot, metrics) {
    const event = { _type: "session_summary", data: metrics };
    appendMetricLine(projectRoot, JSON.stringify(event));
}
// ---------------------------------------------------------------------------
// Metrics Reader
// ---------------------------------------------------------------------------
/**
 * Read all metric events from the JSONL file.
 * @param projectRoot - Absolute path to the project root.
 * @returns Array of all metric events in the file.
 */
export function readMetricEvents(projectRoot) {
    const metricsPath = getMetricsPath(projectRoot);
    if (!existsSync(metricsPath))
        return [];
    const events = [];
    for (const line of readFileSync(metricsPath, "utf-8").split("\n")) {
        if (!line.trim())
            continue;
        try {
            const parsed = JSON.parse(line);
            if (parsed && typeof parsed === "object" && "_type" in parsed) {
                events.push(parsed);
            }
        }
        catch {
            // skip malformed lines
        }
    }
    return events;
}
/**
 * Filter events by type.
 * @param events - Array of metric events to filter.
 * @param type - The event type to filter by.
 * @returns Array of events matching the specified type.
 */
export function filterEvents(events, type) {
    return events.filter((e) => e._type === type);
}
// ---------------------------------------------------------------------------
// Token Tracker — Stateful Session Tracker
// ---------------------------------------------------------------------------
/**
 * Tracks token usage for an active session.
 *
 * Create one instance per orchestrator session. Call `trackRequest` for
 * each API response, `trackAgentComplete` when an agent finishes, and
 * `finalize` when the session ends.
 */
export class TokenTracker {
    sessionId;
    startTime;
    projectRoot;
    totalInputTokens = 0;
    totalOutputTokens = 0;
    totalCacheHitTokens = 0;
    requestCount = 0;
    agentSpawns = 0;
    teamSpawnCost = 0;
    /**
     * Create a new TokenTracker for a session.
     * @param projectRoot - Absolute path to the project root for writing metrics.
     * @param sessionId - Optional session ID; a UUID is generated if not provided.
     */
    constructor(projectRoot, sessionId) {
        this.projectRoot = projectRoot;
        this.sessionId = sessionId ?? randomUUID();
        this.startTime = new Date().toISOString();
    }
    /**
     * Record a single API request. Writes to JSONL immediately.
     * @param metrics - The request metrics to record.
     */
    trackRequest(metrics) {
        this.totalInputTokens += metrics.inputTokens;
        this.totalOutputTokens += metrics.outputTokens;
        this.totalCacheHitTokens += metrics.cacheHitTokens;
        this.requestCount++;
        recordRequest(this.projectRoot, metrics);
    }
    /**
     * Record an agent completing. Writes to JSONL immediately.
     * @param metrics - The agent metrics to record.
     */
    trackAgentComplete(metrics) {
        this.agentSpawns++;
        this.teamSpawnCost += metrics.totalInputTokens + metrics.totalOutputTokens;
        recordAgentComplete(this.projectRoot, metrics);
    }
    /**
     * Get current session totals without finalizing.
     * @returns Current session token counts and request statistics.
     */
    getSessionTotals() {
        return {
            totalTokens: this.totalInputTokens + this.totalOutputTokens,
            inputTokens: this.totalInputTokens,
            outputTokens: this.totalOutputTokens,
            cacheHitTokens: this.totalCacheHitTokens,
            requestCount: this.requestCount,
            agentSpawns: this.agentSpawns,
        };
    }
    /**
     * Finalize the session. Writes summary event. Returns the summary.
     * @param costEstimate - Estimated total cost in USD for this session.
     * @returns The completed session metrics summary.
     */
    finalize(costEstimate) {
        const totalTokens = this.totalInputTokens + this.totalOutputTokens;
        const overheadRatio = totalTokens > 0 ? this.teamSpawnCost / totalTokens : 0;
        const summary = {
            sessionId: this.sessionId,
            startTime: this.startTime,
            totalTokens,
            totalCost: costEstimate,
            agentSpawns: this.agentSpawns,
            overheadRatio: Math.round(overheadRatio * 1000) / 1000,
            teamSpawnCost: this.teamSpawnCost,
        };
        recordSessionSummary(this.projectRoot, summary);
        return summary;
    }
}
// ---------------------------------------------------------------------------
// Level 4 — Trend Computation
// ---------------------------------------------------------------------------
/**
 * Compute trend metrics over a given period from historical JSONL data.
 *
 * Reads all events and filters to those within `periodDays` of now.
 * @param projectRoot - Absolute path to the project root.
 * @param periodDays - Number of days back to include in the trend window.
 * @returns Computed trend metrics for the specified period.
 */
export function computeTrends(projectRoot, periodDays) {
    const events = readMetricEvents(projectRoot);
    const cutoff = new Date();
    cutoff.setDate(cutoff.getDate() - periodDays);
    const cutoffIso = cutoff.toISOString();
    let totalTokens = 0;
    let totalCost = 0;
    let totalRequests = 0;
    let totalAgents = 0;
    let totalSessions = 0;
    let totalCacheHit = 0;
    let totalInput = 0;
    const modelCounts = {};
    for (const event of events) {
        switch (event._type) {
            case "request": {
                const d = event.data;
                if (d.timestamp < cutoffIso)
                    continue;
                totalRequests++;
                totalTokens += d.inputTokens + d.outputTokens;
                totalInput += d.inputTokens;
                totalCacheHit += d.cacheHitTokens;
                modelCounts[d.model] = (modelCounts[d.model] ?? 0) + 1;
                break;
            }
            case "agent_complete": {
                const d = event.data;
                // Agent events don't have a direct timestamp; count all within period
                totalAgents++;
                modelCounts[d.model] = (modelCounts[d.model] ?? 0) + 1;
                break;
            }
            case "session_summary": {
                const d = event.data;
                if (d.startTime < cutoffIso)
                    continue;
                totalSessions++;
                totalCost += d.totalCost;
                break;
            }
            default:
                assertNever(event);
        }
    }
    const avgCacheHitRate = totalInput > 0 ? Math.round((totalCacheHit / totalInput) * 1000) / 1000 : 0;
    return {
        periodDays,
        totalTokens,
        totalCost: Math.round(totalCost * 100) / 100,
        totalRequests,
        totalAgents,
        totalSessions,
        avgCacheHitRate,
        modelDistribution: modelCounts,
    };
}
//# sourceMappingURL=token-tracker.js.map