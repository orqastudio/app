/**
 * Prompt registry — scans installed plugins for knowledge declarations and
 * prompt sections, builds a lookup registry, and caches it to disk.
 *
 * Runs at `orqa plugin install` / `orqa plugin refresh` time, similar to
 * the workflow resolver. The runtime prompt pipeline reads only the cached
 * registry — never raw plugin manifests.
 *
 * Five-stage pipeline (this module covers Stage 1 — Plugin Registry):
 *   1. Plugin Registry (this) → 2. Schema Assembly → 3. Section Resolution
 *   → 4. Token Budgeting → 5. Prompt Output
 */
import * as fs from "node:fs";
import * as path from "node:path";
// ---------------------------------------------------------------------------
// Discovery
// ---------------------------------------------------------------------------
/** Scans installed plugins for knowledge declarations and prompt sections. */
export function scanPluginPromptContributions(projectRoot) {
    const knowledge = [];
    const sections = [];
    const contributors = [];
    const errors = [];
    const containers = ["plugins", "connectors", "integrations"];
    for (const container of containers) {
        const containerDir = path.join(projectRoot, container);
        if (!fs.existsSync(containerDir))
            continue;
        let entries;
        try {
            entries = fs.readdirSync(containerDir, { withFileTypes: true });
        }
        catch {
            continue;
        }
        for (const entry of entries) {
            if (!entry.isDirectory() || entry.name.startsWith("."))
                continue;
            const pluginDir = path.join(containerDir, entry.name);
            const manifestPath = path.join(pluginDir, "orqa-plugin.json");
            if (!fs.existsSync(manifestPath))
                continue;
            let manifest;
            try {
                manifest = JSON.parse(fs.readFileSync(manifestPath, "utf-8"));
            }
            catch (err) {
                errors.push(`${manifestPath}: ${err instanceof Error ? err.message : String(err)}`);
                continue;
            }
            const pluginName = manifest.name ?? entry.name;
            const provides = manifest.provides;
            if (!provides)
                continue;
            const source = classifySource(manifest);
            let contributed = false;
            // Scan knowledge_declarations
            const declarations = provides.knowledge_declarations;
            if (Array.isArray(declarations)) {
                for (const decl of declarations) {
                    const contentFile = decl.content_file
                        ? path.join(pluginDir, decl.content_file)
                        : null;
                    knowledge.push({
                        id: decl.id,
                        plugin: pluginName,
                        source,
                        tier: decl.tier,
                        roles: decl.roles ?? [],
                        stages: decl.stages ?? [],
                        paths: decl.paths ?? [],
                        tags: decl.tags ?? [],
                        priority: decl.priority,
                        summary: decl.summary ?? null,
                        content_file: contentFile,
                    });
                    contributed = true;
                }
            }
            // Scan prompt_sections
            const sectionDecls = provides.prompt_sections;
            if (Array.isArray(sectionDecls)) {
                for (const sec of sectionDecls) {
                    sections.push({
                        id: sec.id,
                        plugin: pluginName,
                        source,
                        type: sec.type,
                        role: sec.role ?? null,
                        stage: sec.stage ?? null,
                        priority: sec.priority,
                        content_file: path.join(pluginDir, sec.content_file),
                    });
                    contributed = true;
                }
            }
            if (contributed) {
                contributors.push(pluginName);
            }
        }
    }
    return { knowledge, sections, contributors, errors };
}
// ---------------------------------------------------------------------------
// Source Classification
// ---------------------------------------------------------------------------
/**
 * Classify a plugin manifest into a knowledge source category.
 *
 * Priority order (highest first):
 *   1. project-rules — not from plugins, but reserved for .orqa/process/rules/
 *   2. project-knowledge — not from plugins, but reserved for .orqa/process/knowledge/
 *   3. plugin — most installed plugins
 *   4. core — the core framework plugin
 */
function classifySource(manifest) {
    const role = manifest.role;
    if (role === "core:framework")
        return "core";
    return "plugin";
}
// ---------------------------------------------------------------------------
// Registry Building
// ---------------------------------------------------------------------------
/** Build the prompt registry from all installed plugins and write to disk. */
export function buildPromptRegistry(projectRoot) {
    const scan = scanPluginPromptContributions(projectRoot);
    const registry = {
        version: 1,
        built_at: new Date().toISOString(),
        knowledge: scan.knowledge,
        sections: scan.sections,
        contributors: scan.contributors,
        errors: scan.errors,
    };
    // Write to .orqa/prompt-registry.json
    const outputDir = path.join(projectRoot, ".orqa");
    if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
    }
    const outputPath = path.join(outputDir, "prompt-registry.json");
    fs.writeFileSync(outputPath, JSON.stringify(registry, null, 2) + "\n", "utf-8");
    return registry;
}
// ---------------------------------------------------------------------------
// Registry Reading
// ---------------------------------------------------------------------------
/** Read the cached prompt registry from disk. Returns null if not found. */
export function readPromptRegistry(projectRoot) {
    const registryPath = path.join(projectRoot, ".orqa", "prompt-registry.json");
    if (!fs.existsSync(registryPath))
        return null;
    try {
        const content = fs.readFileSync(registryPath, "utf-8");
        return JSON.parse(content);
    }
    catch {
        return null;
    }
}
// ---------------------------------------------------------------------------
// Lookup Helpers
// ---------------------------------------------------------------------------
/**
 * Query the registry for knowledge entries matching a (role, stage, paths) tuple.
 *
 * This is the main lookup used by Stage 2 (Schema Assembly) of the pipeline.
 */
