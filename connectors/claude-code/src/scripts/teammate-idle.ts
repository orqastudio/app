#!/usr/bin/env node
/**
 * TeammateIdle hook — all matchers
 *
 * Thin daemon wrapper: team coordination when a teammate becomes idle. The
 * daemon checks the task list for unassigned work, evaluates task dependencies
 * and blockers, and suggests next task assignment.
 * No business logic here — all decisions are made by the daemon.
 */

import { readInput, callDaemon, outputWarn } from "../hooks/shared.js";
import type { HookResult } from "../hooks/shared.js";

/** Run the TeammateIdle hook. */
async function main(): Promise<void> {
  const input = await readInput();

  const agentType = input.agent_type ?? "unknown";

  const context = {
    event: "TeammateIdle" as const,
    agent_type: agentType,
  };

  let result: HookResult;
  try {
    result = await callDaemon<HookResult>("/hook", context);
  } catch {
    process.exit(0);
  }

  if (result.messages?.length > 0) {
    outputWarn(result.messages);
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
