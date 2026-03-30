#!/usr/bin/env node
/**
 * PreCompact hook — all matchers
 *
 * Thin daemon wrapper: preserves critical governance context before context
 * window compaction. The daemon queries active epics and in-progress tasks,
 * writes .state/governance-context.md for recovery, and builds recovery
 * instructions. No business logic here — all decisions are made by the daemon.
 */

import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { readInput, callDaemon, outputWarn } from "../hooks/shared.js";

/** Extended result type for the PreCompact hook, which may return a context blob. */
interface PreCompactResult {
  action: "allow" | "block" | "warn";
  messages: string[];
  violations: Array<{ rule_id: string; action: string; message: string }>;
  /** Markdown content to persist as .state/governance-context.md. */
  context?: string;
}

/** Run the PreCompact hook. */
async function main(): Promise<void> {
  const input = await readInput();

  const projectDir = input.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";
  const stateDir = join(projectDir, ".state");
  mkdirSync(stateDir, { recursive: true });

  let result: PreCompactResult;
  try {
    result = await callDaemon<PreCompactResult>("/hook", { event: "PreCompact" });
  } catch {
    process.exit(0);
  }

  // Write governance context file if daemon returned content
  if (result.context) {
    writeFileSync(join(stateDir, "governance-context.md"), result.context);
  }

  if (result.messages?.length > 0) {
    outputWarn(result.messages);
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
