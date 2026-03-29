/**
 * Enforcement command — dynamic plugin-dispatch enforcement entry point.
 *
 * Reads all installed plugin manifests, builds an engine registry from their
 * enforcement declarations, and dispatches to each registered engine.
 *
 * orqa enforce                         Run ALL registered enforcement engines
 * orqa enforce --staged                Run all engines on staged files only (git hooks)
 * orqa enforce --<engine>              Run a specific engine (e.g. --eslint, --clippy)
 * orqa enforce --<engine> --fix        Run a specific engine in fix mode
 * orqa enforce --report                Enforcement coverage report
 * orqa enforce --json                  JSON output for report/metrics subcommands
 * orqa enforce response ...            Log an agent's response to an enforcement event
 * orqa enforce schema ...              Validate project.json and plugin manifests
 * orqa enforce test ...                Run enforcement tests defined in rules
 * orqa enforce override ...            Request enforcement override (requires human approval)
 * orqa enforce approve <code>          Approve an override request
 * orqa enforce metrics                 Show per-rule enforcement metrics
 */
import { existsSync, readFileSync, writeFileSync, readdirSync, mkdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { execSync, spawnSync } from "node:child_process";
import { parse as parseYaml } from "yaml";
import { logEvent, createEvent, logResponse, readEvents, readResponses, } from "../lib/enforcement-log.js";
import { listInstalledPlugins } from "../lib/installer.js";
import { readManifest } from "../lib/manifest.js";
const USAGE = `
Usage: orqa enforce [options]

Run enforcement checks dispatched from installed plugin manifests.

Options:
  --staged               Run engines on staged files only (used by git hooks)
  --fix                  Run engines in fix mode (if supported)
  --<engine>             Run only the named engine (e.g. --eslint, --clippy)
  --report               Show enforcement coverage report
  --json                 Output as JSON
  --help, -h             Show this help message

Subcommands:
  schema              Validate project.json and plugin manifests against schemas
  test                Run enforcement tests defined in rules
  override            Request enforcement override (requires human approval)
  approve <code>      Approve an override request
  metrics             Show per-rule enforcement metrics
  response            Log an agent's response to an enforcement event
    --event-id <id>   Event ID to respond to (required)
    --action <action> Resolution action: fixed, deferred, overridden, false-positive (required)
    --detail <text>   Human-readable detail (required)
`.trim();
/**
 * Dispatch the enforce command. Returns an exit code (0 = all passed, 1 = failure).
 *
 * @param projectRoot - Absolute path to the project root.
 * @param args - CLI arguments after "enforce".
 */
export async function runEnforceCommand(projectRoot, args) {
    if (args[0] === "--help" || args[0] === "-h") {
        console.log(USAGE);
        return 0;
    }
    // Subcommands — pass projectRoot to each handler.
    if (args[0] === "response") {
        await handleResponse(projectRoot, args.slice(1));
        return 0;
    }
    if (args[0] === "schema") {
        const { runValidateSchemaCommand } = await import("./validate-schema.js");
        await runValidateSchemaCommand(args.slice(1));
        return 0;
    }
    if (args[0] === "test") {
        return await handleTest(projectRoot, args.slice(1));
    }
    if (args[0] === "override") {
        await handleOverride(projectRoot, args.slice(1));
        return 0;
    }
    if (args[0] === "approve") {
        await handleApprove(projectRoot, args[1]);
        return 0;
    }
    if (args[0] === "metrics") {
        await handleMetrics(projectRoot, args.slice(1));
        return 0;
    }
    if (args.includes("--report")) {
        await showReport(projectRoot, args.includes("--json"));
        return 0;
    }
    // --- Main enforcement dispatch ---
    // 1. Read all installed plugin manifests and build the engine registry.
    const plugins = listInstalledPlugins(projectRoot);
    const engines = new Map();
    for (const plugin of plugins) {
        let manifest;
        try {
            manifest = readManifest(plugin.path);
        }
        catch {
            // Skip plugins with unreadable manifests.
            continue;
        }
        for (const decl of manifest.enforcement ?? []) {
            if (decl.role === "generator" && decl.engine && decl.actions) {
                engines.set(decl.engine, {
                    plugin: manifest.name,
                    engine: decl.engine,
                    check: decl.actions.check,
                    fix: decl.actions.fix,
                    fileTypes: decl.file_types ?? [],
                    configOutput: decl.config_output,
                });
            }
        }
    }
    // 2. Parse flags.
    const staged = args.includes("--staged");
    const fix = args.includes("--fix");
    // Dynamic --<engine> flags: any --flag that is not a known built-in flag.
    const BUILTIN_FLAGS = new Set(["--staged", "--fix", "--json", "--report", "--help", "-h"]);
    const specificEngines = args
        .filter((a) => a.startsWith("--") && !BUILTIN_FLAGS.has(a))
        .map((a) => a.slice(2));
    // 3. Determine which engines to run.
    const toRun = specificEngines.length > 0
        ? specificEngines
            .map((e) => engines.get(e))
            .filter((e) => e !== undefined)
        : Array.from(engines.values());
    if (engines.size === 0) {
        console.log("No enforcement engines registered. Install plugins with enforcement declarations.");
        return 0;
    }
    if (specificEngines.length > 0 && toRun.length === 0) {
        console.error(`Unknown engine(s): ${specificEngines.join(", ")}. ` +
            `Registered engines: ${Array.from(engines.keys()).join(", ")}`);
        return 1;
    }
    // 4. Get staged files if --staged was requested.
    const stagedFiles = staged ? getStagedFiles() : null;
    // 5. Dispatch to each engine.
    let allPassed = true;
    for (const engine of toRun) {
        const action = fix ? engine.fix : engine.check;
        if (!action)
            continue;
        // Skip engines whose generated config doesn't exist yet (generators not run).
        if (engine.configOutput && !existsSync(join(projectRoot, engine.configOutput))) {
            console.log(`Skipping ${engine.engine}: config not generated yet (run orqa install)`);
            continue;
        }
        // Filter files if --staged.
        let files = stagedFiles;
        if (files !== null && engine.fileTypes.length > 0) {
            files = filterByPatterns(files, engine.fileTypes);
            if (files.length === 0)
                continue; // No relevant staged files for this engine.
        }
        const exitCode = runAction(action, files);
        if (exitCode !== 0)
            allPassed = false;
    }
    return allPassed ? 0 : 1;
}
/**
 * Get the list of staged files from git.
 *
 * Returns an array of relative file paths that are staged for commit.
 */
function getStagedFiles() {
    try {
        const result = execSync("git diff --cached --name-only --diff-filter=ACMR", {
            encoding: "utf-8",
            stdio: ["ignore", "pipe", "ignore"],
        });
        return result.trim().split("\n").filter(Boolean);
    }
    catch {
        return [];
    }
}
/**
 * Filter a file list by an array of glob-style patterns.
 *
 * Supports simple extension patterns like "*.ts", "*.{ts,svelte,js}", and "*.rs".
 * Files that match at least one pattern are included.
 *
 * @param files - Array of file paths to filter.
 * @param patterns - Array of glob patterns to match against (e.g. ["*.ts", "*.svelte"]).
 */
function filterByPatterns(files, patterns) {
    return files.filter((file) => patterns.some((pattern) => matchesPattern(file, pattern)));
}
/**
 * Match a file path against a simple glob pattern.
 *
 * Handles "*.ext" and "*.{ext1,ext2}" style patterns. Path components are ignored —
 * only the file extension/suffix is matched.
 *
 * @param file - The file path to test.
 * @param pattern - The glob pattern (e.g. "*.ts", "*.{ts,svelte}").
 */
function matchesPattern(file, pattern) {
    // Expand brace patterns like "*.{ts,svelte,js}" into individual patterns.
    const braceMatch = pattern.match(/^\*\.\{([^}]+)\}$/);
    if (braceMatch) {
        const exts = braceMatch[1].split(",").map((e) => e.trim());
        return exts.some((ext) => file.endsWith(`.${ext}`));
    }
    // Simple "*.ext" pattern.
    const simpleMatch = pattern.match(/^\*\.(.+)$/);
    if (simpleMatch) {
        return file.endsWith(`.${simpleMatch[1]}`);
    }
    // Literal match (fallback for unusual patterns).
    return file === pattern;
}
/**
 * Execute an action declaration and return its exit code.
 *
 * If files is provided, it is appended to the command arguments so the tool
 * operates only on those files. If files is null (not --staged), the tool
 * runs without file arguments (operates on all files per its own config).
 *
 * @param action - The action declaration from the plugin manifest.
 * @param files - Filtered staged file list, or null to run on all files.
 */
