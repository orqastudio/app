// Telemetry for hook scripts.
//
// Forwards hook execution metrics to the dev controller dashboard at
// localhost:3001/log. Fire-and-forget — never blocks or throws.
const DASHBOARD_URL = "http://localhost:3001/log";
/**
 * Log hook execution metrics to the dev controller dashboard.
 */
export function logTelemetry(hook, event, startTime, outcome, details, _projectDir) {
    const durationMs = Date.now() - startTime;
    const level = outcome === "error" ? "error" : outcome === "blocked" ? "warn" : "info";
    const message = `[${hook}] ${event}: ${outcome} (${durationMs}ms)`;
    try {
        fetch(DASHBOARD_URL, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ source: `hook:${hook}`, level, message, data: details }),
        }).catch(() => { });
    }
    catch {
        // Never fail — telemetry must not break hooks
    }
}
//# sourceMappingURL=telemetry.js.map