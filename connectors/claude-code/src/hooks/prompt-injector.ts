// UserPromptSubmit hook — all matchers
//
// Thin adapter: classifies the user prompt, calls the prompt pipeline from
// @orqastudio/cli for knowledge/rule injection, then builds the systemMessage
// in Claude Code's format. Connector-specific UX (context line) is layered
// on top of the pipeline output.

import { existsSync, mkdirSync, readFileSync, writeFileSync } from "fs";
import { join } from "path";
import { readInput, outputAllow, readIpcPort, mcpSearchCall } from "./shared.js";

import { logTelemetry } from "./telemetry.js";
import { generatePrompt, type PromptResult } from "@orqastudio/cli";

/** Prompt types that drive workflow stage selection. */
type PromptType =
  | "implementation"
  | "planning"
  | "review"
  | "debugging"
  | "research"
  | "documentation"
  | "governance"
  | "general";

/**
 * Maps thinking-mode frontmatter values to PromptType.
 * Most map 1:1 by name; these are the exceptions.
 */
const THINKING_MODE_MAP: Record<string, PromptType> = {
  "learning-loop": "governance",
  "dogfood-implementation": "implementation",
};

/** Resolve a thinking-mode value to a PromptType. */
function resolveThinkingMode(mode: string): PromptType | null {
  if (THINKING_MODE_MAP[mode]) return THINKING_MODE_MAP[mode];
  const candidates: PromptType[] = [
    "implementation", "planning", "review", "debugging",
    "research", "documentation", "governance", "general",
  ];
  return candidates.includes(mode as PromptType) ? (mode as PromptType) : null;
}

// ---------------------------------------------------------------------------
// Semantic search classifier — Tier 1 (primary, high-quality matching)
// ---------------------------------------------------------------------------

/**
 * Classify a prompt using ONNX semantic search against thinking-mode knowledge artifacts.
 * Returns a PromptType if a confident match is found, null otherwise.
 */
async function classifyWithSearch(message: string, projectDir: string): Promise<PromptType | null> {
  const port = readIpcPort();
  if (port === null) return null;

  // Build a search query from the user message — prefix with context for better matching
  const truncated = message.slice(0, 200);
  const query = `thinking mode classification for user prompt: ${truncated}`;

  const results = await mcpSearchCall(port, projectDir, query, 5);
  if (results.length === 0) return null;

  // Find the best-matching thinking-mode knowledge artifact
  for (const result of results) {
    // Must be a knowledge artifact in the right directory
    if (!result.file.includes("process/knowledge/KNOW-")) continue;

    // Extract thinking-mode value from the content
    const modeMatch = /thinking-mode:\s*(\S+)/.exec(result.content);
    if (!modeMatch) continue;

    const mode = modeMatch[1];
    const resolved = resolveThinkingMode(mode);
    if (resolved) return resolved;
  }

  return null;
}

// ---------------------------------------------------------------------------
// Map PromptType to workflow stage names used by the prompt pipeline
// ---------------------------------------------------------------------------