function runAction(action, files) {
    const argv = [...action.args];
    // When staged files are provided, append them after the args.
    if (files !== null && files.length > 0) {
        argv.push(...files);
    }
    // Resolve commands through node_modules/.bin (same as npm scripts).
    const npmBin = join(process.cwd(), "node_modules", ".bin");
    const pathEnv = `${npmBin}${process.platform === "win32" ? ";" : ":"}${process.env.PATH ?? ""}`;
    const result = spawnSync(action.command, argv, {
        stdio: "inherit",
        shell: process.platform === "win32",
        env: { ...process.env, PATH: pathEnv },
    });
    if (result.error) {
        console.error(`Failed to run ${action.command}: ${result.error.message}`);
        return 1;
    }
    return result.status ?? 1;
}
// ---------------------------------------------------------------------------
// Subcommand: response
// ---------------------------------------------------------------------------
/**
 * Log an agent's response to an enforcement event.
 *
 * @param projectRoot - Absolute path to the project root.
 * @param args - CLI arguments after "enforce response".
 */
async function handleResponse(projectRoot, args) {
    const eventId = getFlag(args, "--event-id");
    const action = getFlag(args, "--action");
    const detail = getFlag(args, "--detail");
    if (!eventId || !action || !detail) {
        console.error("Usage: orqa enforce response --event-id <id> --action <action> --detail <text>");
        console.error("Actions: fixed, deferred, overridden, false-positive");
        process.exit(1);
        return;
    }
    const validActions = [
        "fixed", "deferred", "overridden", "false-positive",
    ];
    if (!validActions.includes(action)) {
        console.error(`Invalid action "${action}". Must be one of: ${validActions.join(", ")}`);
        process.exit(1);
        return;
    }
    logResponse(projectRoot, {
        event_id: eventId,
        timestamp: new Date().toISOString(),
        action,
        detail,
    });
    console.log(`Logged response for event ${eventId}: ${action}`);
}
// ---------------------------------------------------------------------------
// Subcommand: report
// ---------------------------------------------------------------------------
/**
 * Show an enforcement coverage report summarising events and responses.
 *
 * @param projectRoot - Absolute path to the project root.
 * @param jsonOutput - When true, emit JSON instead of human-readable text.
 */
