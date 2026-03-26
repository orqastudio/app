// PostToolUse hook — Write | Edit to .orqa/ files
//
// Thin adapter: calls POST /parse to get the artifact type, then injects
// impact context if it's a high-influence type or has many downstream refs.
// Zero logic — all type classification is in the Rust daemon.

import { existsSync } from "fs";
import { relative } from "path";
import { readInput, callDaemon, outputAllow, isOrqaArtifact } from "./shared.js";
import { logTelemetry } from "./telemetry.js";

interface ParsedArtifact {
  id?: string;
  artifact_type?: string;
  high_influence?: boolean;
  downstream_count?: number;
  downstream_summary?: string;
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
  if (!filePath || !isOrqaArtifact(filePath, projectDir) || !existsSync(filePath)) {
    outputAllow();
  }

  const relPath = relative(projectDir, filePath).replace(/\\/g, "/");

  let parsed: ParsedArtifact;
  try {
    parsed = await callDaemon<ParsedArtifact>("/parse", { file: filePath });
  } catch {
    logTelemetry("impact-check", "PostToolUse", startTime, "unavailable", { file: relPath }, projectDir);
    outputAllow();
  }

  const artifactId = parsed.id;
  const artifactType = parsed.artifact_type ?? "unknown";
  const highInfluence = parsed.high_influence ?? false;
  const downstreamCount = parsed.downstream_count ?? 0;
  const shouldInject = highInfluence || downstreamCount > 20;

  logTelemetry("impact-check", "PostToolUse", startTime, shouldInject ? "injected" : "skipped", {
    file: relPath,
    artifact_id: artifactId,
    artifact_type: artifactType,
    is_high_influence: highInfluence,
    downstream_count: downstreamCount,
  }, projectDir);

  if (!shouldInject) {
    outputAllow();
  }

  const lines = [`IMPACT CONTEXT — ${artifactId ?? relPath} (${artifactType}):`];
  if (highInfluence) {
    lines.push(`This is a ${artifactType} artifact. Changes affect the entire governance framework.`);
  }
  if (downstreamCount > 0) {
    const summary = parsed.downstream_summary ?? "(see graph_relationships for details)";
    lines.push(`It has ${downstreamCount} downstream relationship${downstreamCount !== 1 ? "s" : ""}: ${summary}.`);
    lines.push("You MUST review downstream artifacts for cascading effects before committing. Do NOT commit without verifying all downstream relationships are intact.");
  }

  if (lines.length > 1) {
    process.stdout.write(JSON.stringify({ systemMessage: lines.join("\n") }));
  }
  process.exit(0);
}

main().catch(() => process.exit(0));
