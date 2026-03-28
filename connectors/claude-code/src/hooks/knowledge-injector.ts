// PreToolUse hook — Agent matcher (knowledge-injector)
//
// Thin adapter: calls POST /knowledge on the daemon, which handles role
// detection, declared knowledge lookup (prompt registry), and ONNX semantic
// search. All business logic lives in the daemon.
//
// Non-blocking: injects context via outputWarn(), never denies.

import { readInput, outputAllow, outputWarn, callDaemon } from "./shared.js";
import { logTelemetry } from "./telemetry.js";

interface KnowledgeResult {
  entries: Array<{ id: string; title: string; path: string; source: string; score?: number }>;
}

async function main(): Promise<void> {
  const startTime = Date.now();
  const input = await readInput();

  if (input.tool_name !== "Agent") {
    outputAllow();
  }

  const prompt = (input.tool_input as { prompt?: string })?.prompt ?? "";
  if (!prompt) {
    outputAllow();
  }

  const projectDir = input.cwd ?? process.cwd();

  const result = await callDaemon<KnowledgeResult>("/knowledge", {
    agent_prompt: prompt,
    project_path: projectDir,
  });
  const entries = result?.entries ?? [];

  if (entries.length === 0) {
    logTelemetry("knowledge-injector", "PreToolUse:Agent", startTime, "no-results", {
      declared_count: 0,
      semantic_count: 0,
    });
    outputAllow();
  }

  const declared = entries.filter((e) => e.source === "declared");
  const semantic = entries.filter((e) => e.source === "semantic");
  const parts: string[] = [];

  if (declared.length > 0) {
    parts.push(
      `KNOWLEDGE INJECTION — agent has ${declared.length} declared knowledge artifact(s):\n`,
    );
    for (const k of declared) {
      parts.push(`- **${k.id}** (${k.title}): ${k.path}`);
    }
  }

  if (semantic.length > 0) {
    parts.push(
      `\nSEMANTIC KNOWLEDGE — ${semantic.length} additional artifact(s) found relevant to this task:\n`,
    );
    for (const k of semantic) {
      const pct = Math.round((k.score ?? 0) * 100);
      parts.push(`- **${k.id}** (${k.title}) — relevance: ${pct}%`);
    }
  }

  parts.push("\nRead these knowledge artifacts before starting work.");

  logTelemetry("knowledge-injector", "PreToolUse:Agent", startTime, "injected", {
    declared_count: declared.length,
    declared_ids: declared.map((e) => e.id),
    semantic_count: semantic.length,
    semantic_ids: semantic.map((e) => e.id),
  });

  outputWarn(parts);
}

main().catch(() => {
  outputAllow();
});
