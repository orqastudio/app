// UserPromptSubmit hook — thin adapter
//
// Reads the user prompt, calls the daemon /prompt endpoint for pipeline-composed
// system prompt generation, then layers on connector-specific UX (context line)
// before writing the systemMessage to stdout.

import { existsSync, readFileSync } from "fs";
import { join } from "path";
import { readInput, outputAllow, callDaemon } from "./shared.js";
import { logTelemetry } from "./telemetry.js";

/** Shape returned by the daemon POST /prompt endpoint. */
interface PromptResult {
  prompt: string;
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
  const agentType = hookInput.agent_type ?? "orchestrator";

  if (!userMessage) {
    outputAllow();
  }

  const result = await callDaemon<PromptResult>("/prompt", {
    message: userMessage,
    role: agentType,
    project_path: projectDir,
  });

  const contextLine = getContextLine(projectDir);
  const systemMessage = [result.prompt, contextLine].filter(Boolean).join("\n\n");

  logTelemetry("prompt-injector", "UserPromptSubmit", startTime, "injected", {
    agent_type: agentType,
    query: userMessage.slice(0, 100),
    action: "allow",
  }, projectDir);

  process.stdout.write(JSON.stringify({ systemMessage }));
  process.exit(0);
}

// ---------------------------------------------------------------------------
// Connector-specific helpers — format/UX concerns that stay in the connector
// ---------------------------------------------------------------------------

/**
 * Read project.json and return a concise context line.
 * @param projectDir - Absolute path to the project directory containing .orqa/project.json.
 * @returns A single-line string summarising project name and dogfood status.
 */
function getContextLine(projectDir: string): string {
  const p = join(projectDir, ".orqa", "project.json");
  if (!existsSync(p)) {
    return "Project: unknown. Run `orqa plugin list` to check installed plugins if needed.";
  }
  try {
    const s = JSON.parse(readFileSync(p, "utf-8")) as Record<string, unknown>;
    const name = String(s["name"] ?? "unknown");
    const dogfood = s["dogfood"] ? "active — you are editing the app from the CLI" : "inactive";
    return `Project: ${name}. Dogfood: ${dogfood}. Run \`orqa plugin list\` to check installed plugins if needed.`;
  } catch {
    return "Project: unknown. Run `orqa plugin list` to check installed plugins if needed.";
  }
}

main().catch(() => process.exit(0));
