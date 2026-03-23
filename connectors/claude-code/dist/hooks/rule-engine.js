// PreToolUse hook — Write | Edit | Bash
//
// Thin adapter: reads stdin, builds HookContext, calls POST /hook on the daemon,
// then outputs the result in Claude Code's hook format.
// Zero enforcement logic — all evaluation is in the Rust daemon.
import { readInput, callDaemon, mapEvent, outputBlock, outputWarn, outputAllow } from "./shared.js";
import { logTelemetry } from "./telemetry.js";
async function main() {
    const startTime = Date.now();
    let hookInput;
    try {
        hookInput = await readInput();
    }
    catch {
        process.exit(0);
    }
    const toolName = hookInput.tool_name ?? "";
    const toolInput = hookInput.tool_input ?? {};
    const projectDir = hookInput.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";
    if (!["Write", "Edit", "Bash"].includes(toolName)) {
        outputAllow();
    }
    const context = {
        event: mapEvent("PreToolUse"),
        tool_name: toolName,
        tool_input: toolInput,
        file_path: toolInput.file_path,
        agent_type: hookInput.agent_type,
    };
    let result;
    try {
        result = await callDaemon("/hook", context);
    }
    catch {
        // Daemon unavailable and binary fallback also failed — allow silently
        logTelemetry("rule-engine", "PreToolUse", startTime, "unavailable", { tool: toolName }, projectDir);
        outputAllow();
    }
    logTelemetry("rule-engine", "PreToolUse", startTime, result.action, {
        tool: toolName,
        violations_found: result.violations.length,
        action: result.action,
    }, projectDir);
    if (result.action === "block") {
        outputBlock(result.messages);
    }
    else if (result.action === "warn" && result.messages.length > 0) {
        outputWarn(result.messages);
    }
    process.exit(0);
}
main().catch(() => process.exit(0));
//# sourceMappingURL=rule-engine.js.map