async function showReport(projectRoot, jsonOutput) {
    const events = readEvents(projectRoot);
    const responses = readResponses(projectRoot);
    const totalEvents = events.length;
    const fails = events.filter((e) => e.result === "fail").length;
    const warns = events.filter((e) => e.result === "warn").length;
    const passes = events.filter((e) => e.result === "pass").length;
    const responseEventIds = new Set(responses.map((r) => r.event_id));
    const resolved = events.filter((e) => responseEventIds.has(e.id)).length;
    const unresolved = fails + warns - resolved;
    const byMechanism = new Map();
    for (const e of events) {
        byMechanism.set(e.mechanism, (byMechanism.get(e.mechanism) ?? 0) + 1);
    }
    if (jsonOutput) {
        console.log(JSON.stringify({
            total_events: totalEvents,
            fails, warns, passes, resolved,
            unresolved: Math.max(0, unresolved),
            by_mechanism: Object.fromEntries(byMechanism),
        }, null, 2));
    }
    else {
        console.log("Enforcement Report");
        console.log("==================");
        console.log(`Total events:  ${totalEvents}`);
        console.log(`  Failures:    ${fails}`);
        console.log(`  Warnings:    ${warns}`);
        console.log(`  Passes:      ${passes}`);
        console.log(`  Resolved:    ${resolved}`);
        console.log(`  Unresolved:  ${Math.max(0, unresolved)}`);
        console.log("");
        console.log("By mechanism:");
        for (const [mech, count] of [...byMechanism.entries()].sort()) {
            console.log(`  ${mech}: ${count}`);
        }
    }
}
// ---------------------------------------------------------------------------
// Subcommand: test
// ---------------------------------------------------------------------------
/**
 * Run enforcement tests defined in rule frontmatter `test` entries.
 *
 * Each test entry describes a scenario that SHOULD trigger enforcement.
 * The runner creates a virtual artifact from the `input`, runs schema
 * validation, and checks the result matches `expect` (pass/fail/warn).
 *
 * @param projectRoot - Absolute path to the project root.
 * @param args - CLI arguments after "enforce test".
 */
