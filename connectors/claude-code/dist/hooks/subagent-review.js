// SubagentStop hook — all matchers
//
// Checks for stub patterns in modified files and calls POST /parse for
// artifact integrity on any .orqa/ files that were modified.
// The stub pattern check is a connector-side concern (git diff + file read).
// Artifact integrity check delegates to the daemon.
import { readFileSync, existsSync } from "fs";
import { join } from "path";
import { execSync } from "child_process";
import { parse as parseYaml } from "yaml";
import { readInput, callDaemon, outputAllow } from "./shared.js";
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
    const projectDir = hookInput.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";
    const agentType = hookInput.agent_type ?? "unknown";
    const modifiedFiles = getModifiedFiles(projectDir);
    if (modifiedFiles.length === 0) {
        logTelemetry("subagent-review", "SubagentStop", startTime, "clean", { agent_type: agentType, files_checked: 0, todos_found: 0, artifact_issues: 0 }, projectDir);
        outputAllow();
    }
    const warnings = [];
    // Check for stub markers (connector-side — just grep modified files).
    const stubIssues = checkForStubs(projectDir, modifiedFiles);
    if (stubIssues.length > 0) {
        warnings.push("STUB/TODO markers found in modified files:");
        warnings.push(...stubIssues.map((i) => `  - ${i}`));
    }
    // Validate modified .orqa/ artifacts via daemon.
    const artifactFiles = modifiedFiles.filter((f) => f.startsWith(".orqa/") && f.endsWith(".md"));
    const artifactIssues = [];
    for (const file of artifactFiles) {
        const fullPath = join(projectDir, file);
        if (!existsSync(fullPath))
            continue;
        try {
            const parsed = await callDaemon("/parse", { file: fullPath });
            const errors = (parsed.findings ?? []).filter((f) => f.severity === "error" || f.severity === "Error");
            for (const e of errors)
                artifactIssues.push(`${file}: ${e.message}`);
        }
        catch {
            // Daemon unavailable — fall back to local frontmatter check.
            const localIssues = checkArtifactFrontmatter(projectDir, file);
            artifactIssues.push(...localIssues);
        }
    }
    if (artifactIssues.length > 0) {
        warnings.push("Artifact integrity issues:");
        warnings.push(...artifactIssues.map((i) => `  - ${i}`));
    }
    if (warnings.length === 0) {
        logTelemetry("subagent-review", "SubagentStop", startTime, "clean", {
            agent_type: agentType, files_checked: modifiedFiles.length, todos_found: 0, artifact_issues: 0,
        }, projectDir);
        outputAllow();
    }
    logTelemetry("subagent-review", "SubagentStop", startTime, "warned", {
        agent_type: agentType,
        files_checked: modifiedFiles.length,
        todos_found: stubIssues.length,
        artifact_issues: artifactIssues.length,
    }, projectDir);
    const message = [
        `SUBAGENT REVIEW — ${agentType} completed with warnings:`,
        "",
        ...warnings,
        "",
        "Address these before committing.",
    ].join("\n");
    process.stdout.write(JSON.stringify({ systemMessage: message }));
    process.exit(0);
}
// ---------------------------------------------------------------------------
// Local helpers
// ---------------------------------------------------------------------------
function getModifiedFiles(projectDir) {
    try {
        const out = execSync("git diff --name-only HEAD", {
            cwd: projectDir, encoding: "utf-8", stdio: ["pipe", "pipe", "pipe"],
        });
        return out.trim().split("\n").filter(Boolean);
    }
    catch {
        try {
            const out = execSync("git diff --name-only", {
                cwd: projectDir, encoding: "utf-8", stdio: ["pipe", "pipe", "pipe"],
            });
            return out.trim().split("\n").filter(Boolean);
        }
        catch {
            return [];
        }
    }
}
const STUB_PATTERN = /\b(TODO|FIXME|STUB|HACK|XXX|PLACEHOLDER)\b/i;
function checkForStubs(projectDir, files) {
    const issues = [];
    for (const file of files) {
        if (!file.endsWith(".md") && !file.endsWith(".ts") && !file.endsWith(".rs") && !file.endsWith(".svelte"))
            continue;
        const fullPath = join(projectDir, file);
        if (!existsSync(fullPath))
            continue;
        try {
            const lines = readFileSync(fullPath, "utf-8").split("\n");
            for (let i = 0; i < lines.length; i++) {
                const m = lines[i].match(STUB_PATTERN);
                if (m)
                    issues.push(`${file}:${i + 1} — contains ${m[0]} marker`);
            }
        }
        catch { /* skip */ }
    }
    return issues;
}
/** Fallback when daemon is unavailable — check frontmatter locally. */
function checkArtifactFrontmatter(projectDir, file) {
    const fullPath = join(projectDir, file);
    if (!existsSync(fullPath))
        return [];
    const content = readFileSync(fullPath, "utf-8");
    if (!content.startsWith("---\n"))
        return [`${file} — missing YAML frontmatter`];
    const fmEnd = content.indexOf("\n---", 4);
    if (fmEnd === -1)
        return [`${file} — malformed YAML frontmatter`];
    try {
        const fm = parseYaml(content.slice(4, fmEnd));
        if (!fm || !("id" in fm))
            return [`${file} — frontmatter missing required 'id' field`];
        return [];
    }
    catch {
        return [`${file} — malformed YAML frontmatter`];
    }
}
main().catch(() => process.exit(0));
//# sourceMappingURL=subagent-review.js.map