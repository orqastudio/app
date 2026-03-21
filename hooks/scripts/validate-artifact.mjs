#!/usr/bin/env node
// PostToolUse hook: validates .orqa/ artifacts after Write/Edit operations.
//
// Delegates to the MCP server's graph_validate and graph_health tools for
// comprehensive schema-driven integrity checks. Falls back to minimal
// file-level checks (frontmatter presence, id field) when the MCP server
// is unavailable.
//
// Runs after Write/Edit completes on .orqa/ files. Non-blocking — reports
// validation issues as systemMessage warnings without denying the operation.

import { readFileSync, existsSync } from "fs";
import { join, relative } from "path";
import { spawnSync } from "node:child_process";
import { logTelemetry } from "./telemetry.mjs";

// ---------------------------------------------------------------------------
// MCP bridge helpers
// ---------------------------------------------------------------------------

/**
 * Call the MCP server with multiple tool calls in a single spawn.
 *
 * Sends: initialize + N tools/call messages (newline-delimited JSON-RPC).
 * Returns a Map<id, parsedResult> for each tool call that succeeds.
 *
 * @param {string} projectPath
 * @param {Array<{id: number, name: string, arguments: object}>} calls
 * @returns {Map<number, unknown> | null}
 */
function callMcp(projectPath, calls) {
  const initialize = JSON.stringify({
    jsonrpc: "2.0",
    id: 1,
    method: "initialize",
    params: {
      protocolVersion: "2024-11-05",
      capabilities: {},
      clientInfo: { name: "validate-artifact", version: "1.0.0" },
    },
  });

  const toolCalls = calls.map(({ id, name, arguments: args }) =>
    JSON.stringify({
      jsonrpc: "2.0",
      id,
      method: "tools/call",
      params: { name, arguments: args },
    })
  );

  const input = [initialize, ...toolCalls].join("\n") + "\n";

  let result;
  try {
    result = spawnSync("orqa", ["mcp", projectPath], {
      input,
      encoding: "utf-8",
      timeout: 10000,
      windowsHide: true,
    });
  } catch {
    return null;
  }

  if (result.error || result.status !== 0 || !result.stdout) {
    return null;
  }

  const responses = new Map();
  const lines = result.stdout.split("\n").filter((l) => l.trim());
  for (const line of lines) {
    let parsed;
    try {
      parsed = JSON.parse(line);
    } catch {
      continue;
    }
    if (parsed.id === 1) continue; // skip initialize response
    if (parsed.error) continue;
    const textContent = parsed.result?.content?.[0]?.text;
    if (!textContent) continue;
    try {
      responses.set(parsed.id, JSON.parse(textContent));
    } catch {
      // text content is not JSON — skip
    }
  }

  return responses;
}

// ---------------------------------------------------------------------------
// Fallback: minimal file-level checks when MCP is unavailable
// ---------------------------------------------------------------------------

/**
 * Run minimal file-level checks when MCP server is unavailable.
 * Only checks: frontmatter presence and id field existence.
 *
 * @param {string} filePath  Absolute path to the artifact file
 * @returns {{ errors: string[], warnings: string[] }}
 */
