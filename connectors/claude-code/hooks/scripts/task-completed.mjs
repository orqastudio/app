#!/usr/bin/env node
// TaskCompleted hook — all matchers
//
// Thin daemon wrapper: team coordination when a task is marked complete.
// The daemon handles:
//   - Verifying all acceptance criteria are met
//   - Checking findings file exists and is complete
//   - Evaluating if dependent tasks are now unblocked
//   - Suggesting next task assignment
//
// No business logic here — all decisions are made by the daemon.

const DAEMON_PORT = parseInt(process.env.ORQA_PORT_BASE || "10100", 10) || 10100;
const DAEMON_URL = `http://127.0.0.1:${DAEMON_PORT}`;

async function main() {
  let raw = "";
  for await (const chunk of process.stdin) raw += chunk;
  const input = JSON.parse(raw);

  const agentType = input.agent_type ?? "unknown";
  const toolInput = input.tool_input ?? {};

  const context = {
    event: "TaskCompleted",
    agent_type: agentType,
    tool_input: toolInput,
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

  if (result.action === "block") {
    const msg = result.messages?.join("\n") ?? "Task completion blocked — acceptance criteria not met.";
    process.stderr.write(JSON.stringify({
      hookSpecificOutput: { permissionDecision: "deny" },
      systemMessage: msg,
    }));
    process.exit(2);
  }

  if (result.messages?.length > 0) {
    process.stdout.write(JSON.stringify({
      systemMessage: result.messages.join("\n"),
    }));
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