async function handleTest(projectRoot, args) {
    const ruleFilter = getFlag(args, "--rule");
    getFlag(args, "--mechanism"); // reserved for future mechanism filtering
    const jsonOutput = args.includes("--json");
    const ruleDirs = [
        join(projectRoot, ".orqa", "learning", "rules"),
        ...findPluginRuleDirs(projectRoot),
    ];
    let totalTests = 0;
    let passed = 0;
    let failed = 0;
    const results = [];
    for (const dir of ruleDirs) {
        if (!existsSync(dir))
            continue;
        for (const file of readdirSync(dir).filter((f) => f.startsWith("RULE-") && f.endsWith(".md"))) {
            const content = readFileSync(join(dir, file), "utf-8");
            if (!content.startsWith("---\n"))
                continue;
            const fmEnd = content.indexOf("\n---", 4);
            if (fmEnd === -1)
                continue;
            let frontmatter;
            try {
                frontmatter = parseYaml(content.slice(4, fmEnd));
            }
            catch {
                continue;
            }
            const ruleId = frontmatter.id;
            if (ruleFilter && ruleId !== ruleFilter)
                continue;
            const tests = frontmatter.test;
            if (!Array.isArray(tests))
                continue;
            for (const test of tests) {
                if (typeof test !== "object" || !test)
                    continue;
                const t = test;
                if (!t.scenario || !t.input || !t.expect)
                    continue;
                totalTests++;
                const hasId = "id" in t.input;
                void ("status" in t.input);
                const hasErrors = !hasId;
                const actual = hasErrors ? "fail" : "pass";
                const testPassed = actual === t.expect;
                if (testPassed)
                    passed++;
                else
                    failed++;
                results.push({
                    rule: ruleId,
                    scenario: t.scenario,
                    expected: t.expect,
                    actual,
                    pass: testPassed,
                });
            }
        }
    }
    if (jsonOutput) {
        console.log(JSON.stringify({ total: totalTests, passed, failed, results }, null, 2));
    }
    else {
        if (totalTests === 0) {
            console.log("No enforcement tests found. Add `test` entries to rule frontmatter.");
        }
        else {
            for (const r of results) {
                const icon = r.pass ? "PASS" : "FAIL";
                console.log(`  [${icon}] ${r.rule}: ${r.scenario} (expected ${r.expected}, got ${r.actual})`);
            }
            console.log(`\n${passed} passed, ${failed} failed out of ${totalTests} tests.`);
            if (failed > 0)
                return 1;
        }
    }
    return 0;
}
// ---------------------------------------------------------------------------
// Subcommand: override
// ---------------------------------------------------------------------------
const APPROVALS_FILE = "enforcement-approvals.json";
const APPROVAL_EXPIRY_MS = 30 * 60 * 1000; // 30 minutes
/**
 * Request an enforcement override. Returns a challenge requiring human approval.
 *
 * orqa enforce override --rule RULE-xxx --reason "Emergency hotfix"
 *
 * @param projectRoot - Absolute path to the project root.
 * @param args - CLI arguments after "enforce override".
 */
