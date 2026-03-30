#!/usr/bin/env node
// PostToolUse hook — Write|Edit, TaskUpdate
//
// Thin daemon wrapper: after a tool completes, notifies the daemon to:
//   - Validate written artifacts against the composed schema
//   - Track task completions and update telemetry
//   - Check findings files when tasks are marked complete
//
// No business logic here — all decisions are made by the daemon.

const DAEMON_PORT = parseInt(process.env.ORQA_PORT_BASE || "10100", 10) || 10100;
const DAEMON_URL = `http://127.0.0.1:${DAEMON_PORT}`;

async function main() {
  let raw = "";
  for await (const chunk of process.stdin) raw += chunk;
  const input = JSON.parse(raw);

  const toolName = input.tool_name ?? "";
  const toolInput = input.tool_input ?? {};
  const projectDir = input.cwd ?? process.env.CLAUDE_PROJECT_DIR ?? ".";

  const context = {
    event: "PostAction",
    tool_name: toolName,
    tool_input: toolInput,
    file_path: toolInput.file_path ?? "",
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
