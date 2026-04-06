/**
 * Metrics command — display token usage and cost metrics.
 *
 * orqa metrics [--session] [--trends] [--period <days>] [--agents] [--json]
 *
 * Reads .state/token-metrics.jsonl and outputs a summary.
 */
import { getRoot } from "../lib/root.js";
import { readMetricEvents, filterEvents, computeTrends, } from "../lib/token-tracker.js";
import { estimateCost, inferModelTier } from "../lib/budget-enforcer.js";
const USAGE = `
Usage: orqa metrics [options]

Display token usage and cost metrics from .state/token-metrics.jsonl.

Options:
  --session        Show current/latest session summary (default)
  --trends         Show trend metrics over a period
  --period <days>  Trend period in days (default: 7)
  --agents         Show top agents by cost
  --json           Output as JSON instead of formatted text
  --help, -h       Show this help message
`.trim();
function parseOptions(args) {
    let showSession = false;
    let showTrends = false;
    let showAgents = false;
    let periodDays = 7;
    let jsonOutput = false;
    for (let i = 0; i < args.length; i++) {
        switch (args[i]) {
            case "--session":
                showSession = true;
                break;
            case "--trends":
                showTrends = true;
                break;
            case "--agents":
                showAgents = true;
                break;
            case "--period":
                if (i + 1 < args.length) {
                    periodDays = parseInt(args[i + 1], 10) || 7;
                    i++;
                }
                break;
            case "--json":
                jsonOutput = true;
                break;
        }
    }
    // Default: show session if nothing specified
    if (!showSession && !showTrends && !showAgents) {
        showSession = true;
        showAgents = true;
    }
    return { showSession, showTrends, showAgents, periodDays, jsonOutput };
}
function formatCost(usd) {
    return `$${usd.toFixed(4)}`;
}
function formatTokens(n) {
    if (n >= 1_000_000)
        return `${(n / 1_000_000).toFixed(1)}M`;
    if (n >= 1_000)
        return `${(n / 1_000).toFixed(1)}k`;
    return String(n);
}
function formatPercent(ratio) {
    return `${(ratio * 100).toFixed(1)}%`;
}
function printSessionSummary(sessions) {
    if (sessions.length === 0) {
        console.log("No session data found.");
        return;
    }
    const latest = sessions[sessions.length - 1];
    console.log("Session Summary (latest)");
    console.log("------------------------");
    console.log(`  Session ID:     ${latest.sessionId}`);
    console.log(`  Start time:     ${latest.startTime}`);
    console.log(`  Total tokens:   ${formatTokens(latest.totalTokens)}`);
    console.log(`  Estimated cost: ${formatCost(latest.totalCost)}`);
    console.log(`  Agent spawns:   ${latest.agentSpawns}`);
    console.log(`  Overhead ratio: ${formatPercent(latest.overheadRatio)}`);
    console.log(`  Spawn cost:     ${formatTokens(latest.teamSpawnCost)} tokens`);
    console.log();
}
function printAgentSummary(agents) {
    if (agents.length === 0) {
        console.log("No agent data found.");
        return;
    }
    // Sort by total tokens descending
    const sorted = [...agents].sort((a, b) => b.totalInputTokens + b.totalOutputTokens - (a.totalInputTokens + a.totalOutputTokens));
    console.log("Top Agents by Token Usage");
    console.log("-------------------------");
    const top = sorted.slice(0, 10);
    for (const agent of top) {
        const total = agent.totalInputTokens + agent.totalOutputTokens;
        const tier = inferModelTier(agent.model);
        const cost = estimateCost(tier, agent.totalInputTokens, agent.totalOutputTokens);
        console.log(`  ${agent.agentId.substring(0, 12)}  ${agent.role.padEnd(14)} ${formatTokens(total).padStart(8)}  ${formatCost(cost).padStart(8)}  ${agent.model}  ctx:${formatPercent(agent.contextUtilization)}`);
    }
    console.log();
}
function printTrends(projectRoot, periodDays) {
    const trends = computeTrends(projectRoot, periodDays);
    console.log(`Trends (last ${periodDays} days)`);
    console.log("-------------------");
    console.log(`  Total tokens:    ${formatTokens(trends.totalTokens)}`);
    console.log(`  Total cost:      ${formatCost(trends.totalCost)}`);
    console.log(`  Requests:        ${trends.totalRequests}`);
    console.log(`  Agents:          ${trends.totalAgents}`);
    console.log(`  Sessions:        ${trends.totalSessions}`);
    console.log(`  Avg cache rate:  ${formatPercent(trends.avgCacheHitRate)}`);
    if (Object.keys(trends.modelDistribution).length > 0) {
        console.log("  Model distribution:");
        const total = Object.values(trends.modelDistribution).reduce((s, n) => s + n, 0);
        for (const [model, count] of Object.entries(trends.modelDistribution)) {
            const pct = total > 0 ? ((count / total) * 100).toFixed(1) : "0.0";
            console.log(`    ${model}: ${count} (${pct}%)`);
        }
    }
    console.log();
}
/**
 * Dispatch the metrics command: show artifact and enforcement metrics.
 * @param args - CLI arguments after "metrics".
 */
export async function runMetricsCommand(args) {
    if (args.includes("--help") || args.includes("-h")) {
        console.log(USAGE);
        return;
    }
    const projectRoot = getRoot();
    const options = parseOptions(args);
    const events = readMetricEvents(projectRoot);
    if (events.length === 0) {
        console.log("No metrics data found in .state/token-metrics.jsonl");
        console.log("Metrics are recorded during orchestrated agent sessions.");
        return;
    }
    if (options.jsonOutput) {
        const output = {};
        if (options.showSession) {
            const sessions = filterEvents(events, "session_summary").map((e) => e.data);
            output.session = sessions.length > 0 ? sessions[sessions.length - 1] : null;
        }
        if (options.showAgents) {
            output.agents = filterEvents(events, "agent_complete").map((e) => e.data);
        }
        if (options.showTrends) {
            output.trends = computeTrends(projectRoot, options.periodDays);
        }
        console.log(JSON.stringify(output, null, 2));
        return;
    }
    if (options.showSession) {
        const sessions = filterEvents(events, "session_summary").map((e) => e.data);
        printSessionSummary(sessions);
    }
    if (options.showAgents) {
        const agents = filterEvents(events, "agent_complete").map((e) => e.data);
        printAgentSummary(agents);
    }
    if (options.showTrends) {
        printTrends(projectRoot, options.periodDays);
    }
}
//# sourceMappingURL=metrics.js.map