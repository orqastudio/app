// UserPromptSubmit hook — departure detection
//
// Detects when the user signals they are leaving (going to bed, stepping away,
// brb, etc.) and injects a systemMessage reminding the orchestrator to:
//   1. Write session state to tmp/session-state.md with progress and next priorities
//   2. Continue working on outstanding tasks — do NOT stop
//   3. Compile any requested findings/summaries before the user returns

import { readInput, outputAllow } from "./shared.js";
import { logTelemetry } from "./telemetry.js";

/**
 * Departure keyword patterns. Each regex is tested against the lowercased
 * user message. Patterns use word boundaries where possible to avoid false
 * positives (e.g. "leaving" in "leaving the function as-is" is a risk, but
 * the full phrase set makes accidental matches unlikely in practice).
 */
const DEPARTURE_PATTERNS: RegExp[] = [
  /\bgoing to bed\b/,
  /\bgoing afk\b/,
  /\bgoing offline\b/,
  /\bstepping away\b/,
  /\btaking a break\b/,
  /\bheading out\b/,
  /\bsigning off\b/,
  /\blogging off\b/,
  /\bbe right back\b/,
  /\bbrb\b/,
  /\bgotta go\b/,
  /\bgotta run\b/,
  /\bgoing to sleep\b/,
  /\bcall it a night\b/,
  /\bcall it a day\b/,
  /\bdone for (the|today|tonight|now)\b/,
  /\bback later\b/,
  /\bback in a (bit|while|few)\b/,
  /\bgoing to (lunch|dinner|eat)\b/,
];

function isDepartureMessage(message: string): boolean {
  const lower = message.toLowerCase();
  return DEPARTURE_PATTERNS.some((pattern) => pattern.test(lower));
}

async function main(): Promise<void> {
  const startTime = Date.now();

  let hookInput;
  try {
    hookInput = await readInput();
  } catch {
    process.exit(0);
  }

  const userMessage = hookInput.user_message ?? hookInput.prompt ?? "";
  const projectDir = hookInput.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";

  if (!userMessage || !isDepartureMessage(userMessage)) {
    logTelemetry(
      "departure-detection", "UserPromptSubmit", startTime, "no-match",
      { query: userMessage.slice(0, 100) },
      projectDir,
    );
    outputAllow();
  }

  // Departure detected — inject session state writing reminder.
  logTelemetry(
    "departure-detection", "UserPromptSubmit", startTime, "departure-detected",
    { query: userMessage.slice(0, 100) },
    projectDir,
  );

  const systemMessage = [
    "DEPARTURE DETECTED — The user is leaving.",
    "",
    "You MUST do the following IMMEDIATELY:",
    "",
    "1. Write session state to tmp/session-state.md NOW with:",
    "   - Current scope (active epic/task IDs)",
    "   - Step checklist with completion status",
    "   - Decisions made this session",
    "   - Next Session Priorities (ordered list of what to do next)",
    "",
    "2. Continue working on ALL outstanding tasks. Do NOT stop.",
    "   The user is stepping away, not asking you to pause.",
    "",
    "3. Compile any requested findings or summaries so they are",
    "   ready when the user returns.",
    "",
    "4. Commit and push all work before finishing.",
    "",
    "Reference: RULE-e3f5a7b9, RULE-4f7e2a91, RULE-e352fd0a",
  ].join("\n");

  process.stdout.write(JSON.stringify({ systemMessage }));
  process.exit(0);
}

main().catch(() => process.exit(0));
