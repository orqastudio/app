#!/usr/bin/env node
// Shared telemetry helper for hook scripts.
//
// Appends a JSON-lines entry to tmp/hook-metrics.json and posts a log line
// to the dev controller dashboard at localhost:3001/log (non-blocking).
//
// Usage:
//   import { logTelemetry } from "./telemetry.mjs";
//   const startTime = Date.now();
//   // ... hook logic ...
//   logTelemetry("hook-name", "EventType", startTime, "success", { ... });

import { appendFileSync, mkdirSync, existsSync } from "fs";
import { join } from "path";

/**
 * Log hook execution metrics.
 *
 * @param {string} hook         - Hook script name (e.g. "prompt-injector")
 * @param {string} event        - Claude Code hook event (e.g. "UserPromptSubmit")
 * @param {number} startTime    - Date.now() value captured before hook logic ran
 * @param {string} outcome      - Short outcome label (e.g. "injected", "blocked", "clean", "error")
 * @param {Record<string, unknown>} details - Hook-specific metrics
 * @param {string} [projectDir] - Project root directory (defaults to process.cwd())
 */
export function logTelemetry(hook, event, startTime, outcome, details, projectDir) {
  const entry = {
    timestamp: new Date().toISOString(),
    hook,
    event,
    duration_ms: Date.now() - startTime,
    outcome,
    details,
  };

  // Append to tmp/hook-metrics.json (JSON Lines format)
  const dir = projectDir || process.cwd();
  const tmpDir = join(dir, "tmp");
  try {
    if (!existsSync(tmpDir)) {
      mkdirSync(tmpDir, { recursive: true });
    }
    appendFileSync(join(tmpDir, "hook-metrics.json"), JSON.stringify(entry) + "\n", "utf-8");
  } catch {
    // Silently ignore filesystem errors — telemetry must never break hooks
  }

  // Post to dev controller dashboard (fire-and-forget, non-blocking)
  try {
    const level = outcome === "error" ? "error" : "info";
    const message = `[${hook}] ${outcome} (${entry.duration_ms}ms) ${JSON.stringify(details)}`;
    fetch("http://localhost:3001/log", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ source: `hook:${hook}`, level, message }),
    }).catch(() => {});
  } catch {
    // Silently ignore network errors — dashboard may not be running
  }
}