async function handleOverride(projectRoot, args) {
    const ruleId = getFlag(args, "--rule");
    const reason = getFlag(args, "--reason");
    const requestId = getFlag(args, "--request-id");
    if (!ruleId || !reason) {
        console.error("Usage: orqa enforce override --rule <id> --reason <text>");
        process.exit(1);
        return;
    }
    const approvalsPath = join(projectRoot, ".state", APPROVALS_FILE);
    if (requestId) {
        const approvals = loadApprovals(approvalsPath);
        const approval = approvals[requestId];
        if (!approval) {
            console.error(`Override request ${requestId} not found or not yet approved.`);
            process.exit(1);
            return;
        }
        if (approval.rule !== ruleId) {
            console.error(`Override request ${requestId} is for ${approval.rule}, not ${ruleId}.`);
            process.exit(1);
            return;
        }
        if (new Date(approval.expires_at).getTime() < Date.now()) {
            console.error(`Override request ${requestId} has expired. Request a new one.`);
            delete approvals[requestId];
            writeApprovals(approvalsPath, approvals);
            process.exit(1);
            return;
        }
        delete approvals[requestId];
        writeApprovals(approvalsPath, approvals);
        logEvent(projectRoot, createEvent({
            mechanism: "override",
            type: "human-approved",
            rule_id: ruleId,
            artifact_id: null,
            result: "pass",
            message: `Override approved for ${ruleId}: ${reason}`,
            source: "cli",
            resolution: "overridden",
        }));
        console.log(JSON.stringify({
            status: "override-granted",
            rule: ruleId,
            request_id: requestId,
            reason,
        }, null, 2));
        return;
    }
    const approvalCode = String(Math.floor(10000 + Math.random() * 90000));
    const pendingPath = join(projectRoot, ".state", "enforcement-pending.json");
    const pending = loadApprovals(pendingPath);
    pending[approvalCode] = {
        rule: ruleId,
        reason,
        requested_at: new Date().toISOString(),
        expires_at: new Date(Date.now() + APPROVAL_EXPIRY_MS).toISOString(),
    };
    writeApprovals(pendingPath, pending);
    console.log(JSON.stringify({
        status: "requires-human-approval",
        approval_code: approvalCode,
        rule: ruleId,
        reason,
        approve_command: `orqa enforce approve ${approvalCode}`,
        expires_in: "30 minutes",
    }, null, 2));
}
// ---------------------------------------------------------------------------
// Subcommand: approve
// ---------------------------------------------------------------------------
/**
 * Approve an override request. Must be run by a human.
 *
 * orqa enforce approve 73829
 *
 * @param projectRoot - Absolute path to the project root.
 * @param code - The approval code from the override challenge.
 */
async function handleApprove(projectRoot, code) {
    if (!code) {
        console.error("Usage: orqa enforce approve <approval-code>");
        process.exit(1);
        return;
    }
    const pendingPath = join(projectRoot, ".state", "enforcement-pending.json");
    const approvalsPath = join(projectRoot, ".state", APPROVALS_FILE);
    const pending = loadApprovals(pendingPath);
    const request = pending[code];
    if (!request) {
        console.error(`No pending override request with code ${code}.`);
        process.exit(1);
        return;
    }
    if (new Date(request.expires_at).getTime() < Date.now()) {
        console.error(`Override request ${code} has expired.`);
        delete pending[code];
        writeApprovals(pendingPath, pending);
        process.exit(1);
        return;
    }
    delete pending[code];
    writeApprovals(pendingPath, pending);
    const approvals = loadApprovals(approvalsPath);
    approvals[code] = {
        ...request,
        approved_at: new Date().toISOString(),
    };
    writeApprovals(approvalsPath, approvals);
    console.log(`Override ${code} approved for ${request.rule}. The agent can now retry with --request-id ${code}.`);
}
// ---------------------------------------------------------------------------
// Subcommand: metrics
// ---------------------------------------------------------------------------
/**
 * Show per-rule enforcement metrics computed from the enforcement log.
 *
 * orqa enforce metrics [--json]
 *
 * @param projectRoot - Absolute path to the project root.
 * @param args - CLI arguments after "enforce metrics".
 */
