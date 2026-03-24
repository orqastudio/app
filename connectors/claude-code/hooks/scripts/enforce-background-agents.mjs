/**
 * PreToolUse hook: enforce background agent team usage.
 *
 * Checks two conditions on every Agent tool call (warn only, never block):
 * 1. `run_in_background: true` must be set — keeps orchestrator available.
 * 2. `team_name` must be set — ensures TeamCreate was called first.
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

const warnings = [];

// Check 1: run_in_background must be true.
if (!tool_input?.run_in_background) {
  warnings.push(
    "RULE-00a8c660 / RULE-532100d9: Agent tool called without `run_in_background: true`.",
    "Background execution keeps the orchestrator context clean and enables parallel coordination.",
    "Set `run_in_background: true` on every Agent tool invocation unless you have an explicit reason not to.",
  );
}

// Check 2: team_name must be set (TeamCreate must have been called first).
if (!tool_input?.team_name) {
  warnings.push(
    "RULE-00a8c660 violation: Agent tool called without `team_name`.",
    "ALL delegated work MUST use TeamCreate first, then spawn agents with the `team_name` parameter.",
    "Even single-task delegations require a team — this keeps the orchestrator available for conversation.",
  );
}

if (warnings.length === 0) {
  // Both conditions met — no warning needed.
  allow();
  process.exit(0);
}

process.stdout.write(JSON.stringify({ systemMessage: warnings.join(" ") }));
