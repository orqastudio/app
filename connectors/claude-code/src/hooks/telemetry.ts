// Telemetry for hook scripts.
//
// Forwards hook execution metrics to the dev controller dashboard.
// URL defaults to localhost:10401/log but can be overridden by ORQA_TELEMETRY_URL.
// Fire-and-forget — never blocks or throws.

import type { TelemetryDetails } from "../types.js";

/**
 * Resolve the telemetry dashboard URL from ORQA_TELEMETRY_URL env var.
 * Defaults to the dev controller at localhost:10401/log.
 * @returns Dashboard URL to POST telemetry events to.
 */
function getDashboardUrl(): string {
  const override = process.env["ORQA_TELEMETRY_URL"];
  if (override !== undefined && override !== "") return override;
  return "http://localhost:10401/log";
}

const DASHBOARD_URL = getDashboardUrl();

/**
 * Log hook execution metrics to the dev controller dashboard.
 * @param hook - Hook script name (e.g. "prompt-injector").
 * @param event - Claude Code event name (e.g. "UserPromptSubmit").
 * @param startTime - Unix timestamp in ms when the hook started (from Date.now()).
 * @param outcome - Result string: "injected", "blocked", "error", etc.
 * @param details - Additional key-value data to include in the log entry.
 * @param _projectDir - Reserved for future dashboard routing; unused.
 */
export function logTelemetry(
  hook: string,
  event: string,
  startTime: number,
  outcome: string,
  details: TelemetryDetails,
  _projectDir?: string,
): void {
  const durationMs = Date.now() - startTime;
  const level = outcome === "error" ? "error" : outcome === "blocked" ? "warn" : "info";
  const message = `[${hook}] ${event}: ${outcome} (${durationMs}ms)`;

  try {
    fetch(DASHBOARD_URL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ source: `hook:${hook}`, level, message, data: details }),
    }).catch(() => {});
  } catch {
    // Never fail — telemetry must not break hooks
  }
}
