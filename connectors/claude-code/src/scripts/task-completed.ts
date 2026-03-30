#!/usr/bin/env node
/**
 * TaskCompleted hook — all matchers
 *
 * Thin daemon wrapper: team coordination when a task is marked complete. The
 * daemon verifies all acceptance criteria are met, checks the findings file
 * exists and is complete, evaluates if dependent tasks are now unblocked, and
 * suggests next task assignment. Blocks task completion if criteria are unmet.
 * No business logic here — all decisions are made by the daemon.
 */

import { readInput, callDaemon, outputBlock, outputWarn } from "../hooks/shared.js";
import type { HookResult } from "../hooks/shared.js";

/** Run the TaskCompleted hook. */
async function main(): Promise<void> {
  const input = await readInput();

  const agentType = input.agent_type ?? "unknown";
  const toolInput = input.tool_input ?? {};

  const context = {
    event: "TaskCompleted" as const,
    agent_type: agentType,
    tool_input: toolInput,
  };

  let result: HookResult;
  try {
    result = await callDaemon<HookResult>("/hook", context);
  } catch {
    process.exit(0);
  }

  if (result.action === "block") {
    const messages = result.messages?.length
      ? result.messages
      : ["Task completion blocked — acceptance criteria not met."];
    outputBlock(messages);
  }

  if (result.messages?.length > 0) {
    outputWarn(result.messages);
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
