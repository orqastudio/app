// PostToolUse hook — TaskUpdate
//
// Thin adapter: delegates findings-file existence check and deferral
// detection to the daemon. The daemon enforces RULE-04684a16.

import { readInput, callDaemon, mapEvent, outputBlock, outputWarn, outputAllow } from "./shared.js";
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

  const projectDir = hookInput.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";

  const context: HookContext = {
    event: mapEvent("PostToolUse"),
    tool_name: hookInput.tool_name,
    tool_input: hookInput.tool_input,
  };

  let result: HookResult;
  try {
    result = await callDaemon<HookResult>("/hook", context);
  } catch {
    // Daemon unavailable — fail-open (daemon-gate.sh blocks sessions without daemon)
    logTelemetry("findings-check", "PostToolUse", startTime, "unavailable", {}, projectDir);
    outputAllow();
  }

  logTelemetry(
    "findings-check", "PostToolUse", startTime,
    result.action,
    { violations: result.violations?.length ?? 0 },
    projectDir,
  );

  if (result.action === "block") {
    outputBlock(result.messages);
  } else if (result.action === "warn" && result.messages.length > 0) {
    outputWarn(result.messages);
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
