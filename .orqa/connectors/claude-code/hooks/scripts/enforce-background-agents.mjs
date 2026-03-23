/**
 * PreToolUse hook: enforce background agent team usage.
 *
 * When the orchestrator spawns an Agent tool call without
 * `run_in_background: true`, this hook injects a systemMessage
 * warning. The call is never blocked — warn only.
 *
 * stdin  — JSON: { tool_name, tool_input }
 * stdout — JSON: { decision?, systemMessage? }
 */

/** Default: allow with no message. */
const allow = () => {
  process.stdout.write(JSON.stringify({}));
};

/**
 * Read all of stdin as a string.  Works on both Unix (/dev/stdin)
 * and Windows (where /dev/stdin does not exist).
 */
function readStdin() {
  return new Promise((resolve, reject) => {
    const chunks = [];
    process.stdin.setEncoding("utf-8");
    process.stdin.on("data", (chunk) => chunks.push(chunk));
    process.stdin.on("end", () => resolve(chunks.join("")));
    process.stdin.on("error", reject);
  });
}

const raw = (await readStdin()).trim();

if (!raw) {
  allow();
  process.exit(0);
}

let payload;
try {
  payload = JSON.parse(raw);
} catch {
  // Malformed input — let the call through silently.
  allow();
  process.exit(0);
}

const { tool_name, tool_input } = payload;

// Only act on Agent tool calls.
if (tool_name !== "Agent") {
  allow();
  process.exit(0);
}

const runInBackground = tool_input?.run_in_background;

if (runInBackground) {
  // Already configured correctly — no warning needed.
  allow();
  process.exit(0);
}

// Agent call without run_in_background — emit a warning.
const warning = [
  "RULE-00a8c660 / RULE-532100d9: Agent tool called without `run_in_background: true`.",
  "Background execution keeps the orchestrator context clean and enables parallel coordination.",
  "Set `run_in_background: true` on every Agent tool invocation unless you have an explicit reason not to.",
].join(" ");

process.stdout.write(JSON.stringify({ systemMessage: warning }));
