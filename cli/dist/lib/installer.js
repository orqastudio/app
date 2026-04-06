/**
 * Plugin installer — download, extract, and manage plugin installations.
 *
 * Plugins are distributed as .tar.gz archives from GitHub Releases.
 * Local path installs are also supported for development.
 *
 * Post-install hooks: callers pass an optional `postInstall` callback in InstallOptions.
 * Connector-specific setup (e.g. .claude/ wiring) lives in the connector package,
 * not here. The CLI installer is platform-agnostic.
 */
import * as fs from "node:fs";
import * as path from "node:path";
import * as crypto from "node:crypto";
import { readLockfile, writeLockfile } from "./lockfile.js";
import { readManifest, validateManifest } from "./manifest.js";
import { PLATFORM_CONFIG } from "@orqastudio/types";
/**
 * Detect methodology exclusivity conflicts (legacy role-based check).
 *
 * Plugins with a `core:*` role are exclusive — only one per domain
 * (framework, discovery, delivery, governance) is allowed per project.
 * This function scans installed plugins and returns a conflict if the
 * incoming plugin's core role is already occupied.
 * @param manifest - The incoming plugin's manifest.
 * @param projectRoot - Absolute path to the project root.
 * @returns A conflict object if a conflict is found, or null.
 */
export function detectMethodologyConflict(manifest, projectRoot) {
    const role = manifest.role;
    if (!role || !role.startsWith("core:"))
        return null;
    for (const container of ["plugins", "connectors", "sidecars"]) {
        const dir = path.join(projectRoot, container);
        if (!fs.existsSync(dir))
            continue;
        for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
            if (!entry.isDirectory() || entry.name.startsWith("."))
                continue;
            try {
                const installed = readManifest(path.join(dir, entry.name));
                if (installed.name === manifest.name)
                    continue;
                if (installed.role === role) {
                    return {
                        incomingPlugin: manifest.name,
                        role,
                        existingPlugin: installed.name,
                    };
                }
            }
            catch {
                /* skip invalid */
            }
        }
    }
    return null;
}
/**
 * P5-26: Enforce one-methodology-plugin-per-project.
 *
 * Reads installed plugins and rejects installation when another methodology
 * plugin is already present. Reinstalling the same plugin (same name) succeeds.
 * Non-methodology plugins are unaffected.
 * @param manifest - The incoming plugin's manifest.
 * @param projectRoot - Absolute path to the project root.
 * @throws {Error} With descriptive message naming the conflicting plugin.
 */
function enforceOneMethodology(manifest, projectRoot) {
    const purpose = manifest.purpose ?? [];
    if (!purpose.includes("methodology"))
        return;
    for (const container of ["plugins", "connectors"]) {
        const dir = path.join(projectRoot, container);
        if (!fs.existsSync(dir))
            continue;
        for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
            if (!entry.isDirectory() || entry.name.startsWith("."))
                continue;
            try {
                const installed = readManifest(path.join(dir, entry.name));
                if (installed.name === manifest.name)
                    continue; // Same plugin = update
                const installedPurpose = installed.purpose ?? [];
                if (installedPurpose.includes("methodology")) {
                    throw new Error(`Cannot install methodology plugin '${manifest.name}': project already has ` +
                        `methodology plugin '${installed.name}' installed. Only one methodology plugin ` +
                        `is allowed per project. Uninstall '${installed.name}' first.`);
                }
            }
            catch (err) {
                // Re-throw constraint violations; swallow manifest read errors
                if (err instanceof Error && err.message.startsWith("Cannot install"))
                    throw err;
            }
        }
    }
}
/**
 * P5-27: Enforce one-workflow-plugin-per-stage.
 *
 * Reads installed plugins and rejects installation when another plugin already
 * fills the same stage slot. Reinstalling the same plugin (same name) succeeds.
 * Plugins without stageSlot are unaffected.
 * @param manifest - The incoming plugin's manifest.
 * @param projectRoot - Absolute path to the project root.
 * @throws {Error} With descriptive message naming the conflicting plugin and slot.
 */
