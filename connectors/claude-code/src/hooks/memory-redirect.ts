// PreToolUse hook — Write | Edit
//
// Warns when writing feedback/project memory files — process learnings belong
// in .orqa/process/lessons/, not auto-memory. No daemon call needed.

import { readInput, outputAllow } from "./shared.js";

async function main(): Promise<void> {
  let hookInput;
  try {
    hookInput = await readInput();
  } catch {
    process.exit(0);
  }

  const filePath = (hookInput.tool_input?.file_path ?? "").replace(/\\/g, "/");
  if (!filePath) {
    outputAllow();
  }

  const isMemoryFeedback = filePath.includes("memory/feedback_");
  const isMemoryProject = filePath.includes("memory/project_");
  if (!isMemoryFeedback && !isMemoryProject) {
    outputAllow();
  }

  const systemMessage =
    "STOP: This is a process learning. You MUST create a lesson artifact in .orqa/process/lessons/ instead of writing to auto-memory. Lessons go through the learning loop (promotion to rules at recurrence >= 2). Do NOT store process learnings in auto-memory — auto-memory is ONLY for user preferences (user type) and references (reference type).";

  process.stdout.write(JSON.stringify({ systemMessage }));
  process.exit(0);
}

main().catch(() => process.exit(0));