function minimalFallbackChecks(filePath) {
  const errors = [];
  const warnings = [];

  if (!existsSync(filePath)) {
    return { errors, warnings };
  }

  let content;
  try {
    content = readFileSync(filePath, "utf-8");
  } catch {
    return { errors, warnings };
  }

  if (!content.startsWith("---\n")) {
    errors.push("Missing YAML frontmatter (file must start with ---)");
    return { errors, warnings };
  }

  if (!content.slice(4).includes("\n---")) {
    errors.push("Unclosed YAML frontmatter (missing closing ---)");
    return { errors, warnings };
  }

  const endIdx = content.indexOf("\n---", 4);
  const frontmatter = content.slice(4, endIdx);
  const hasId = frontmatter.split("\n").some((l) => l.startsWith("id:"));

  if (!hasId) {
    errors.push("Missing required frontmatter field: id");
  }

  warnings.push(
    "MCP server unavailable — only minimal checks were run. " +
      "Run `orqa validate` for full integrity check."
  );

  return { errors, warnings };
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/**
 * Check if a file path is within the .orqa/ directory.
 *
 * @param {string} filePath
 * @param {string} projectDir
 * @returns {boolean}
 */
function isOrqaArtifact(filePath, projectDir) {
  const rel = relative(projectDir, filePath).replace(/\\/g, "/");
  return rel.startsWith(".orqa/") && filePath.endsWith(".md");
}

/**
 * Format a graph_health response into a one-line summary string.
 *
 * @param {object} health  Parsed graph_health JSON (GraphHealth struct fields)
 * @returns {string}
 */
function formatHealthSummary(health) {
  if (!health || typeof health !== "object") return "";

  const traceability =
    typeof health.pillar_traceability === "number"
      ? `${health.pillar_traceability.toFixed(1)}% traceability`
      : null;
  const orphans =
    typeof health.orphan_count === "number"
      ? `${health.orphan_count} orphan${health.orphan_count !== 1 ? "s" : ""}`
      : null;
  const components =
    typeof health.component_count === "number"
      ? `${health.component_count} cluster${health.component_count !== 1 ? "s" : ""}`
      : null;

  const parts = [traceability, orphans, components].filter(Boolean);
  return parts.length > 0 ? `Graph health: ${parts.join(", ")}` : "";
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main() {
  const startTime = Date.now();

  let input = "";
  for await (const chunk of process.stdin) {
    input += chunk;
  }

  let hookInput;
  try {
    hookInput = JSON.parse(input);
  } catch {
    process.exit(0);
  }

  const toolName = hookInput.tool_name || "";
  const toolInput = hookInput.tool_input || {};
  const projectDir = hookInput.cwd || process.env.CLAUDE_PROJECT_DIR || ".";

  // Only validate Write and Edit on .orqa/ files.
  if (!["Write", "Edit"].includes(toolName)) {
    process.exit(0);
  }

  const filePath = toolInput.file_path || "";
  if (!isOrqaArtifact(filePath, projectDir)) {
    process.exit(0);
  }

  const relPath = relative(projectDir, filePath).replace(/\\/g, "/");

  // ---------------------------------------------------------------------------
  // Primary path: delegate to MCP graph_validate + graph_health
  // ---------------------------------------------------------------------------

  const mcpResponses = callMcp(projectDir, [
    { id: 2, name: "graph_validate", arguments: {} },
    { id: 3, name: "graph_health", arguments: {} },
  ]);

  if (!mcpResponses) {
    // Fallback: MCP unavailable — run minimal checks.
    const { errors, warnings } = minimalFallbackChecks(filePath);

    const totalIssues = errors.length + warnings.length;

    logTelemetry(
      "validate-artifact",
      "PostToolUse",
      startTime,
      "fallback",
      {
        file: relPath,
        mcp_available: false,
        errors_found: errors.length,
        warnings_issued: warnings.length,
      },
      projectDir
    );

    if (totalIssues === 0) {
      process.exit(0);
    }

    const lines = [`ARTIFACT VALIDATION (fallback) — ${relPath}:`];
    if (errors.length > 0) {
      lines.push("  Errors (must fix before committing):");
      for (const e of errors) lines.push(`    - ${e}`);
    }
    if (warnings.length > 0) {
      lines.push("  Warnings:");
      for (const w of warnings) lines.push(`    - ${w}`);
    }
    lines.push("");
    lines.push("Fix errors before committing. Run `orqa validate` for full integrity check.");

    process.stdout.write(JSON.stringify({ systemMessage: lines.join("\n") }));
    process.exit(0);
  }

  // Parse graph_validate results — filter to findings for this artifact.
  const validateResult = mcpResponses.get(2);
  const healthResult = mcpResponses.get(3);

  /** @type {Array<{severity: string, category: string, message: string, artifact_id: string, auto_fixable: boolean}>} */
  const allChecks = Array.isArray(validateResult) ? validateResult : [];

  // Determine the artifact ID from the file path (look it up in validate results
  // or extract directly from the file frontmatter).
  let artifactId = null;
  try {
    const content = readFileSync(filePath, "utf-8");
    if (content.startsWith("---\n")) {
      const endIdx = content.indexOf("\n---", 4);
      if (endIdx !== -1) {
        const fmLines = content.slice(4, endIdx).split("\n");
        const idLine = fmLines.find((l) => l.startsWith("id:"));
        if (idLine) {
          artifactId = idLine.replace(/^id:\s*/, "").replace(/^"|"$/g, "").trim();
        }
      }
    }
  } catch {
    // If we can't read the file, keep artifactId null and show all findings.
  }

  // Filter to findings relevant to this artifact.
  const relevantChecks = artifactId
    ? allChecks.filter((c) => c.artifact_id === artifactId)
    : allChecks;

  const errors = relevantChecks.filter((c) => c.severity === "Error").map((c) => c.message);
  const warnings = relevantChecks.filter((c) => c.severity !== "Error").map((c) => c.message);

  const healthSummary = formatHealthSummary(healthResult);
  const totalIssues = errors.length + warnings.length;

  logTelemetry(
    "validate-artifact",
    "PostToolUse",
    startTime,
    totalIssues === 0 ? "valid" : "invalid",
    {
      file: relPath,
      artifact_id: artifactId,
      mcp_available: true,
      errors_found: errors.length,
      warnings_issued: warnings.length,
      health_summary: healthSummary,
    },
    projectDir
  );

  if (totalIssues === 0 && !healthSummary) {
    process.exit(0);
  }

  const lines = [];

  if (totalIssues > 0) {
    lines.push(`ARTIFACT VALIDATION — ${relPath}:`);
    if (errors.length > 0) {
      lines.push("  Errors (must fix before committing):");
      for (const e of errors) lines.push(`    - ${e}`);
    }
    if (warnings.length > 0) {
      lines.push("  Warnings:");
      for (const w of warnings) lines.push(`    - ${w}`);
    }
    lines.push("");
    lines.push("Fix errors before committing. Run `orqa validate --fix` for auto-remediation.");
  }

  if (healthSummary) {
    lines.push(healthSummary);
  }

  if (lines.length > 0) {
    process.stdout.write(JSON.stringify({ systemMessage: lines.join("\n") }));
  }

  process.exit(0);
}

main().catch(() => process.exit(0));
