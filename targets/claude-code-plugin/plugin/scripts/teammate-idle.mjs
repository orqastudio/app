#!/usr/bin/env node
// TeammateIdle hook — all matchers
//
// Thin daemon wrapper: team coordination when a teammate becomes idle.
// The daemon handles:
//   - Checking the task list for unassigned work
//   - Evaluating task dependencies and blockers
//   - Suggesting next task assignment
//
// No business logic here — all decisions are made by the daemon.

const DAEMON_PORT = (parseInt(process.env.ORQA_PORT_BASE || "10200", 10) || 10200) + 58;
const DAEMON_URL = `http://127.0.0.1:${DAEMON_PORT}`;

async function main() {
  let raw = "";
  for await (const chunk of process.stdin) raw += chunk;
  const input = JSON.parse(raw);

  const agentType = input.agent_type ?? "unknown";

  const context = {
    event: "TeammateIdle",
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
