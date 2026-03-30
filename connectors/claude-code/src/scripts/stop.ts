#!/usr/bin/env node
/**
 * Stop hook — all matchers
 *
 * Thin daemon wrapper: saves session state before the session ends. The daemon
 * persists active work items, writes session state for continuity, and cleans
 * up session artifacts. No business logic here — all decisions are made by the daemon.
 */

import { existsSync, unlinkSync } from "node:fs";
import { join } from "node:path";
import { readInput, callDaemon, outputWarn } from "../hooks/shared.js";
import type { HookResult } from "../hooks/shared.js";

/** Run the Stop hook. */
async function main(): Promise<void> {
  const input = await readInput();

  const projectDir = input.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";
  const stateDir = join(projectDir, ".state");

  // Call daemon to save session state
  try {
    const result = await callDaemon<HookResult>("/hook", { event: "SessionEnd" });
    if (result.messages?.length > 0) {
      outputWarn(result.messages);
    }
  } catch {
    // Daemon unavailable — best-effort, don't block shutdown
  }

  // Clean up session guard
  const guardFile = join(stateDir, ".session-started");
  if (existsSync(guardFile)) {
    try {
      unlinkSync(guardFile);
    } catch {
      // Ignore cleanup errors
    }
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
