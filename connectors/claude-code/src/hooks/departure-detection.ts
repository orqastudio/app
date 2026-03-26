// UserPromptSubmit hook — departure detection
//
// Thin adapter: delegates departure-pattern matching to the daemon.
// The daemon detects departure signals and returns the appropriate
// session-state reminder message.

import { readInput, callDaemon, mapEvent, outputAllow } from "./shared.js";
import type { HookContext, HookResult } from "./shared.js";
import { logTelemetry } from "./telemetry.js";

async function main(): Promise<void> {
  const startTime = Date.now();

  let hookInput;
  try {
    hookInput = await readInput();
  } catch {
    process.exit(0);
  }

  const userMessage = hookInput.user_message ?? hookInput.prompt ?? "";
  const projectDir = hookInput.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";

  if (!userMessage) {
    outputAllow();
  }

  const context: HookContext = {
    event: mapEvent("UserPromptSubmit"),
    user_message: userMessage,
  };

  let result: HookResult;
  try {
    result = await callDaemon<HookResult>("/hook", context);
  } catch {
    // Daemon unavailable — fail-open
    logTelemetry("departure-detection", "UserPromptSubmit", startTime, "unavailable", {}, projectDir);
    outputAllow();
  }

  logTelemetry(
    "departure-detection", "UserPromptSubmit", startTime,
    result.action === "warn" ? "departure-detected" : "no-match",
    { query: userMessage.slice(0, 100) },
    projectDir,
  );

  if (result.messages.length > 0) {
    process.stdout.write(JSON.stringify({ systemMessage: result.messages.join("\n") }));
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
