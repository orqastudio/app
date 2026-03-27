#!/usr/bin/env node
// PreCompact hook — all matchers
//
// Thin daemon wrapper: preserves critical governance context before
// context window compaction. The daemon handles:
//   - Querying active epics and in-progress tasks
//   - Writing .state/governance-context.md for recovery
//   - Building recovery instructions
//
// No business logic here — all decisions are made by the daemon.

import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const DAEMON_PORT = (parseInt(process.env.ORQA_PORT_BASE || "10200", 10) || 10200) + 58;
const DAEMON_URL = `http://127.0.0.1:${DAEMON_PORT}`;

async function main() {
  let raw = "";
  for await (const chunk of process.stdin) raw += chunk;
  const input = JSON.parse(raw);

  const projectDir = input.cwd ?? process.env.CLAUDE_PROJECT_DIR ?? ".";
  const stateDir = join(projectDir, ".state");
  mkdirSync(stateDir, { recursive: true });

  let result;
  try {
    const res = await fetch(`${DAEMON_URL}/hook`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ event: "PreCompact" }),
      signal: AbortSignal.timeout(8000),
    });
    if (!res.ok) throw new Error(`daemon returned ${res.status}`);
    result = await res.json();
  } catch {
    process.exit(0);
  }

  // Write governance context file if daemon returned content
  if (result.context) {
    writeFileSync(join(stateDir, "governance-context.md"), result.context);
  }

  if (result.messages?.length > 0) {
    process.stdout.write(JSON.stringify({
      systemMessage: result.messages.join("\n"),
    }));
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
