/**
 * Prompt registry generator — builds .orqa/prompt-registry.json from installed plugins.
 *
 * Reads knowledge_declarations from all installed plugin manifests and merges them
 * into a single registry file. Each declaration's content_file is resolved from the
 * plugin directory to a project-root-relative path.
 *
 * Called by orqa plugin install (after schema composition) and orqa install.
 * Satisfies P3 (Generated, Not Loaded): the registry is generated from the plugin
 * registry, not hand-maintained.
 */
import * as fs from "node:fs";
import * as path from "node:path";
import { listInstalledPlugins } from "./installer.js";
import { readManifest } from "./manifest.js";
// ---------------------------------------------------------------------------
// Generation
// ---------------------------------------------------------------------------
/**
 * Generate .orqa/prompt-registry.json from all installed plugins.
 *
 * Scans all installed plugins, reads each plugin's knowledge_declarations,
 * resolves content_file paths to project-root-relative paths, and writes the
 * merged registry to .orqa/prompt-registry.json.
 *
 * Duplicate ids (same plugin + id) are deduplicated — last declaration wins
 * in case a plugin is scanned more than once.
 * @param projectRoot - Absolute path to the project root.
 * @returns The absolute path to the written registry file.
 */
export function generatePromptRegistry(projectRoot) {
    const entries = [];
    // Track seen ids to detect cross-plugin collisions (warn but include both
    // by appending a plugin-scoped suffix).
    const seenIds = new Map();
    for (const plugin of listInstalledPlugins(projectRoot)) {
        let manifest;
        try {
            manifest = readManifest(plugin.path);
        }
        catch {
            // Skip plugins with unreadable manifests.
            continue;
        }
        // knowledge_declarations is a PluginManifest field in @orqastudio/types,
        // accessed via a typed cast to satisfy the compiler.
        const declarations = manifest
            .knowledge_declarations ?? [];
        for (const decl of declarations) {
            // Resolve content_file from plugin-relative to project-root-relative.
            let resolvedContentFile = null;
            if (decl.content_file) {
                const absPath = path.join(plugin.path, decl.content_file);
                resolvedContentFile = path.relative(projectRoot, absPath).replace(/\\/g, "/");
            }
            // Build a stable unique id: if two plugins declare the same id,
            // suffix the duplicate with a plugin-scoped fragment.
            let entryId = decl.id;
            if (seenIds.has(entryId) && seenIds.get(entryId) !== manifest.name) {
                entryId = `${decl.id}-${manifest.name.replace(/[^a-z0-9]/gi, "-")}`;
            }
            seenIds.set(decl.id, manifest.name);
            entries.push({
                id: entryId,
                plugin: manifest.name,
                source: "plugin",
                tier: decl.tier,
                roles: decl.roles ?? [],
                stages: decl.stages ?? [],
                paths: decl.paths ?? [],
                tags: decl.tags ?? [],
                priority: decl.priority,
                summary: decl.summary ?? null,
                content_file: resolvedContentFile,
            });
        }
    }
    const registry = {
        version: 1,
        built_at: new Date().toISOString(),
        knowledge: entries,
    };
    const outputPath = path.join(projectRoot, ".orqa", "prompt-registry.json");
    const outputDir = path.dirname(outputPath);
    if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
    }
    fs.writeFileSync(outputPath, JSON.stringify(registry, null, 2) + "\n", "utf-8");
    return outputPath;
}
//# sourceMappingURL=prompt-registry.js.map