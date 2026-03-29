/**
 * Code quality checks — CLI adapter for the shared validation engine +
 * plugin-provided lint tools.
 *
 * orqa check              Run validation engine + all plugin lint tools
 * orqa check validate     Run the shared validation engine only
 * orqa check <tool>       Run a specific plugin tool (eslint, clippy, etc.)
 * orqa check configure    Generate config files from coding standards rules
 * orqa check verify       Run all governance checks (integrity, version, license, readme)
 * orqa check enforce      Enforcement + integrity validation
 * orqa check audit        Full governance audit with escalation scanning
 * orqa check schema       Validate project.json and plugin manifests
 *
 * The validation engine (engine/validation/) runs the same checks as the LSP:
 * schema validation, relationship type checks, broken links, missing inverses,
 * status transitions, and more. Plugin tools (eslint, clippy, etc.) are
 * discovered from installed plugin manifests (orqa-plugin.json).
 */
import { execSync } from "node:child_process";
import * as fs from "node:fs";
import * as path from "node:path";
import { loadPluginTools, extractEnforcementEntries, generateConfigs, } from "../lib/config-generator.js";
import { getRoot } from "../lib/root.js";
import { runValidation, formatReport, } from "../lib/validation-engine.js";
const USAGE = `
Usage: orqa check [subcommand]

Run all code quality checks (validation engine + plugin lint tools):

Subcommands:
  validate      Run the shared validation engine only (artifact graph integrity)
  lint          Run all lint tools (clippy, eslint, svelte-check)
  format        Run all formatters (rustfmt, prettier)
  configure     Generate config files from coding standards rules
  verify        Run all governance checks (integrity, version, license, readme)
  enforce       Enforcement + integrity validation (--fix, --mechanism, --report)
  audit         Full governance audit with escalation scanning
  schema        Validate project.json and plugin manifests
  <tool-name>   Run a specific plugin tool (e.g. eslint, clippy, svelte-check)

Running 'orqa check' with no subcommand runs the validation engine AND all
plugin lint tools. The validation engine runs the same checks as the LSP
(schema validation, broken links, missing inverses, relationship types, etc.).
`.trim();
/** Tool names that belong to the "lint" category. */
const LINT_TOOLS = new Set(["clippy", "eslint", "svelte-check"]);
/** Tool names that belong to the "format" category. */
const FORMAT_TOOLS = new Set(["rustfmt", "prettier"]);
/**
 * Dispatch the check command: validation engine, plugin lint tools, or a specific subcommand.
 * @param args - CLI arguments after "check".
 */
export async function runCheckCommand(args) {
    if (args[0] === "--help" || args[0] === "-h") {
        console.log(USAGE);
        return;
    }
    const root = getRoot();
    const target = args[0];
    if (target === "configure") {
        await cmdConfigure(root);
        return;
    }
    // Merged subcommands: verify, enforce, audit, schema
    if (target === "verify") {
        const { runVerifyCommand } = await import("./verify.js");
        await runVerifyCommand();
        return;
    }
    if (target === "enforce") {
        const { runEnforceCommand } = await import("./enforce.js");
        const enforceExit = await runEnforceCommand(root, args.slice(1));
        if (enforceExit !== 0)
            process.exit(enforceExit);
        return;
    }
    if (target === "audit") {
        const { runAuditCommand } = await import("./audit.js");
        await runAuditCommand(args.slice(1));
        return;
    }
    if (target === "schema") {
        const { runEnforceCommand } = await import("./enforce.js");
        await runEnforceCommand(root, ["schema", ...args.slice(1)]);
        return;
    }
    // "orqa check validate" — run only the shared validation engine
    if (target === "validate") {
        const validationFailed = await runValidationStep(root, args.includes("--fix"));
        if (validationFailed)
            process.exit(1);
        return;
    }
    // "orqa check lint" — run all lint-category plugin tools
    if (target === "lint") {
        const pluginFailed = await runPluginToolsByCategory(root, LINT_TOOLS);
        if (pluginFailed)
            process.exit(1);
        return;
    }
    // "orqa check format" — run all format-category plugin tools
    if (target === "format") {
        const pluginFailed = await runPluginToolsByCategory(root, FORMAT_TOOLS);
        if (pluginFailed)
            process.exit(1);
        return;
    }
    // If a specific tool name is given, run only that plugin tool (no validation engine)
    if (target) {
        const pluginFailed = await runPluginTools(root, target);
        if (pluginFailed)
            process.exit(1);
        return;
    }
    // Default: no subcommand → run validation engine + all plugin lint tools
    let failed = false;
    console.log("=== Artifact validation (shared engine) ===");
    const validationFailed = await runValidationStep(root, args.includes("--fix"));
    if (validationFailed)
        failed = true;
    console.log("\n=== Plugin lint tools ===");
    const pluginFailed = await runPluginTools(root, undefined);
    if (pluginFailed)
        failed = true;
    if (failed) {
        process.exit(1);
    }
}
/**
 * Run the shared validation engine (same checks as LSP).
 * @param root - Absolute path to the project root.
 * @param autoFix - Whether to apply automatic fixes.
 * @returns True if errors were found.
 */
