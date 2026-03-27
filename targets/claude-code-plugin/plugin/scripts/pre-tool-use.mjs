#!/usr/bin/env node
// PreToolUse hook — Write|Edit|Bash
//
// Thin daemon wrapper: validates artifact operations and file access
// enforcement before tool execution. The daemon checks:
//   - Agent role has permission to modify the target path
//   - Artifact schema compliance for .orqa/ writes
//   - Rule engine evaluation for the proposed action
//
// No business logic here — all decisions are made by the daemon.

import { execFileSync } from "node:child_process";

const DAEMON_PORT = (parseInt(process.env.ORQA_PORT_BASE || "10200", 10) || 10200) + 58;
const DAEMON_URL = `http://127.0.0.1:${DAEMON_PORT}`;

async function main() {
  let raw = "";
  for await (const chunk of process.stdin) raw += chunk;
  const input = JSON.parse(raw);

  const toolName = input.tool_name ?? "";
  const toolInput = input.tool_input ?? {};
  const projectDir = input.cwd ?? process.env.CLAUDE_PROJECT_DIR ?? ".";
  const agentType = input.agent_type ?? "orchestrator";

  const context = {
    event: "PreAction",
    tool_name: toolName,
    tool_input: toolInput,
    file_path: toolInput.file_path ?? toolInput.command ?? "",
    agent_type: agentType,
  };

  let result;
  try {
    const res = await fetch(`${DAEMON_URL}/hook`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(context),
      signal: AbortSignal.timeout(8000),
    });
    if (!res.ok) throw new Error(`daemon returned ${res.status}`);
    result = await res.json();
  } catch {
    // Daemon unavailable — fail-open (session-start blocks sessions without daemon)
    process.exit(0);
  }

  if (result.action === "block") {
    const msg = result.messages?.join("\n") ?? "Action blocked by governance rules.";
    process.stderr.write(JSON.stringify({
      hookSpecificOutput: { permissionDecision: "deny" },
      systemMessage: msg,
    }));
    process.exit(2);
  }

  if (result.action === "warn" && result.messages?.length > 0) {
    process.stdout.write(JSON.stringify({
      systemMessage: result.messages.join("\n"),
    }));
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
