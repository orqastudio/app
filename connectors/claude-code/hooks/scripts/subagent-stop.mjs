#!/usr/bin/env node
// SubagentStop hook — all matchers
//
// Thin daemon wrapper: reviews subagent output when a subagent completes.
// The daemon handles:
//   - Stub detection (incomplete implementations)
//   - Deferral scanning (silently deferred acceptance criteria)
//   - Artifact integrity checks
//   - Findings file validation
//
// No business logic here — all decisions are made by the daemon.

const DAEMON_PORT = parseInt(process.env.ORQA_PORT_BASE || "10100", 10) || 10100;
const DAEMON_URL = `http://127.0.0.1:${DAEMON_PORT}`;

async function main() {
  let raw = "";
  for await (const chunk of process.stdin) raw += chunk;
  const input = JSON.parse(raw);

  const agentType = input.agent_type ?? "unknown";

  const context = {
    event: "SubagentStop",
    agent_type: agentType,
  };

  let result;
  try {
    const res = await fetch(`${DAEMON_URL}/hook`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(context),
      signal: AbortSignal.timeout(12000),
    });
    if (!res.ok) throw new Error(`daemon returned ${res.status}`);
    result = await res.json();
  } catch {
    // Daemon unavailable — fail-open
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
