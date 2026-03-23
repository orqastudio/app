// PreToolUse hook — Write | Edit
//
// Warns when writing feedback/project memory files — process learnings belong
// in .orqa/process/lessons/, not auto-memory. No daemon call needed.
import { readInput, outputAllow } from "./shared.js";
async function main() {
    let hookInput;
    try {
        hookInput = await readInput();
    }
    catch {
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
    const systemMessage = "This looks like a process learning. Consider creating a lesson artifact in .orqa/process/lessons/ instead — lessons go through the learning loop (promotion to rules at recurrence >= 2). Auto-memory is for user preferences (user type) and references (reference type) only.";
    process.stdout.write(JSON.stringify({ systemMessage }));
    process.exit(0);
}
main().catch(() => process.exit(0));
//# sourceMappingURL=memory-redirect.js.map