#!/usr/bin/env node
// Stop hook — all matchers
//
// Thin daemon wrapper: saves session state before the session ends.
// The daemon handles:
//   - Persisting active work items (epics, tasks, decisions)
//   - Writing session state for continuity across sessions
//   - Cleaning up session guard file
//
// No business logic here — all decisions are made by the daemon.

import { existsSync, unlinkSync } from "node:fs";
import { join } from "node:path";

const DAEMON_PORT = (parseInt(process.env.ORQA_PORT_BASE || "10200", 10) || 10200) + 58;
const DAEMON_URL = `http://127.0.0.1:${DAEMON_PORT}`;

async function main() {
  let raw = "";
  for await (const chunk of process.stdin) raw += chunk;
  const input = JSON.parse(raw);

  const projectDir = input.cwd ?? process.env.CLAUDE_PROJECT_DIR ?? ".";
  const stateDir = join(projectDir, ".state");

  // Call daemon to save session state
  try {
    const res = await fetch(`${DAEMON_URL}/hook`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ event: "SessionEnd" }),
      signal: AbortSignal.timeout(8000),
    });
    if (!res.ok) throw new Error(`daemon returned ${res.status}`);
    const result = await res.json();

    if (result.messages?.length > 0) {
      process.stdout.write(JSON.stringify({
        systemMessage: result.messages.join("\n"),
      }));
    }
  } catch {
    // Daemon unavailable — best-effort, don't block shutdown
  }

  // Clean up session guard
  const guardFile = join(stateDir, ".session-started");
  if (existsSync(guardFile)) {
    try { unlinkSync(guardFile); } catch { /* ignore */ }
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
