/**
 * On-demand knowledge retrieval — queries knowledge artifacts by tags, role,
 * or content for agents that need more context beyond what the prompt pipeline
 * pre-loads.
 *
 * Integration point: the MCP search tools (mcp__orqastudio__search_semantic)
 * are available at runtime. This module provides the disk-based fallback and
 * the preamble that tells agents how to access on-demand knowledge.
 */
import * as fs from "node:fs";
import * as path from "node:path";
import { parseFrontmatterFromContent } from "./frontmatter.js";
import { estimateTokens } from "./prompt-pipeline.js";
// ---------------------------------------------------------------------------
// On-Demand Preamble
// ---------------------------------------------------------------------------
/**
 * Generate the on-demand knowledge preamble text that instructs agents on
 * how to retrieve full knowledge text via semantic search.
 *
 * Included in generated prompts when on-demand knowledge entries exist.
 */
export function generateOnDemandPreamble(onDemandCount) {
    if (onDemandCount === 0)
        return "";
    return [
        "<on-demand-knowledge>",
        `There are ${onDemandCount} additional knowledge artifacts available on-demand.`,
        "To retrieve full content for a specific topic, use the semantic search tool:",
        "  mcp__orqastudio__search_semantic with your query",
        "Only retrieve knowledge when you need specific details not covered by the",
        "pre-loaded knowledge above. Prefer summaries for planning; retrieve full",
        "content only when implementing.",
        "</on-demand-knowledge>",
    ].join("\n");
}
// ---------------------------------------------------------------------------
// Registry-Based Retrieval
// ---------------------------------------------------------------------------
/**
 * Query the prompt registry for on-demand knowledge entries that match
 * the given criteria. Returns registry entries (not full content).
 */
export function queryOnDemandEntries(registry, options) {
    return registry.knowledge.filter((entry) => {
        if (entry.tier !== "on-demand")
            return false;
        if (options.tags && options.tags.length > 0) {
            const hasMatchingTag = options.tags.some((tag) => entry.tags.includes(tag));
            if (!hasMatchingTag)
                return false;
        }
        if (options.role) {
            const roleMatch = entry.roles.length === 0 || entry.roles.includes(options.role);
            if (!roleMatch)
                return false;
        }
        return true;
    });
}
// ---------------------------------------------------------------------------
// Disk-Based Content Retrieval
// ---------------------------------------------------------------------------
/**
 * Retrieve full knowledge artifact content from disk.
 *
 * Reads knowledge artifacts from .orqa/ directories, matching by tags, role,
 * or text content. Returns full artifact body text (not summary) within
 * the specified token budget.
 */
export function retrieveKnowledge(projectPath, options) {
    const budget = options.tokenBudget ?? 2000;
    const results = [];
    let usedTokens = 0;
    const knowledgeDirs = [
        path.join(projectPath, ".orqa", "process", "knowledge"),
        path.join(projectPath, "app", ".orqa", "process", "knowledge"),
    ];
    for (const dir of knowledgeDirs) {
        if (!fs.existsSync(dir))
            continue;
        let entries;
        try {
            entries = fs.readdirSync(dir, { withFileTypes: true });
        }
        catch {
            continue;
        }
        for (const entry of entries) {
            if (!entry.isFile() || !entry.name.endsWith(".md"))
                continue;
            if (usedTokens >= budget)
                break;
            const filePath = path.join(dir, entry.name);
            const artifact = readKnowledgeArtifact(filePath, options);
            if (!artifact)
                continue;
            if (usedTokens + artifact.tokens <= budget) {
                results.push(artifact);
                usedTokens += artifact.tokens;
            }
        }
    }
    return results;
}
/**
 * Read a single knowledge artifact file and check if it matches the query.
 * Returns null if it does not match or cannot be read.
 */
function readKnowledgeArtifact(filePath, options) {
    let content;
    try {
        content = fs.readFileSync(filePath, "utf-8");
    }
    catch {
        return null;
    }
    const parsed = parseFrontmatterFromContent(content);
    if (!parsed)
        return null;
    const [fm, body] = parsed;
    if (fm.type !== "knowledge")
        return null;
    const title = fm.title ?? "";
    const id = fm.id ?? "";
    const tags = Array.isArray(fm.tags) ? fm.tags : [];
    // Tag filtering
    if (options.tags && options.tags.length > 0) {
        const category = fm.category ?? "";
        const allTags = [...tags, category].filter(Boolean);
        const hasMatch = options.tags.some((t) => allTags.includes(t) ||
            title.toLowerCase().includes(t.toLowerCase()));
        if (!hasMatch)
            return false;
    }
    // Text query filtering
    if (options.textQuery) {
        const query = options.textQuery.toLowerCase();
        const searchable = `${title} ${body}`.toLowerCase();
        if (!searchable.includes(query))
            return null;
    }
    const trimmedBody = body.trim();
    const tokens = estimateTokens(trimmedBody);
    return {
        id,
        title,
        content: trimmedBody,
        tokens,
        tags,
        plugin: "project",
    };
}
/**
 * Count on-demand knowledge entries in the registry.
 */
export function countOnDemandEntries(registry) {
    return registry.knowledge.filter((e) => e.tier === "on-demand").length;
}
//# sourceMappingURL=knowledge-retrieval.js.map