// PreToolUse hook — Write | Edit
//
// Thin adapter: delegates memory-path enforcement to the daemon.
// The daemon decides which memory paths should be redirected to lessons.

import { readInput, callDaemon, mapEvent, outputBlock, outputWarn, outputAllow } from "./shared.js";
import type { HookContext, HookResult } from "./shared.js";

async function main(): Promise<void> {
  let hookInput;
  try {
    hookInput = await readInput();
  } catch {
    process.exit(0);
  }

  const context: HookContext = {
    event: mapEvent("PreToolUse"),
    tool_name: hookInput.tool_name,
    tool_input: hookInput.tool_input,
  };

  let result: HookResult;
  try {
    result = await callDaemon<HookResult>("/hook", context);
  } catch {
    // Daemon unavailable — fail-open (daemon-gate.sh blocks sessions without daemon)
    outputAllow();
  }

  if (result.action === "block") {
    outputBlock(result.messages);
  } else if (result.action === "warn" && result.messages.length > 0) {
    outputWarn(result.messages);
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
