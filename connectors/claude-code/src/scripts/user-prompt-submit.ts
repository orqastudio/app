#!/usr/bin/env node
/**
 * UserPromptSubmit hook — all matchers
 *
 * Thin daemon wrapper: classifies the user prompt and injects workflow context.
 * Performs a daemon health gate (blocks if the daemon is unreachable), then
 * delegates prompt classification and pipeline execution (knowledge/rule
 * injection per workflow stage) to the daemon.
 * No business logic here — all decisions are made by the daemon.
 */

import { getPort } from "@orqastudio/constants";
import { readInput, callDaemon, outputBlock, outputWarn } from "../hooks/shared.js";
import type { HookResult } from "../hooks/shared.js";

const DAEMON_PORT = getPort("daemon");
const DAEMON_URL = `http://127.0.0.1:${DAEMON_PORT}`;

/** Run the UserPromptSubmit hook. */
async function main(): Promise<void> {
  const input = await readInput();

  const userMessage = input.user_message ?? input.prompt ?? "";
  const agentType = input.agent_type ?? "orchestrator";

  // Daemon health gate — block if unreachable
  try {
    await fetch(`${DAEMON_URL}/health`, {
      signal: AbortSignal.timeout(2000),
    });
  } catch {
    outputBlock([
      "OrqaStudio daemon is not running. Rule enforcement requires the daemon.",
      "",
      "Start it with: orqa daemon start",
      "",
      `Daemon expected on port ${DAEMON_PORT}.`,
    ]);
  }

  if (!userMessage) {
    process.exit(0);
  }

  // Call the prompt pipeline via the daemon
  const context = {
    event: "PromptSubmit" as const,
    user_message: userMessage,
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
