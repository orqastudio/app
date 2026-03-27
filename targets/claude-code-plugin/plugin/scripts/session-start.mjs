#!/usr/bin/env node
// SessionStart hook — all matchers
//
// Thin daemon wrapper: initializes the session and loads project context.
// The daemon handles:
//   - Installation verification (plugin state, required files)
//   - Daemon health gate (blocks if daemon is unreachable)
//   - Graph integrity checks
//   - Session continuity (loads previous session state)
//   - Git state warnings (stashes, uncommitted changes)
//
// No business logic here — all decisions are made by the daemon.

import { existsSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const DAEMON_PORT = (parseInt(process.env.ORQA_PORT_BASE || "10200", 10) || 10200) + 58;
const DAEMON_URL = `http://127.0.0.1:${DAEMON_PORT}`;

async function main() {
  let raw = "";
  for await (const chunk of process.stdin) raw += chunk;
  const input = JSON.parse(raw);

  const projectDir = input.cwd ?? process.env.CLAUDE_PROJECT_DIR ?? ".";
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
    const msg = [
      "OrqaStudio daemon is not running. Rule enforcement requires the daemon.",
      "",
      "Start it with: orqa daemon start",
      `Daemon expected on port ${DAEMON_PORT}.`,
    ].join("\n");
    process.stderr.write(JSON.stringify({
      hookSpecificOutput: { permissionDecision: "deny" },
      systemMessage: msg,
    }));
    process.exit(2);
  }

  // Call daemon for session initialization
  let result;
  try {
    const res = await fetch(`${DAEMON_URL}/hook`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ event: "SessionStart" }),
      signal: AbortSignal.timeout(10000),
    });
    if (!res.ok) throw new Error(`daemon returned ${res.status}`);
    result = await res.json();
  } catch {
    result = { messages: [] };
  }

  // Load session continuity context
  const parts = [];

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
    process.stdout.write(JSON.stringify({
      systemMessage: parts.join("\n\n"),
    }));
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
