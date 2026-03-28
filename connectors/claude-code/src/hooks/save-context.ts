// PreCompact hook — all matchers
//
// Thin adapter: delegates governance context composition to the daemon via
// POST /compact-context, then writes the result to .state/governance-context.md
// and returns the summary as a systemMessage.

import { writeFileSync, existsSync, mkdirSync } from "fs";
import { join } from "path";
import { readInput, callDaemon } from "./shared.js";
import { logTelemetry } from "./telemetry.js";

interface CompactContextResult {
  context_document: string;
  summary: string;
}

async function main(): Promise<void> {
  const startTime = Date.now();

  let hookInput;
  try {
    hookInput = await readInput();
  } catch {
    process.exit(0);
  }

  const projectDir = hookInput.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";
  const stateDir = join(projectDir, ".state");

  if (!existsSync(stateDir)) {
    mkdirSync(stateDir, { recursive: true });
  }

  const result = await callDaemon<CompactContextResult>("/compact-context", {
    project_path: projectDir,
  });

  writeFileSync(join(stateDir, "governance-context.md"), result.context_document);

  logTelemetry("save-context", "PreCompact", startTime, "saved", {}, projectDir);

  process.stdout.write(JSON.stringify({ systemMessage: result.summary }));
  process.exit(0);
}
main().catch(() => process.exit(0));
