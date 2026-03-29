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
/** Level 1 — Per-request metrics captured for every API call. */
export interface RequestMetrics {
    timestamp: string;
    agentId: string;
    taskId?: string;
    inputTokens: number;
    outputTokens: number;
    cacheHitTokens: number;
    reasoningTokens: number;
    model: string;
    latencyMs: number;
}
/** Level 2 — Per-agent aggregate metrics across an agent's lifetime. */
export interface AgentMetrics {
    agentId: string;
    role: string;
    model: string;
    totalInputTokens: number;
    totalOutputTokens: number;
    contextUtilization: number;
    requestCount: number;
    lifetimeMs: number;
}
/** Level 3 — Per-session aggregate metrics. */
export interface SessionMetrics {
    sessionId: string;
    startTime: string;
    totalTokens: number;
    totalCost: number;
    agentSpawns: number;
    overheadRatio: number;
    teamSpawnCost: number;
}
/** Discriminated union for JSONL events. */
export type MetricEvent = {
    _type: "request";
    data: RequestMetrics;
} | {
    _type: "agent_complete";
    data: AgentMetrics;
} | {
    _type: "session_summary";
    data: SessionMetrics;
};
/** Level 4 — Trend aggregate (computed, not stored). */
export interface TrendMetrics {
    periodDays: number;
    totalTokens: number;
    totalCost: number;
    totalRequests: number;
    totalAgents: number;
    totalSessions: number;
    avgCacheHitRate: number;
    modelDistribution: Record<string, number>;
}
/**
 * Get the path to the token metrics file for a project.
 * @param projectRoot - Absolute path to the project root.
 * @returns Absolute path to the token metrics JSONL file.
 */
export declare function getMetricsPath(projectRoot: string): string;
/**
 * Write a request-level metric event.
 * @param projectRoot - Absolute path to the project root.
 * @param metrics - The request metrics to record.
 */
export declare function recordRequest(projectRoot: string, metrics: RequestMetrics): void;
/**
 * Write an agent-complete metric event.
 * @param projectRoot - Absolute path to the project root.
 * @param metrics - The agent metrics to record.
 */
export declare function recordAgentComplete(projectRoot: string, metrics: AgentMetrics): void;
/**
 * Write a session-summary metric event.
 * @param projectRoot - Absolute path to the project root.
 * @param metrics - The session metrics to record.
 */
export declare function recordSessionSummary(projectRoot: string, metrics: SessionMetrics): void;
/**
 * Read all metric events from the JSONL file.
 * @param projectRoot - Absolute path to the project root.
 * @returns Array of all metric events in the file.
 */
export declare function readMetricEvents(projectRoot: string): MetricEvent[];
/**
 * Filter events by type.
 * @param events - Array of metric events to filter.
 * @param type - The event type to filter by.
 * @returns Array of events matching the specified type.
 */
export declare function filterEvents<T extends MetricEvent["_type"]>(events: MetricEvent[], type: T): Extract<MetricEvent, {
    _type: T;
}>[];
/**
 * Tracks token usage for an active session.
 *
 * Create one instance per orchestrator session. Call `trackRequest` for
 * each API response, `trackAgentComplete` when an agent finishes, and
 * `finalize` when the session ends.
 */
export declare class TokenTracker {
    readonly sessionId: string;
    readonly startTime: string;
    private readonly projectRoot;
    private totalInputTokens;
    private totalOutputTokens;
    private totalCacheHitTokens;
    private requestCount;
    private agentSpawns;
    private teamSpawnCost;
    /**
     * Create a new TokenTracker for a session.
     * @param projectRoot - Absolute path to the project root for writing metrics.
     * @param sessionId - Optional session ID; a UUID is generated if not provided.
     */
    constructor(projectRoot: string, sessionId?: string);
    /**
     * Record a single API request. Writes to JSONL immediately.
     * @param metrics - The request metrics to record.
     */
    trackRequest(metrics: RequestMetrics): void;
    /**
     * Record an agent completing. Writes to JSONL immediately.
     * @param metrics - The agent metrics to record.
     */
    trackAgentComplete(metrics: AgentMetrics): void;
    /**
     * Get current session totals without finalizing.
     * @returns Current session token counts and request statistics.
     */
    getSessionTotals(): {
        totalTokens: number;
        inputTokens: number;
        outputTokens: number;
        cacheHitTokens: number;
        requestCount: number;
        agentSpawns: number;
    };
    /**
     * Finalize the session. Writes summary event. Returns the summary.
     * @param costEstimate - Estimated total cost in USD for this session.
     * @returns The completed session metrics summary.
     */
    finalize(costEstimate: number): SessionMetrics;
}
/**
 * Compute trend metrics over a given period from historical JSONL data.
 *
 * Reads all events and filters to those within `periodDays` of now.
 * @param projectRoot - Absolute path to the project root.
 * @param periodDays - Number of days back to include in the trend window.
 * @returns Computed trend metrics for the specified period.
 */
export declare function computeTrends(projectRoot: string, periodDays: number): TrendMetrics;
//# sourceMappingURL=token-tracker.d.ts.map