#!/usr/bin/env node
// UserPromptSubmit hook — all matchers
//
// Thin daemon wrapper: classifies the user prompt and injects workflow context.
// The daemon handles:
//   - Prompt classification (semantic search or keyword fallback)
//   - Prompt pipeline execution (knowledge/rule injection per workflow stage)
//   - Daemon health gate (blocks if daemon is unreachable)
//
// No business logic here — all decisions are made by the daemon.

const DAEMON_PORT = (parseInt(process.env.ORQA_PORT_BASE || "10200", 10) || 10200) + 58;
const DAEMON_URL = `http://127.0.0.1:${DAEMON_PORT}`;

async function main() {
  let raw = "";
  for await (const chunk of process.stdin) raw += chunk;
  const input = JSON.parse(raw);

  const userMessage = input.user_message ?? input.prompt ?? "";
  const projectDir = input.cwd ?? process.env.CLAUDE_PROJECT_DIR ?? ".";
  const agentType = input.agent_type ?? "orchestrator";

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
      "",
      `Daemon expected on port ${DAEMON_PORT}.`,
    ].join("\n");
    process.stderr.write(JSON.stringify({
      hookSpecificOutput: { permissionDecision: "deny" },
      systemMessage: msg,
    }));
    process.exit(2);
  }

  if (!userMessage) {
    process.exit(0);
  }

  // Call the prompt pipeline via the daemon
  const context = {
    event: "PromptSubmit",
    user_message: userMessage,
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
    process.exit(0);
  }

  if (result.messages?.length > 0) {
    process.stdout.write(JSON.stringify({
      systemMessage: result.messages.join("\n"),
    }));
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
