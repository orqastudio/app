// SubagentStop hook — all matchers
//
// Thin adapter: delegates subagent review (stub detection, deferral
// scanning, artifact integrity) to the daemon. The daemon handles all
// enforcement logic; this hook just forwards the context and outputs
// the result.

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

  const projectDir = hookInput.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";
  const agentType = hookInput.agent_type ?? "unknown";

  const context: HookContext = {
    event: mapEvent("SubagentStop"),
    agent_type: agentType,
  };

  let result: HookResult;
  try {
    result = await callDaemon<HookResult>("/hook", context);
  } catch {
    // Daemon unavailable — fail-open (daemon-gate.sh blocks sessions without daemon)
    logTelemetry("subagent-review", "SubagentStop", startTime, "unavailable", {
      agent_type: agentType,
    }, projectDir);
    outputAllow();
  }

  logTelemetry("subagent-review", "SubagentStop", startTime, result.action, {
    agent_type: agentType,
    violations: result.violations?.length ?? 0,
  }, projectDir);

  if (result.messages.length > 0) {
    process.stdout.write(JSON.stringify({ systemMessage: result.messages.join("\n") }));
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
