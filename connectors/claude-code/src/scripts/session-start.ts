#!/usr/bin/env node
/**
 * SessionStart hook — all matchers
 *
 * Thin daemon wrapper: initializes the session and loads project context.
 * Performs a daemon health gate (blocks the session if the daemon is unreachable),
 * then delegates all session initialization to the daemon, and injects the
 * previous session state into the context window if it exists.
 * No business logic here — all decisions are made by the daemon.
 */

import { existsSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { getPort } from "@orqastudio/constants";
import { readInput, callDaemon, outputBlock } from "../hooks/shared.js";
import type { HookResult } from "../hooks/shared.js";

const DAEMON_PORT = getPort("daemon");
const DAEMON_URL = `http://127.0.0.1:${DAEMON_PORT}`;

/** Run the SessionStart hook. */
async function main(): Promise<void> {
  const input = await readInput();

  const projectDir = input.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";
  const stateDir = join(projectDir, ".state");

  // Session guard — only run once per session
  const guardFile = join(stateDir, ".session-started");
  if (existsSync(guardFile)) {
    process.exit(0);
  }
  mkdirSync(stateDir, { recursive: true });
  writeFileSync(guardFile, new Date().toISOString());

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
      `Daemon expected on port ${DAEMON_PORT}.`,
    ]);
  }

  // Call daemon for session initialization
  let result: HookResult;
  try {
    result = await callDaemon<HookResult>("/hook", { event: "SessionStart" });
  } catch {
    result = { action: "allow", messages: [], violations: [] };
  }

  // Load session continuity context
  const parts: string[] = [];

  if (result.messages?.length > 0) {
    parts.push(result.messages.join("\n"));
  }

  const sessionStatePath = join(stateDir, "session-state.md");
  if (existsSync(sessionStatePath)) {
    const sessionState = readFileSync(sessionStatePath, "utf-8");
    parts.push("=== PREVIOUS SESSION STATE ===");
    parts.push(sessionState);
    parts.push("=== END SESSION STATE ===");
    parts.push("Read the session state above. Resume where the previous session left off.");
  }

  if (parts.length > 0) {
    process.stdout.write(JSON.stringify({ systemMessage: parts.join("\n\n") }));
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
