/**
 * Version sync — propagate a canonical version across all package.json,
 * orqa-plugin.json, Cargo.toml, and plugin.json files in a dev environment.
 */
export interface VersionSyncResult {
    version: string;
    updated: string[];
    skipped: string[];
}
/**
 * Read the canonical version from the VERSION file.
 */
export declare function readCanonicalVersion(projectRoot: string): string;
/**
 * Write the canonical version to the VERSION file.
 */
export declare function writeCanonicalVersion(projectRoot: string, version: string): void;
/**
 * Sync a version across all package.json, orqa-plugin.json, Cargo.toml,
 * and .claude-plugin/plugin.json files found in the dev environment.
 */
export declare function syncVersions(projectRoot: string, version: string): VersionSyncResult;
/**
 * Check if all packages in the dev environment have the same version.
 */
export declare function checkVersionDrift(projectRoot: string): Array<{
    file: string;
    version: string;
}>;
//# sourceMappingURL=version-sync.d.ts.map