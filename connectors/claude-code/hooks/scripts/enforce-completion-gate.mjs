/**
 * PreToolUse hook: enforce completion gate before new work.
 *
 * Fires on TeamCreate. Checks .state/session-state.md for a
 * "### Completion Gate" section. If the section exists and has
 * unchecked items ([ ]), warns the orchestrator to resolve them
 * before starting new work.
 *
 * Non-blocking (warn only) — the orchestrator sees the message
 * and should act on it before proceeding.
 *
 * See RULE-5d2d39b7 for the full completion gate protocol.
 *
 * stdin  — JSON: { tool_name, tool_input }
 * stdout — JSON: { systemMessage? }
 */

const allow = () => {
  process.stdout.write(JSON.stringify({}));
};

function readStdin() {
  return new Promise((resolve, reject) => {
    const chunks = [];
    process.stdin.setEncoding("utf-8");
    process.stdin.on("data", (chunk) => chunks.push(chunk));
    process.stdin.on("end", () => resolve(chunks.join("")));
    process.stdin.on("error", reject);
  });
}

async function main() {
  let input;
  try {
    const raw = await readStdin();
    input = JSON.parse(raw);
  } catch {
    return allow();
  }

  // Only fire on TeamCreate
  if (input.tool_name !== "TeamCreate") {
    return allow();
  }

  // Check session state for completion gate
  const fs = await import("node:fs");
  const path = await import("node:path");
  const projectDir = input.cwd || process.env.CLAUDE_PROJECT_DIR || ".";
  const sessionStatePath = path.join(projectDir, ".state", "session-state.md");

  if (!fs.existsSync(sessionStatePath)) {
    // No session state — can't check, allow
    return allow();
  }

  const content = fs.readFileSync(sessionStatePath, "utf-8");

  // Look for unchecked items in Outstanding or Completion Gate sections
  const outstandingMatch = content.match(/### (?:Outstanding|Completion Gate|Follow-up)[\s\S]*?(?=###|$)/i);
  if (!outstandingMatch) {
    return allow();
  }

  const section = outstandingMatch[0];
  const unchecked = section.match(/- \[ \] .+/g);

  if (!unchecked || unchecked.length === 0) {
    return allow();
  }

  // Found unresolved items — warn
  const items = unchecked.map(line => line.replace("- [ ] ", "  • ")).join("\n");
  const message = [
    "COMPLETION GATE (RULE-5d2d39b7): Unresolved follow-up items found before new work.",
    "",
    `${unchecked.length} outstanding item(s):`,
    items,
    "",
    "Resolve these before creating a new team. Follow-up items are first-class work,",
    "not afterthoughts. Either complete them now or get explicit user approval to defer.",
  ].join("\n");

  process.stdout.write(JSON.stringify({ systemMessage: message }));
}

main().catch(() => allow());