function enforceOnePerStage(manifest, projectRoot) {
    const incomingSlot = manifest.stage_slot;
    if (!incomingSlot)
        return;
    for (const container of ["plugins", "connectors"]) {
        const dir = path.join(projectRoot, container);
        if (!fs.existsSync(dir))
            continue;
        for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
            if (!entry.isDirectory() || entry.name.startsWith("."))
                continue;
            try {
                const installed = readManifest(path.join(dir, entry.name));
                if (installed.name === manifest.name)
                    continue; // Same plugin = update
                const existingSlot = installed.stage_slot;
                if (existingSlot === incomingSlot) {
                    throw new Error(`Cannot install workflow plugin '${manifest.name}': stage slot '${incomingSlot}' ` +
                        `is already filled by '${installed.name}'. Only one workflow plugin may occupy ` +
                        `each stage slot. Uninstall '${installed.name}' first.`);
                }
            }
            catch (err) {
                if (err instanceof Error && err.message.startsWith("Cannot install"))
                    throw err;
            }
        }
    }
}
/**
 * Detect relationship key collisions between an incoming plugin and
 * existing definitions (core.json + already-installed plugins).
 * @param manifest - The incoming plugin's manifest.
 * @param projectRoot - Absolute path to the project root.
 * @returns Array of key collision results.
 */
function detectCollisions(manifest, projectRoot) {
    const collisions = [];
    // Build existing relationship map: key → { source, rel }
    const existing = [];
    // Core relationships
    for (const rel of PLATFORM_CONFIG.relationships) {
        existing.push({ source: "core", rel });
    }
    // Already-installed plugin relationships
    for (const container of ["plugins", "connectors", "sidecars"]) {
        const dir = path.join(projectRoot, container);
        if (!fs.existsSync(dir))
            continue;
        for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
            if (!entry.isDirectory() || entry.name.startsWith("."))
                continue;
            try {
                const installed = readManifest(path.join(dir, entry.name));
                if (installed.name === manifest.name)
                    continue; // Skip self
                for (const rel of installed.provides.relationships) {
                    existing.push({ source: installed.name, rel });
                }
            }
            catch {
                /* skip invalid */
            }
        }
    }
    // Check incoming relationships against existing
    for (const incoming of manifest.provides.relationships) {
        const match = existing.find((e) => e.rel.key === incoming.key);
        if (match) {
            collisions.push({
                key: incoming.key,
                existingSource: match.source,
                existingDescription: match.rel.description ?? "",
                existingSemantic: match.rel.semantic,
                existingFrom: match.rel.from ?? [],
                existingTo: match.rel.to ?? [],
                incomingDescription: incoming.description ?? "",
                incomingSemantic: incoming.semantic,
                incomingFrom: incoming.from ?? [],
                incomingTo: incoming.to ?? [],
                semanticMatch: match.rel.semantic === incoming.semantic,
            });
        }
    }
    return collisions;
}
/**
 * Resolve the plugins directory for a project.
 * @param projectRoot - Absolute path to the project root.
 * @returns Absolute path to the plugins directory.
 */
function pluginsDir(projectRoot) {
    return path.join(projectRoot, "plugins");
}
/**
 * Install a plugin from a GitHub release archive or local path.
 * @param options - Installation options including source, version, and project root.
 * @returns Installation result with name, version, and path.
 */
