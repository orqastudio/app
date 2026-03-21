#!/usr/bin/env node
// Memory redirect: warns when writing feedback/project memory files.
// Process learnings belong in .orqa/process/lessons/, not auto-memory.
// Used by PreToolUse hook. Reads hook input from stdin. Non-blocking.

async function main() {
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

  const filePath = (hookInput.tool_input?.file_path || "").replace(/\\/g, "/");

  if (!filePath) {
    process.exit(0);
  }

  const isMemoryFeedback = filePath.includes("memory/feedback_");
  const isMemoryProject = filePath.includes("memory/project_");

  if (!isMemoryFeedback && !isMemoryProject) {
    process.exit(0);
  }

  const systemMessage =
    "This looks like a process learning. Consider creating a lesson artifact in .orqa/process/lessons/ instead — lessons go through the learning loop (promotion to rules at recurrence >= 2). Auto-memory is for user preferences (user type) and references (reference type) only.";

  process.stdout.write(JSON.stringify({ systemMessage }));
  process.exit(0);
}

main().catch(() => process.exit(0));
