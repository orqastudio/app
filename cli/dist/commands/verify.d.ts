/**
 * Verify command — run all checks in one go.
 *
 * orqa verify
 *
 * Runs: integrity validation, version drift, license audit, readme audit,
 * and plugin-drift detection (compares installed file hashes against source hashes).
 * Exits non-zero if any check fails.
 */
/** Run all governance checks: integrity, version, license, readme, and plugin drift. */
export declare function runVerifyCommand(): Promise<void>;
//# sourceMappingURL=verify.d.ts.map