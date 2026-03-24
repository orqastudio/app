// PostToolUse hook — Write | Edit to .orqa/ files
//
// Thin adapter: calls POST /parse on the written file, then outputs any
// validation errors as a non-blocking systemMessage.
// Zero validation logic — all schema checks are in the Rust daemon.

import { relative } from "path";
import { readInput, callDaemon, outputAllow } from "./shared.js";
import { logTelemetry } from "./telemetry.js";

interface ValidationResult {
  valid: boolean;
  errors: string[];
}

interface ParsedArtifact {
  id: string;
  type: string;
  status?: string;
  title: string;
  frontmatter: Record<string, unknown>;
  content: string;
  validation: ValidationResult;
}

async function main(): Promise<void> {
  const startTime = Date.now();

  let hookInput;
  try {
    hookInput = await readInput();
  } catch {
    process.exit(0);
  }

  const toolName = hookInput.tool_name ?? "";
  const toolInput = hookInput.tool_input ?? {};
  const projectDir = hookInput.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";

  if (!["Write", "Edit"].includes(toolName)) {
    outputAllow();
  }

  const filePath = toolInput.file_path ?? "";
  if (!filePath || !isOrqaArtifact(filePath, projectDir)) {
    outputAllow();
  }

  const relPath = relative(projectDir, filePath).replace(/\\/g, "/");

  let parsed: ParsedArtifact;
  try {
    parsed = await callDaemon<ParsedArtifact>("/parse", { file: filePath });
  } catch {
    logTelemetry("validate-artifact", "PostToolUse", startTime, "unavailable", { file: relPath }, projectDir);
    outputAllow();
  }

  const validation = parsed.validation;
  const errors = validation.errors ?? [];

  logTelemetry("validate-artifact", "PostToolUse", startTime,
    validation.valid ? "valid" : "invalid",
    { file: relPath, errors_found: errors.length },
    projectDir,
  );

  if (validation.valid && errors.length === 0) {
    outputAllow();
  }

  const lines = [`ARTIFACT VALIDATION — ${relPath}:`];
  lines.push("  Errors (must fix before committing):");
  for (const e of errors) lines.push(`    - ${e}`);
  lines.push("");
  lines.push("You MUST fix all errors before committing. Do NOT commit with validation errors. Run `orqa enforce --fix` for auto-remediation.");

  process.stdout.write(JSON.stringify({ systemMessage: lines.join("\n") }));
  process.exit(0);
}

/** Quick check: is this file a governance artifact under .orqa/? */
function isOrqaArtifact(filePath: string, projectDir: string): boolean {
  if (!filePath.endsWith(".md")) return false;
  const rel = relative(projectDir, filePath).replace(/\\/g, "/");
  return rel.startsWith(".orqa/");
}

main().catch(() => process.exit(0));
