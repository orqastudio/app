/**
 * Verify command — run all checks in one go.
 *
 * orqa verify
 *
 * Runs: integrity validation, version drift, license audit, readme audit,
 * and plugin-drift detection (compares installed file hashes against source hashes).
 * Exits non-zero if any check fails.
 */
import { execSync } from "node:child_process";
import { callDaemonGraph } from "../lib/daemon-client.js";
import { getRoot } from "../lib/root.js";
/** Run all governance checks: integrity, version, license, readme, and plugin drift. */
export async function runVerifyCommand() {
    const root = getRoot();
    let failed = false;
    const checks = [
        { name: "integrity", cmd: "orqa enforce ." },
        { name: "version", cmd: "orqa version check" },
        { name: "license", cmd: "orqa repo license" },
        { name: "readme", cmd: "orqa repo readme" },
    ];
    for (const check of checks) {
        try {
            execSync(check.cmd, { cwd: root, stdio: "inherit" });
        }
        catch {
            failed = true;
        }
    }
    // Plugin drift check — compares source_hash vs installed_hash for every
    // file in every plugin_installation record. Fails verify when any plugin
    // has drifted, listing the drifted files so the user knows what changed.
    try {
        const report = await callDaemonGraph("GET", "/plugins/drift");
        if (!report.clean) {
            console.error("[verify] plugin drift detected:");
            for (const plugin of report.drifted_plugins) {
                console.error(`  plugin: ${plugin.plugin_name}`);
                for (const file of plugin.drifted_files) {
                    console.error(`    drifted: ${file.path}`);
                    console.error(`      source:    ${file.source_hash}`);
                    console.error(`      installed: ${file.installed_hash}`);
                }
            }
            failed = true;
        }
    }
    catch (err) {
        // Daemon unreachable — skip drift check rather than hard-failing.
        // Drift detection requires a running daemon with SurrealDB available.
        console.warn(`[verify] plugin drift check skipped: ${err instanceof Error ? err.message : String(err)}`);
    }
    if (failed) {
        process.exit(1);
    }
}
//# sourceMappingURL=verify.js.map