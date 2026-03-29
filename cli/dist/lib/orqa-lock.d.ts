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
import type { OrqaLockfile, OrqaPluginLockEntry } from "@orqastudio/types";
/**
 * Read the orqa-lock.json file from `.orqa/` inside the project root.
 * Returns an empty lockfile structure when the file does not yet exist.
 * @param projectRoot - Absolute path to the project root.
 * @returns The parsed lockfile, or an initialised empty lockfile.
 */
export declare function readOrqaLock(projectRoot: string): OrqaLockfile;
/**
 * Write the orqa-lock.json file to `.orqa/` inside the project root.
 * @param projectRoot - Absolute path to the project root.
 * @param lock - The lockfile data to write.
 */
export declare function writeOrqaLock(projectRoot: string, lock: OrqaLockfile): void;
/**
 * Compute the SHA-256 hash of a file's contents.
 * Returns the hash prefixed with "sha256:" for unambiguous identification.
 * @param filePath - Absolute path to the file to hash.
 * @returns "sha256:<hex-digest>".
 */
export declare function computeManifestHash(filePath: string): string;
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
export declare function buildLockEntry(version: string, shortPath: string, pluginDir: string, contentHashes: Record<string, string>): OrqaPluginLockEntry;
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
export declare function isPluginManifestChanged(projectRoot: string, pluginDir: string, pluginName: string): boolean;
//# sourceMappingURL=orqa-lock.d.ts.map