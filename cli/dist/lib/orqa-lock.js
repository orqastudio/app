/**
 * orqa-lock.json — plugin version and content integrity tracking.
 *
 * Stored at `.orqa/orqa-lock.json`. Written after every install so that
 * `orqa plugin outdated` can detect version bumps and manifest changes
 * without network access.
 *
 * Separate from `plugins.lock.json` (the GitHub-archive SHA-256 lockfile),
 * which only covers remote installs. orqa-lock covers all install paths
 * including first-party (in-repo) plugins and the plugin sync pipeline.
 */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { join } from "node:path";
const LOCK_FILE = "orqa-lock.json";
/**
 * Read the orqa-lock.json file from `.orqa/` inside the project root.
 * Returns an empty lockfile structure when the file does not yet exist.
 * @param projectRoot - Absolute path to the project root.
 * @returns The parsed lockfile, or an initialised empty lockfile.
 */
export function readOrqaLock(projectRoot) {
    const lockPath = join(projectRoot, ".orqa", LOCK_FILE);
    if (!existsSync(lockPath)) {
        return { lockfileVersion: 1, plugins: {} };
    }
    return JSON.parse(readFileSync(lockPath, "utf8"));
}
/**
 * Write the orqa-lock.json file to `.orqa/` inside the project root.
 * @param projectRoot - Absolute path to the project root.
 * @param lock - The lockfile data to write.
 */
export function writeOrqaLock(projectRoot, lock) {
    const lockPath = join(projectRoot, ".orqa", LOCK_FILE);
    writeFileSync(lockPath, JSON.stringify(lock, null, 2) + "\n");
}
/**
 * Compute the SHA-256 hash of a file's contents.
 * Returns the hash prefixed with "sha256:" for unambiguous identification.
 * @param filePath - Absolute path to the file to hash.
 * @returns "sha256:<hex-digest>".
 */
export function computeManifestHash(filePath) {
    const content = readFileSync(filePath);
    return "sha256:" + createHash("sha256").update(content).digest("hex");
}
/**
 * Build a lock entry for a plugin directory.
 *
 * Hashes the orqa-plugin.json manifest and records all provided content
 * file hashes. The caller provides content hashes from the CopyResult so
 * we do not need to re-read files here.
 * @param version - The plugin version string from the manifest.
 * @param shortPath - Relative path to the plugin from the project root.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param contentHashes - Hashes of the installed content files (from CopyResult).
 * @returns A complete OrqaPluginLockEntry.
 */
export function buildLockEntry(version, shortPath, pluginDir, contentHashes) {
    const manifestPath = join(pluginDir, "orqa-plugin.json");
    return {
        version,
        path: shortPath,
        installedAt: new Date().toISOString(),
        manifestHash: computeManifestHash(manifestPath),
        contentHashes,
    };
}
/**
 * Check whether a plugin's manifest has changed since it was last installed.
 *
 * Returns true when the plugin has never been installed (no lock entry) or
 * when the current orqa-plugin.json hash differs from the locked value.
 * @param projectRoot - Absolute path to the project root.
 * @param pluginDir - Absolute path to the plugin directory.
 * @param pluginName - Plugin name as it appears in project.json.
 * @returns true when the plugin appears outdated, false when up-to-date.
 */
export function isPluginManifestChanged(projectRoot, pluginDir, pluginName) {
    const lock = readOrqaLock(projectRoot);
    const entry = lock.plugins[pluginName];
    if (!entry)
        return true; // never installed via this mechanism
    const manifestPath = join(pluginDir, "orqa-plugin.json");
    if (!existsSync(manifestPath))
        return true;
    const currentHash = computeManifestHash(manifestPath);
    return currentHash !== entry.manifestHash;
}
//# sourceMappingURL=orqa-lock.js.map