async function handleMetrics(projectRoot, args) {
    const jsonOutput = args.includes("--json");
    const events = readEvents(projectRoot);
    const responses = readResponses(projectRoot);
    const responseMap = new Map();
    for (const r of responses) {
        responseMap.set(r.event_id, { action: r.action, detail: r.detail });
    }
    const ruleMetrics = new Map();
    for (const event of events) {
        const ruleId = event.rule_id ?? event.artifact_id ?? "unknown";
        const m = ruleMetrics.get(ruleId) ?? {
            fires: 0, fails: 0, warns: 0, resolved: 0,
            fixed: 0, deferred: 0, overridden: 0, false_positive: 0,
        };
        m.fires++;
        if (event.result === "fail")
            m.fails++;
        if (event.result === "warn")
            m.warns++;
        const response = responseMap.get(event.id);
        if (response) {
            m.resolved++;
            if (response.action === "fixed")
                m.fixed++;
            else if (response.action === "deferred")
                m.deferred++;
            else if (response.action === "overridden")
                m.overridden++;
            else if (response.action === "false-positive")
                m.false_positive++;
        }
        ruleMetrics.set(ruleId, m);
    }
    const alerts = [];
    for (const [ruleId, m] of ruleMetrics) {
        if (m.fires === 0)
            continue;
        const fpRate = m.false_positive / m.fires;
        const overrideRate = m.overridden / m.fires;
        const resolutionRate = m.resolved / (m.fails + m.warns || 1);
        if (fpRate > 0.3) {
            alerts.push(`${ruleId}: high false-positive rate (${(fpRate * 100).toFixed(0)}%) — review rule scope`);
        }
        if (overrideRate > 0.2) {
            alerts.push(`${ruleId}: high override rate (${(overrideRate * 100).toFixed(0)}%) — rule may be too restrictive`);
        }
        if (resolutionRate < 0.5 && m.fails + m.warns > 3) {
            alerts.push(`${ruleId}: low resolution rate (${(resolutionRate * 100).toFixed(0)}%) — enforcement may need escalation`);
        }
    }
    if (jsonOutput) {
        console.log(JSON.stringify({
            rules: Object.fromEntries(ruleMetrics),
            alerts,
        }, null, 2));
    }
    else {
        if (ruleMetrics.size === 0) {
            console.log("No enforcement metrics available. Run `orqa enforce` first.");
            return;
        }
        console.log("Enforcement Metrics");
        console.log("===================\n");
        for (const [ruleId, m] of [...ruleMetrics.entries()].sort((a, b) => b[1].fires - a[1].fires)) {
            const resRate = m.fails + m.warns > 0
                ? `${((m.resolved / (m.fails + m.warns)) * 100).toFixed(0)}%`
                : "n/a";
            console.log(`${ruleId}:`);
            console.log(`  Fires: ${m.fires}  Fails: ${m.fails}  Warns: ${m.warns}`);
            console.log(`  Resolved: ${m.resolved} (${resRate})  Fixed: ${m.fixed}  Overridden: ${m.overridden}  FP: ${m.false_positive}`);
        }
        if (alerts.length > 0) {
            console.log("\nAlerts:");
            for (const a of alerts) {
                console.log(`  ! ${a}`);
            }
        }
    }
}
// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------
/**
 * Find rule directories contributed by installed plugins.
 *
 * @param projectRoot - Absolute path to the project root.
 */
function findPluginRuleDirs(projectRoot) {
    const dirs = [];
    const pluginsDir = join(projectRoot, "plugins");
    if (!existsSync(pluginsDir))
        return dirs;
    for (const entry of readdirSync(pluginsDir, { withFileTypes: true })) {
        if (!entry.isDirectory())
            continue;
        const rulesDir = join(pluginsDir, entry.name, "rules");
        if (existsSync(rulesDir))
            dirs.push(rulesDir);
    }
    return dirs;
}
/**
 * Load a JSON approvals/pending file, returning an empty object on error.
 *
 * @param filePath - Absolute path to the JSON file.
 */
function loadApprovals(filePath) {
    try {
        if (!existsSync(filePath))
            return {};
        return JSON.parse(readFileSync(filePath, "utf-8"));
    }
    catch {
        return {};
    }
}
/**
 * Write a JSON approvals/pending file, creating parent directories as needed.
 *
 * @param filePath - Absolute path to the JSON file.
 * @param data - The data to write.
 */
function writeApprovals(filePath, data) {
    const dir = dirname(filePath);
    if (!existsSync(dir))
        mkdirSync(dir, { recursive: true });
    writeFileSync(filePath, JSON.stringify(data, null, 2) + "\n", "utf-8");
}
/**
 * Extract a named flag value from an args array (e.g. --flag value).
 *
 * @param args - The argument array to search.
 * @param flag - The flag name (e.g. "--event-id").
 */
function getFlag(args, flag) {
    const idx = args.indexOf(flag);
    if (idx === -1 || idx + 1 >= args.length)
        return undefined;
    return args[idx + 1];
}
//# sourceMappingURL=enforce.js.map