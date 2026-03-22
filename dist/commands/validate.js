/**
 * Validation command — delegates to the Rust orqa-validation binary.
 *
 * The Rust binary is the single source of truth for validation. It reads
 * schemas from plugin manifests, validates frontmatter, relationships,
 * and graph integrity.
 *
 * Falls back to the TypeScript validator if the binary is not built.
 *
 * orqa validate [path] [--json] [--fix]
 */
import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { runValidateSchemaCommand } from "./validate-schema.js";
const USAGE = `
Usage: orqa validate [subcommand|path] [options]

Run integrity validation on the specified path (defaults to current directory).

Subcommands:
  schema              Validate project.json and plugin manifests against schemas

Options:
  --fix               Auto-fix objectively fixable errors (e.g. missing inverses)
  --json              Output results as JSON
  --help, -h          Show this help message
`.trim();
/**
 * Find the Rust validation binary. Checks common build locations.
 */
function findRustBinary(projectRoot) {
    const candidates = [
        join(projectRoot, "target", "release", "orqa-validation"),
        join(projectRoot, "target", "release", "orqa-validation.exe"),
        join(projectRoot, "target", "debug", "orqa-validation"),
        join(projectRoot, "target", "debug", "orqa-validation.exe"),
        join(projectRoot, "app", "backend", "target", "release", "orqa-validation"),
        join(projectRoot, "app", "backend", "target", "release", "orqa-validation.exe"),
        join(projectRoot, "app", "backend", "target", "debug", "orqa-validation"),
        join(projectRoot, "app", "backend", "target", "debug", "orqa-validation.exe"),
    ];
    for (const candidate of candidates) {
        if (existsSync(candidate))
            return candidate;
    }
    return null;
}
/**
 * Run validation via the Rust binary.
 */
function runRustValidator(binaryPath, targetPath, jsonOutput, autoFix) {
    const args = [targetPath];
    if (autoFix)
        args.push("--fix");
    try {
        const output = execFileSync(binaryPath, args, {
            encoding: "utf-8",
            timeout: 30000,
            windowsHide: true,
        });
        return { exitCode: 0, output };
    }
    catch (e) {
        const err = e;
        return {
            exitCode: err.status ?? 2,
            output: err.stdout ?? err.stderr ?? String(e),
        };
    }
}
/**
 * Fallback: run the TypeScript validator (deprecated — remove when Rust binary
 * is reliably available in all build configurations).
 */
async function runTypeScriptFallback(targetPath, jsonOutput, autoFix) {
    // Dynamic import so the TS validator modules are only loaded if needed
    const { buildGraph } = await import("../validator/graph.js");
    const { buildCheckContext, runChecksWithSummary } = await import("../validator/checker.js");
    const { applyFixes } = await import("../validator/fixer.js");
    const graph = buildGraph({ projectRoot: targetPath });
    const ctx = buildCheckContext(targetPath);
    let summary = runChecksWithSummary(graph, ctx);
    if (autoFix && summary.findings.some((f) => f.autoFixable)) {
        const fixSummary = applyFixes(summary.findings, graph, ctx, targetPath);
        if (fixSummary.applied > 0) {
            if (!jsonOutput) {
                console.log(`Auto-fixed ${fixSummary.applied} issue(s).`);
            }
            const rebuiltGraph = buildGraph({ projectRoot: targetPath });
            summary = runChecksWithSummary(rebuiltGraph, ctx);
        }
    }
    if (jsonOutput) {
        console.log(JSON.stringify(summary, null, 2));
    }
    else {
        const { errors, warnings, totalFindings } = summary;
        if (totalFindings === 0) {
            console.log("Integrity check passed. 0 errors, 0 warnings.");
        }
        else {
            const byCategory = new Map();
            for (const f of summary.findings) {
                const list = byCategory.get(f.category) ?? [];
                list.push(f);
                byCategory.set(f.category, list);
            }
            for (const [category, findings] of byCategory) {
                console.log(`\n${category} (${findings.length}):`);
                for (const f of findings) {
                    const icon = f.severity === "error" ? "E" : "W";
                    console.log(`  [${icon}] ${f.artifactId}: ${f.message}`);
                }
            }
            console.log(`\n${errors} error(s), ${warnings} warning(s).`);
            if (errors > 0)
                process.exit(1);
        }
    }
}
export async function runValidateCommand(args) {
    if (args.includes("--help") || args.includes("-h")) {
        console.log(USAGE);
        return;
    }
    if (args[0] === "schema") {
        await runValidateSchemaCommand(args.slice(1));
        return;
    }
    const jsonOutput = args.includes("--json");
    const autoFix = args.includes("--fix");
    const targetPath = args.find((a) => !a.startsWith("--")) ?? process.cwd();
    // Try Rust binary first (single source of truth)
    const rustBinary = findRustBinary(targetPath);
    if (rustBinary) {
        const { exitCode, output } = runRustValidator(rustBinary, targetPath, jsonOutput, autoFix);
        if (jsonOutput) {
            // Rust binary outputs JSON — pass through
            process.stdout.write(output);
        }
        else {
            // Rust binary outputs JSON — parse and format for human display
            try {
                const report = JSON.parse(output);
                const checks = report.checks ?? [];
                const errors = checks.filter((c) => c.severity === "Error" || c.severity === "error").length;
                const warnings = checks.length - errors;
                if (checks.length === 0) {
                    console.log("Integrity check passed. 0 errors, 0 warnings.");
                }
                else {
                    const byCategory = new Map();
                    for (const c of checks) {
                        const list = byCategory.get(c.category) ?? [];
                        list.push(c);
                        byCategory.set(c.category, list);
                    }
                    for (const [category, findings] of byCategory) {
                        console.log(`\n${category} (${findings.length}):`);
                        for (const f of findings) {
                            const icon = f.severity === "Error" || f.severity === "error" ? "E" : "W";
                            console.log(`  [${icon}] ${f.artifact_id}: ${f.message}`);
                        }
                    }
                    console.log(`\n${errors} error(s), ${warnings} warning(s).`);
                    if (report.fixes_applied?.length > 0) {
                        console.log(`Auto-fixed ${report.fixes_applied.length} issue(s).`);
                    }
                }
            }
            catch {
                // If JSON parsing fails, just output raw
                process.stdout.write(output);
            }
        }
        process.exit(exitCode);
    }
    // Fallback to TypeScript validator (deprecated)
    if (!jsonOutput) {
        console.error("Note: Rust validator binary not found — using TypeScript fallback.");
        console.error("Build with: cargo build --release -p orqa-validation\n");
    }
    await runTypeScriptFallback(targetPath, jsonOutput, autoFix);
}
//# sourceMappingURL=validate.js.map