async function runValidationStep(root, autoFix) {
    try {
        const { report, exitCode } = await runValidation(root, root, autoFix);
        const { text, errors } = formatReport(report);
        console.log(text);
        return errors > 0 || exitCode !== 0;
    }
    catch (e) {
        const msg = e instanceof Error ? e.message : String(e);
        console.error(msg);
        // Validation engine not available — warn but don't block lint tools
        return false;
    }
}
/**
 * Run plugin-provided lint tools. If `target` is specified, run only that tool.
 * @param root - Absolute path to the project root.
 * @param target - Specific tool name to run, or undefined to run all.
 * @returns True if any tool failed.
 */
async function runPluginTools(root, target) {
    const pluginTools = loadPluginTools(root);
    if (pluginTools.size === 0) {
        console.log("No plugins with tools installed.");
        return false;
    }
    const toRun = [];
    for (const [pluginName, tools] of pluginTools) {
        for (const [toolName, tool] of tools) {
            if (target && toolName !== target)
                continue;
            toRun.push({ pluginName, toolName, tool });
        }
    }
    if (target && toRun.length === 0) {
        console.error(`Unknown tool: ${target}`);
        console.error("\nAvailable tools:");
        for (const [pluginName, tools] of pluginTools) {
            for (const toolName of tools.keys()) {
                console.error(`  ${toolName} (from ${pluginName})`);
            }
        }
        process.exit(1);
    }
    let failed = false;
    for (const { pluginName, toolName, tool } of toRun) {
        const projectDir = findProjectDir(root, pluginName);
        if (!projectDir) {
            console.log(`  ${toolName} (${pluginName}) — skipped, no matching project found`);
            continue;
        }
        console.log(`  ${toolName} (${pluginName}) in ${path.relative(root, projectDir)}...`);
        try {
            execSync(tool.command, { cwd: projectDir, stdio: "inherit" });
        }
        catch {
            failed = true;
        }
    }
    return failed;
}
async function cmdConfigure(root) {
    console.log("Generating config files from coding standards rules...\n");
    const pluginTools = loadPluginTools(root);
    const entries = [
        ...extractEnforcementEntries(path.join(root, ".orqa/learning/rules")),
        ...extractEnforcementEntries(path.join(root, "app/.orqa/learning/rules")),
    ];
    if (entries.length === 0) {
        console.log("No enforcement entries found in coding standards rules.");
        console.log("Add enforcement entries to rules in .orqa/learning/rules/ with plugin/tool/config.");
        return;
    }
    const generated = generateConfigs(root, entries, pluginTools);
    if (generated.length === 0) {
        console.log("No matching plugin tools installed for the enforcement entries.");
        return;
    }
    for (const g of generated) {
        console.log(`  ${g.file} — ${g.entries} entries`);
    }
    console.log(`\nGenerated ${generated.length} config file(s).`);
}
/**
 * Run all plugin tools whose names match the given category set.
 * @param root - Absolute path to the project root.
 * @param category - Set of tool names to run (e.g. lint or format tools).
 * @returns True if any tool failed.
 */
async function runPluginToolsByCategory(root, category) {
    const pluginTools = loadPluginTools(root);
    let failed = false;
    let ran = 0;
    for (const [pluginName, tools] of pluginTools) {
        for (const [toolName, tool] of tools) {
            if (!category.has(toolName))
                continue;
            ran++;
            const projectDir = findProjectDir(root, pluginName);
            if (!projectDir) {
                console.log(`  ${toolName} (${pluginName}) — skipped, no matching project found`);
                continue;
            }
            console.log(`  ${toolName} (${pluginName}) in ${path.relative(root, projectDir)}...`);
            try {
                execSync(tool.command, { cwd: projectDir, stdio: "inherit" });
            }
            catch {
                failed = true;
            }
        }
    }
    if (ran === 0) {
        const names = [...category].join(", ");
        console.log(`No installed plugin tools match category: ${names}`);
    }
    return failed;
}
/**
 * Find the project directory where a plugin's tools should run.
 * @param root - Absolute path to the project root.
 * @param pluginName - Name of the plugin (e.g. "svelte", "tauri").
 * @returns The directory to run the tool in, or null if not found.
 */
function findProjectDir(root, pluginName) {
    if (pluginName.includes("svelte") || pluginName.includes("typescript")) {
        const appDir = path.join(root, "app");
        if (fs.existsSync(path.join(appDir, "package.json")))
            return appDir;
        if (fs.existsSync(path.join(root, "package.json")))
            return root;
    }
    if (pluginName.includes("tauri") || pluginName.includes("rust")) {
        const srcTauri = path.join(root, "app/src-tauri");
        if (fs.existsSync(path.join(srcTauri, "Cargo.toml")))
            return srcTauri;
    }
    return fs.existsSync(root) ? root : null;
}
//# sourceMappingURL=check.js.map