/** Map prompt classification to a workflow stage string for the pipeline. */
function promptTypeToStage(pt: PromptType): string {
  switch (pt) {
    case "implementation": return "implement";
    case "planning": return "plan";
    case "review": return "review";
    case "debugging": return "debug";
    case "research": return "research";
    case "documentation": return "document";
    case "governance": return "govern";
    case "general": return "general";
  }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

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

  // Three-tier prompt classification:
  //   1. ONNX semantic search against thinking-mode knowledge artifacts (primary)
  //   2. Keyword regex matching (fallback when search unavailable)
  //   3. "general" default (when nothing matches)
  let promptType: PromptType;
  let classificationMethod: "semantic" | "keyword";
  try {
    const semanticResult = await classifyWithSearch(userMessage, projectDir);
    if (semanticResult) {
      promptType = semanticResult;
      classificationMethod = "semantic";
    } else {
      promptType = classifyPrompt(userMessage);
      classificationMethod = "keyword";
    }
  } catch {
    promptType = classifyPrompt(userMessage);
    classificationMethod = "keyword";
  }

  // Run the prompt pipeline — plugin-composed knowledge/rule injection.
  const workflowStage = promptTypeToStage(promptType);
  let pipelineResult: PromptResult;
  try {
    pipelineResult = generatePrompt({
      role: agentType || "orchestrator",
      workflowStage,
      projectPath: projectDir,
    });
  } catch {
    // Pipeline unavailable — produce a minimal fallback prompt
    pipelineResult = {
      prompt: `<role>${agentType || "orchestrator"}</role>`,
      totalTokens: 0,
      budget: 2500,
      includedSections: [],
      trimmedSections: [],
      errors: ["Prompt pipeline failed — using minimal fallback"],
    };
  }

  // Context line (connector-specific — stays here).
  const contextLine = getContextLine(projectDir);

  // Build the full preamble document (written to file for reference).
  const preambleDoc = [
    "# Orchestrator Preamble",
    "",
    "## Prompt Classification",
    "",
    `Classified as: **${promptType}** (via ${classificationMethod})`,
    "",
    "## Project Context",
    "",
    contextLine,
    "",
    "## Pipeline Output",
    "",
    `Sections included: ${pipelineResult.includedSections.length}`,
    `Sections trimmed: ${pipelineResult.trimmedSections.length}`,
    `Tokens: ${pipelineResult.totalTokens} / ${pipelineResult.budget}`,
    "",
    pipelineResult.prompt,
    "",
  ].join("\n");

  // Write full preamble to .state/orchestrator-preamble.md for reference.
  const stateDir = join(projectDir, ".state");
  mkdirSync(stateDir, { recursive: true });
  writeFileSync(join(stateDir, "orchestrator-preamble.md"), preambleDoc, "utf-8");

  logTelemetry("prompt-injector", "UserPromptSubmit", startTime, "injected", {
    agent_type: agentType,
    prompt_type: promptType,
    classification_method: classificationMethod,
    pipeline_sections: pipelineResult.includedSections.length,
    pipeline_trimmed: pipelineResult.trimmedSections.length,
    pipeline_tokens: pipelineResult.totalTokens,
    pipeline_budget: pipelineResult.budget,
    pipeline_errors: pipelineResult.errors.length,
    query: userMessage.slice(0, 100),
    action: "allow",
  }, projectDir);

  // Build systemMessage — pipeline prompt + connector-specific UX.
  const parts: string[] = [];

  // Pipeline-generated prompt (role definition, safety rules, knowledge, etc.)
  if (pipelineResult.prompt) {
    parts.push(pipelineResult.prompt);
  }

  // Context line
  parts.push(contextLine);

  // Reference the full preamble file for additional context.
  parts.push("Full session context: .state/orchestrator-preamble.md");

  // Pipeline errors (visible to agent for debugging)
  if (pipelineResult.errors.length > 0) {
    parts.push(`[Pipeline warnings: ${pipelineResult.errors.join("; ")}]`);
  }

  const systemMessage = parts.join("\n\n");
  process.stdout.write(JSON.stringify({ systemMessage }));
  process.exit(0);
}

// ---------------------------------------------------------------------------
// Prompt classification — Tier 2 keyword fallback
// ---------------------------------------------------------------------------

/**
 * Classify a user prompt into a type using keyword matching.
 * Tier 2 fallback — used when ONNX semantic search is unavailable.
 */
function classifyPrompt(message: string): PromptType {
  const lower = message.toLowerCase();

  // Implementation signals
  if (
    /\b(implement|build|create|add|write code|fix bug|refactor|migrate|wire up|hook up)\b/.test(lower)
  ) {
    return "implementation";
  }

  // Debugging signals
  if (/\b(debug|investigate|why does|broken|error|crash|failing|not working|trace)\b/.test(lower)) {
    return "debugging";
  }

  // Review signals
  if (/\b(review|audit|check|verify|validate|assess|compliance)\b/.test(lower)) {
    return "review";
  }

  // Planning signals
  if (/\b(plan|design|scope|epic|roadmap|milestone|break down|approach)\b/.test(lower)) {
    return "planning";
  }

  // Documentation signals
  if (/\b(document|docs|write up|describe|explain|specification)\b/.test(lower)) {
    return "documentation";
  }

  // Research signals
  if (/\b(research|explore|investigate options|compare|evaluate|what are the)\b/.test(lower)) {
    return "research";
  }

  // Governance signals
  if (
    /\b(rule|governance|enforce|lesson|artifact|pillar|promote|knowledge)\b/.test(lower)
  ) {
    return "governance";
  }

  return "general";
}

// ---------------------------------------------------------------------------
// Connector-specific helpers — format/UX concerns that stay in the connector
// ---------------------------------------------------------------------------

/** Read project.json and return a concise context line. */
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