export async function installPlugin(options) {
    const root = options.projectRoot ?? process.cwd();
    const dir = pluginsDir(root);
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
    // Detect source type
    if (fs.existsSync(options.source)) {
        return installFromLocalPath(options.source, dir, options.postInstall);
    }
    // GitHub owner/repo format
    if (options.source.includes("/") && !options.source.includes(path.sep)) {
        return installFromGitHub(options.source, options.version, dir, root, options.postInstall);
    }
    throw new Error(`Invalid source: ${options.source}. Use owner/repo for GitHub or a local path.`);
}
async function installFromLocalPath(source, pluginsDirectory, postInstall) {
    const projectRoot = path.dirname(pluginsDirectory);
    const manifest = readManifest(source);
    const errors = validateManifest(manifest);
    if (errors.length > 0) {
        throw new Error(`Invalid plugin manifest:\n  ${errors.join("\n  ")}`);
    }
    // P5-26: enforce one-methodology constraint.
    enforceOneMethodology(manifest, projectRoot);
    // P5-27: enforce one-per-stage constraint.
    enforceOnePerStage(manifest, projectRoot);
    const collisions = detectCollisions(manifest, projectRoot);
    const targetDir = path.join(pluginsDirectory, manifest.name.replace(/^@[^/]+\//, ""));
    if (fs.existsSync(targetDir)) {
        fs.rmSync(targetDir, { recursive: true });
    }
    fs.cpSync(source, targetDir, { recursive: true });
    postInstall?.(targetDir, projectRoot);
    // P5-28: read post-install action flags from the manifest.
    const requiresSchemaRecomposition = manifest.affects_schema ?? false;
    // A non-empty enforcement array means the plugin participates in enforcement generation.
    const requiresEnforcementRegeneration = (manifest.enforcement?.length ?? 0) > 0;
    return {
        name: manifest.name,
        version: manifest.version,
        path: targetDir,
        source: "local",
        collisions,
        requiresSchemaRecomposition,
        requiresEnforcementRegeneration,
    };
}
async function installFromGitHub(repo, version, pluginsDirectory, projectRoot, postInstall) {
    // Resolve latest version if not specified
    const tag = version ?? (await fetchLatestTag(repo));
    const archiveUrl = `https://github.com/${repo}/releases/download/${tag}/${repo.split("/")[1]}-${tag}.tar.gz`;
    console.log(`Downloading ${repo}@${tag}...`);
    const response = await fetch(archiveUrl);
    if (!response.ok) {
        throw new Error(`Failed to download ${archiveUrl}: ${response.status} ${response.statusText}`);
    }
    const buffer = Buffer.from(await response.arrayBuffer());
    const sha256 = crypto.createHash("sha256").update(buffer).digest("hex");
    // Extract to temp, then move to plugins/
    const tmpDir = path.join(pluginsDirectory, `.tmp-${Date.now()}`);
    fs.mkdirSync(tmpDir, { recursive: true });
    try {
        await extractTarGz(buffer, tmpDir);
        // Find the manifest in extracted contents
        const entries = fs.readdirSync(tmpDir);
        const extractedDir = entries.length === 1 ? path.join(tmpDir, entries[0]) : tmpDir;
        const manifest = readManifest(extractedDir);
        const errors = validateManifest(manifest);
        if (errors.length > 0) {
            throw new Error(`Invalid plugin manifest:\n  ${errors.join("\n  ")}`);
        }
        // P5-26: enforce one-methodology constraint before moving files.
        enforceOneMethodology(manifest, projectRoot);
        // P5-27: enforce one-per-stage constraint before moving files.
        enforceOnePerStage(manifest, projectRoot);
        const pluginDir = path.join(pluginsDirectory, manifest.name.replace(/^@[^/]+\//, ""));
        if (fs.existsSync(pluginDir)) {
            fs.rmSync(pluginDir, { recursive: true });
        }
        fs.renameSync(extractedDir, pluginDir);
        // Update lockfile
        const lockfile = readLockfile(projectRoot);
        lockfile.plugins = lockfile.plugins.filter((p) => p.name !== manifest.name);
        lockfile.plugins.push({
            name: manifest.name,
            version: manifest.version,
            repo,
            sha256,
            installedAt: new Date().toISOString(),
        });
        writeLockfile(projectRoot, lockfile);
        const collisions = detectCollisions(manifest, projectRoot);
        postInstall?.(pluginDir, projectRoot);
        console.log(`Installed ${manifest.name}@${manifest.version}`);
        // P5-28: read post-install action flags from the manifest.
        const requiresSchemaRecomposition = manifest.affects_schema ?? false;
        // A non-empty enforcement array means the plugin participates in enforcement generation.
        const requiresEnforcementRegeneration = (manifest.enforcement?.length ?? 0) > 0;
        return {
            name: manifest.name,
            version: manifest.version,
            path: pluginDir,
            source: "github",
            collisions,
            requiresSchemaRecomposition,
            requiresEnforcementRegeneration,
        };
    }
    finally {
        if (fs.existsSync(tmpDir)) {
            fs.rmSync(tmpDir, { recursive: true });
        }
    }
}
async function fetchLatestTag(repo) {
    const response = await fetch(`https://api.github.com/repos/${repo}/releases/latest`, {
        headers: { Accept: "application/vnd.github.v3+json" },
    });
    if (!response.ok) {
        throw new Error(`Failed to fetch latest release for ${repo}: ${response.status}`);
    }
    const data = (await response.json());
    return data.tag_name;
}
async function extractTarGz(buffer, targetDir) {
    // Use tar CLI for extraction (available on all platforms)
    const { execSync } = await import("node:child_process");
    const tmpArchive = path.join(targetDir, "archive.tar.gz");
    fs.writeFileSync(tmpArchive, buffer);
    execSync(`tar -xzf "${tmpArchive}" -C "${targetDir}"`);
    fs.unlinkSync(tmpArchive);
}
/**
 * Uninstall a plugin by name.
 * @param name - The plugin name to uninstall.
 * @param projectRoot - Optional absolute path to the project root (defaults to cwd).
 */
export function uninstallPlugin(name, projectRoot) {
    const root = projectRoot ?? process.cwd();
    const dir = pluginsDir(root);
    const shortName = name.replace(/^@[^/]+\//, "");
    const pluginDir = path.join(dir, shortName);
    if (!fs.existsSync(pluginDir)) {
        throw new Error(`Plugin not found: ${name} (expected at ${pluginDir})`);
    }
    fs.rmSync(pluginDir, { recursive: true });
    // Update lockfile
    const lockfile = readLockfile(root);
    lockfile.plugins = lockfile.plugins.filter((p) => p.name !== name);
    writeLockfile(root, lockfile);
    console.log(`Uninstalled ${name}`);
}
/**
 * List all installed plugins by scanning plugins/, connectors/, and sidecars/.
 *
 * plugins/ is scanned two levels deep because plugins are organised into
 * taxonomy subdirectories: plugins/<taxonomy>/<plugin>/orqa-plugin.json.
 * connectors/ and sidecars/ are scanned one level deep.
 * @param projectRoot - Optional absolute path to the project root (defaults to cwd).
 * @returns Array of install results for all found plugins.
 */
export function listInstalledPlugins(projectRoot) {
    const root = projectRoot ?? process.cwd();
    const results = [];
    // Scan plugins/ two levels deep: plugins/<taxonomy>/<plugin>/orqa-plugin.json.
    const pluginsDir = path.join(root, "plugins");
    if (fs.existsSync(pluginsDir)) {
        for (const taxonomy of fs.readdirSync(pluginsDir, { withFileTypes: true })) {
            if (!taxonomy.isDirectory() || taxonomy.name.startsWith("."))
                continue;
            const taxonomyPath = path.join(pluginsDir, taxonomy.name);
            for (const plugin of fs.readdirSync(taxonomyPath, { withFileTypes: true })) {
                if (!plugin.isDirectory() || plugin.name.startsWith("."))
                    continue;
                const pluginPath = path.join(taxonomyPath, plugin.name);
                if (!fs.existsSync(path.join(pluginPath, "orqa-plugin.json")))
                    continue;
                try {
                    const manifest = readManifest(pluginPath);
                    const lockfile = readLockfile(root);
                    const locked = lockfile.plugins.find((p) => p.name === manifest.name);
                    results.push({
                        name: manifest.name,
                        version: manifest.version,
                        path: pluginPath,
                        source: locked ? "github" : "local",
                        collisions: [],
                        requiresSchemaRecomposition: false,
                        requiresEnforcementRegeneration: false,
                    });
                }
                catch {
                    // Skip invalid plugins
                }
            }
        }
    }
    // Scan connectors/ and sidecars/ one level deep.
    for (const container of ["connectors", "sidecars"]) {
        const dir = path.join(root, container);
        if (!fs.existsSync(dir))
            continue;
        for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
            if (!entry.isDirectory() || entry.name.startsWith("."))
                continue;
            const pluginPath = path.join(dir, entry.name);
            if (!fs.existsSync(path.join(pluginPath, "orqa-plugin.json")))
                continue;
            try {
                const manifest = readManifest(pluginPath);
                const lockfile = readLockfile(root);
                const locked = lockfile.plugins.find((p) => p.name === manifest.name);
                results.push({
                    name: manifest.name,
                    version: manifest.version,
                    path: pluginPath,
                    source: locked ? "github" : "local",
                    collisions: [],
                    requiresSchemaRecomposition: false,
                    requiresEnforcementRegeneration: false,
                });
            }
            catch {
                // Skip invalid plugins
            }
        }
    }
    return results;
}
//# sourceMappingURL=installer.js.map