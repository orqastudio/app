/**
 * Version sync — propagate a canonical version across all package.json,
 * orqa-plugin.json, Cargo.toml, and plugin.json files in a dev environment.
 *
 * The VERSION file at the dev repo root is the single source of truth.
 * No submodule may define its own version independently.
 */
export interface VersionSyncResult {
    version: string;
    updated: string[];
    skipped: string[];
}
export interface VersionDrift {
    file: string;
    found: string;
    expected: string;
    type: "package" | "dependency" | "cargo";
}
/**
 * Validate a version string.
 * Must be semver: X.Y.Z or X.Y.Z-suffix (e.g. 0.1.0-dev, 1.0.0-rc.1)
 * @param version - The version string to validate.
 * @returns True if the version string is valid semver.
 */
export declare function isValidVersion(version: string): boolean;
/**
 * Read the canonical version from the VERSION file.
 * @param projectRoot - Absolute path to the project root.
 * @returns The canonical version string.
 */
export declare function readCanonicalVersion(projectRoot: string): string;
/**
 * Write the canonical version to the VERSION file.
 * @param projectRoot - Absolute path to the project root.
 * @param version - The version string to write.
 */
export declare function writeCanonicalVersion(projectRoot: string, version: string): void;
/**
 * Sync a version across all package.json, orqa-plugin.json, Cargo.toml,
 * and .claude-plugin/plugin.json files found in the dev environment.
 * @param projectRoot - Absolute path to the project root.
 * @param version - The version string to sync across all files.
 * @returns Result listing updated and skipped files.
 */
export declare function syncVersions(projectRoot: string, version: string): VersionSyncResult;
/**
 * Check if all packages in the dev environment have the same version.
 * Checks package versions, `\@orqastudio/*` dependency versions, and Cargo.toml.
 * @param projectRoot - Absolute path to the dev repo root.
 * @returns Array of VersionDrift entries, one per file/field with a mismatched version.
 */
export declare function checkVersionDrift(projectRoot: string): VersionDrift[];
//# sourceMappingURL=version-sync.d.ts.map