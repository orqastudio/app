// PreToolUse hook — Write | Edit to .orqa/ files
//
// Thin adapter: calls POST /query to get schema requirements for the artifact
// type being written, then warns if required relationships are missing.
// Warn-only — never blocks. Zero enforcement logic in connector.
import { existsSync } from "fs";
import { relative } from "path";
import { readInput, callDaemon, outputAllow } from "./shared.js";
import { logTelemetry } from "./telemetry.js";
import { parse as parseYaml } from "yaml";
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
    if (!["Write", "Edit"].includes(toolName)) {
        outputAllow();
    }
    const filePath = toolInput.file_path ?? "";
    if (!filePath || !isOrqaArtifact(filePath, projectDir)) {
        outputAllow();
    }
    // Only run on NEW artifacts — skip if the file already exists.
    if (existsSync(filePath)) {
        outputAllow();
    }
    const content = toolName === "Write"
        ? (toolInput.content ?? "")
        : (toolInput.new_string ?? toolInput.content ?? "");
    if (!content) {
        outputAllow();
    }
    // Extract artifact type from frontmatter for the query.
    const fm = parseFrontmatter(content);
    if (!fm) {
        outputAllow();
    }
    const frontmatterType = fm["type"] ? String(fm["type"]) : undefined;
    const relPath = relative(projectDir, filePath).replace(/\\/g, "/");
    let schema;
    try {
        schema = await callDaemon("/query", {
            type: frontmatterType ?? inferTypeFromPath(relPath),
            status: "active",
        });
    }
    catch {
        logTelemetry("artifact-enforcement", "PreToolUse", startTime, "unavailable", { file: relPath }, projectDir);
        outputAllow();
    }
    const requirements = schema.requirements ?? [];
    if (requirements.length === 0) {
        outputAllow();
    }
    // Check which required relationships are present in the frontmatter.
    const presentKeys = extractRelationshipKeys(fm);
    const missing = requirements.filter((r) => !presentKeys.has(r.key));
    logTelemetry("artifact-enforcement", "PreToolUse", startTime, missing.length === 0 ? "ok" : "warned", { file: relPath, missing_count: missing.length, missing: missing.map((r) => r.key) }, projectDir);
    if (missing.length === 0) {
        outputAllow();
    }
    const artifactType = frontmatterType ?? inferTypeFromPath(relPath) ?? "artifact";
    const lines = [
        `ARTIFACT RELATIONSHIP WARNING — ${relPath} (type: ${artifactType}):`,
        `New ${artifactType} artifact is missing required relationships:`,
        ...missing.map((r) => `  - ${r.label}`),
        "",
        "Add these relationships before committing. Run `orqa enforce` to check the full graph after writing.",
    ];
    process.stdout.write(JSON.stringify({ systemMessage: lines.join("\n") }));
    process.exit(0);
}
// ---------------------------------------------------------------------------
// Local helpers (connector-specific path/content analysis — no daemon needed)
// ---------------------------------------------------------------------------
function isOrqaArtifact(filePath, projectDir) {
    if (!filePath.endsWith(".md"))
        return false;
    const rel = relative(projectDir, filePath).replace(/\\/g, "/");
    return rel.startsWith(".orqa/");
}
function parseFrontmatter(content) {
    const fmEnd = content.indexOf("\n---", 4);
    if (!content.startsWith("---\n") || fmEnd === -1)
        return null;
    try {
        return parseYaml(content.slice(4, fmEnd));
    }
    catch {
        return null;
    }
}
function extractRelationshipKeys(fm) {
    const keys = new Set();
    for (const val of Object.values(fm)) {
        if (!Array.isArray(val))
            continue;
        for (const item of val) {
            if (item && typeof item === "object" && "type" in item) {
                keys.add(String(item["type"]));
            }
        }
    }
    return keys;
}
function inferTypeFromPath(relPath) {
    const segments = relPath.split("/");
    // e.g. .orqa/delivery/epics/EPIC-xxx.md → "epics" → "epic"
    const dir = segments[segments.length - 2];
    if (!dir)
        return null;
    return dir.endsWith("s") ? dir.slice(0, -1) : dir;
}
main().catch(() => process.exit(0));
//# sourceMappingURL=artifact-enforcement.js.map