export function queryKnowledge(registry, query) {
    return registry.knowledge.filter((entry) => {
        // Tier-based filtering
        if (entry.tier === "always") {
            // Always-tier: match by role and/or paths
            const roleMatch = entry.roles.length === 0 ||
                (query.role !== undefined && entry.roles.includes(query.role));
            const pathMatch = entry.paths.length === 0 || matchesAnyPath(query.filePaths, entry.paths);
            return roleMatch || pathMatch;
        }
        if (entry.tier === "stage-triggered") {
            // Stage-triggered: must match stage, optionally role/paths
            if (query.stage === undefined)
                return false;
            const stageMatch = entry.stages.includes(query.stage);
            if (!stageMatch)
                return false;
            const roleMatch = entry.roles.length === 0 ||
                (query.role !== undefined && entry.roles.includes(query.role));
            return roleMatch;
        }
        // On-demand: not returned by query — retrieved via semantic search
        return false;
    });
}
/** Query the registry for prompt sections matching a (role, stage) tuple. */
export function querySections(registry, query) {
    return registry.sections.filter((section) => {
        const roleMatch = section.role === null ||
            (query.role !== undefined && section.role === query.role);
        const stageMatch = section.stage === null ||
            (query.stage !== undefined && section.stage === query.stage);
        return roleMatch && stageMatch;
    });
}
// ---------------------------------------------------------------------------
// Path Matching (simple glob support)
// ---------------------------------------------------------------------------
/**
 * Check if any of the file paths match any of the glob patterns.
 *
 * Supports simple glob patterns:
 * - `**` matches any number of path segments
 * - `*` matches any characters within a single path segment
 */
function matchesAnyPath(filePaths, patterns) {
    if (!filePaths || filePaths.length === 0)
        return false;
    for (const fp of filePaths) {
        const normalized = fp.replace(/\\/g, "/");
        for (const pattern of patterns) {
            if (simpleGlobMatch(normalized, pattern))
                return true;
        }
    }
    return false;
}
/** Simple glob matcher — converts glob to regex. */
function simpleGlobMatch(filePath, pattern) {
    // Escape regex special chars except * and **
    const regexStr = pattern
        .replace(/\\/g, "/")
        .replace(/[.+^${}()|[\]]/g, "\\$&")
        .replace(/\*\*/g, "\0")
        .replace(/\*/g, "[^/]*")
        .replace(/\0/g, ".*");
    try {
        return new RegExp(`^${regexStr}$`).test(filePath);
    }
    catch {
        return false;
    }
}
// ---------------------------------------------------------------------------
// CLI Integration
// ---------------------------------------------------------------------------
/**
 * Run prompt registry build and print results.
 *
 * Called from `cmdPluginSync` in install.ts and plugin refresh commands.
 */
export function runPromptRegistryBuild(projectRoot) {
    const registry = buildPromptRegistry(projectRoot);
    const totalEntries = registry.knowledge.length + registry.sections.length;
    if (totalEntries === 0 && registry.errors.length === 0) {
        // No prompt contributions found — nothing to report
        return;
    }
    if (registry.errors.length > 0) {
        console.log("  Prompt registry warnings:");
        for (const err of registry.errors) {
            console.log(`    - ${err}`);
        }
    }
    if (registry.knowledge.length > 0) {
        const byTier = groupBy(registry.knowledge, (k) => k.tier);
        const tierSummary = Object.entries(byTier)
            .map(([tier, entries]) => `${entries.length} ${tier}`)
            .join(", ");
        console.log(`  Prompt registry: ${registry.knowledge.length} knowledge entries (${tierSummary})`);
    }
    if (registry.sections.length > 0) {
        console.log(`  Prompt registry: ${registry.sections.length} prompt sections`);
    }
    if (registry.contributors.length > 0) {
        const relPath = ".orqa/prompt-registry.json";
        console.log(`  Prompt registry cached → ${relPath} (${registry.contributors.length} plugin(s))`);
    }
}
// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------
function groupBy(items, keyFn) {
    const result = {};
    for (const item of items) {
        const key = keyFn(item);
        if (!result[key])
            result[key] = [];
        result[key].push(item);
    }
    return result;
}
//# sourceMappingURL=prompt-registry.js.map