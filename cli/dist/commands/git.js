/**
 * Git commands — monorepo-aware git operations.
 *
 * orqa git status    Show which components have changes
 * orqa git pr        Create a pull request on the local git server
 * orqa git sync      Push to all remotes
 * orqa git audit     Check git infrastructure health
 * orqa git license   Audit LICENSE files across all repos
 * orqa git readme    Audit README.md files across all repos
 * orqa git hosting   Local git server management
 */
import { execSync } from "node:child_process";
import * as fs from "node:fs";
import * as path from "node:path";
import { getRoot } from "../lib/root.js";
const USAGE = `
Usage: orqa git <subcommand>

Subcommands:
  status    Show which components have changes
  pr        Create a pull request on the local git server
  sync      Push to all remotes
  audit     Check git infrastructure health
  license   Audit LICENSE files across all repos (--json)
  readme    Audit README.md files across all repos (--json)
  hosting   Local git server management (up, down, setup, status, logs, push, mirror)

Options:
  --help, -h  Show this help message
`.trim();
const PR_USAGE = `
Usage: orqa git pr [options]

Create a pull request on the local git server.

Options:
  -t, --title <title>   PR title (default: branch name)
  -b, --body <body>     PR body / description
  --help, -h            Show this help message
`.trim();
/** Top-level directories that contain components. */
const COMPONENT_ROOTS = [
    "libs",
    "plugins",
    "connectors",
    "sidecars",
    "app",
    "tools",
    "templates",
    "registry",
];
// ── Helpers ──────────────────────────────────────────────────────────
function git(args, root) {
    return execSync(`git ${args}`, { cwd: root, encoding: "utf-8" }).trim();
}
function gitSilent(args, root) {
    try {
        return execSync(`git ${args}`, {
            cwd: root,
            encoding: "utf-8",
            stdio: ["pipe", "pipe", "pipe"],
        }).trim();
    }
    catch {
        return null;
    }
}
/**
 * Discover component directories that actually exist on disk.
 * A "component" is a direct child of one of the COMPONENT_ROOTS.
 * `app` itself counts as a component if it has no children, otherwise
 * each child directory under `app/` is a component.
 * @param root - Absolute path to the project root.
 * @returns Array of relative component paths.
 */
function discoverComponents(root) {
    const components = [];
    for (const cr of COMPONENT_ROOTS) {
        const dirPath = path.join(root, cr);
        if (!fs.existsSync(dirPath))
            continue;
        // Special case: `app` may be a single component or contain sub-components
        if (cr === "app") {
            const children = safeReaddir(dirPath);
            if (children.length === 0) {
                components.push("app");
            }
            else {
                for (const child of children) {
                    if (child.startsWith("."))
                        continue;
                    const childPath = path.join(dirPath, child);
                    if (fs.statSync(childPath).isDirectory()) {
                        components.push(`app/${child}`);
                    }
                }
                // Also treat app itself as a component for root-level app files
                components.push("app");
            }
            continue;
        }
        const children = safeReaddir(dirPath);
        for (const child of children) {
            if (child.startsWith("."))
                continue;
            const childPath = path.join(dirPath, child);
            if (fs.statSync(childPath).isDirectory()) {
                components.push(`${cr}/${child}`);
            }
        }
    }
    return components.sort();
}
function safeReaddir(dir) {
    try {
        return fs.readdirSync(dir);
    }
    catch {
        return [];
    }
}
/**
 * Given a file path from `git status --porcelain`, find which component
 * it belongs to (or "(root)" / ".orqa" for governance files).
 * @param filePath - Relative file path from git status output.
 * @param components - Array of known component paths.
 * @returns The component name or category string.
 */
function classifyFile(filePath, components) {
    // Longest-prefix match against known components
    for (const comp of components.sort((a, b) => b.length - a.length)) {
        if (filePath.startsWith(comp + "/") || filePath === comp) {
            return comp;
        }
    }
    // Governance files
    if (filePath.startsWith(".orqa/"))
        return ".orqa";
    return "(root)";
}
function parseStatusPorcelain(output) {
    const changes = [];
    for (const line of output.split("\n")) {
        if (!line || line.length < 4)
            continue;
        const index = line[0];
        const worktree = line[1];
        // Porcelain format: XY <space> filepath  (or XY <space> from -> to for renames)
        let filePath = line.slice(3);
        // Handle renames: "R  old -> new" — use the destination path
        const arrowIdx = filePath.indexOf(" -> ");
        if (arrowIdx !== -1) {
            filePath = filePath.slice(arrowIdx + 4);
        }
        // Normalize path separators
        filePath = filePath.replace(/\\/g, "/");
        changes.push({ index, worktree, filePath });
    }
    return changes;
}
// ── Subcommands ──────────────────────────────────────────────────────
async function cmdStatus(root) {
    const porcelain = gitSilent("status --porcelain", root);
    if (!porcelain) {
        console.log("Working tree is clean. No changes in any component.");
        return;
    }
    const components = discoverComponents(root);
    const changes = parseStatusPorcelain(porcelain);
    // Group by component
    const grouped = new Map();
    for (const change of changes) {
        const comp = classifyFile(change.filePath, components);
        const summary = grouped.get(comp) ?? { staged: 0, modified: 0, untracked: 0 };
        if (change.index === "?" && change.worktree === "?") {
            summary.untracked++;
        }
        else {
            // Index column: staged changes (anything other than ' ' or '?')
            if (change.index !== " " && change.index !== "?") {
                summary.staged++;
            }
            // Worktree column: unstaged modifications (anything other than ' ' or '?')
            if (change.worktree !== " " && change.worktree !== "?") {
                summary.modified++;
            }
        }
        grouped.set(comp, summary);
    }
    if (grouped.size === 0) {
        console.log("Working tree is clean. No changes in any component.");
        return;
    }
    console.log("Components with changes:");
    const sortedEntries = [...grouped.entries()].sort(([a], [b]) => a.localeCompare(b));
    for (const [comp, summary] of sortedEntries) {
        const parts = [];
        if (summary.modified > 0)
            parts.push(`${summary.modified} modified`);
        if (summary.staged > 0)
            parts.push(`${summary.staged} staged`);
        if (summary.untracked > 0)
            parts.push(`${summary.untracked} untracked`);
        console.log(`  ${comp.padEnd(30)} ${parts.join(", ")}`);
    }
    const changedCount = grouped.size;
    const totalCount = components.length;
    const cleanCount = totalCount - changedCount;
    // Subtract non-component entries from clean count
    const nonComponentEntries = [...grouped.keys()].filter((k) => k === "(root)" || k === ".orqa").length;
    console.log(`\nClean: ${cleanCount + nonComponentEntries} components`);
}
async function cmdPr(root, args) {
    if (args.includes("--help") || args.includes("-h")) {
        console.log(PR_USAGE);
        return;
    }
    // Parse flags
    let title = null;
    let body = null;
    for (let i = 0; i < args.length; i++) {
        const arg = args[i];
        if ((arg === "-t" || arg === "--title") && i + 1 < args.length) {
            title = args[++i];
        }
        else if ((arg === "-b" || arg === "--body") && i + 1 < args.length) {
            body = args[++i];
        }
    }
    // Get current branch
    const branch = git("rev-parse --abbrev-ref HEAD", root);
    if (branch === "main") {
        console.error("Cannot create a PR from the main branch. Switch to a feature branch first.");
        process.exit(1);
    }
    // Use branch name as default title
    if (!title) {
        title = branch.replace(/[/_-]/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
    }
    // Push to local
    console.log(`Pushing branch '${branch}' to local...`);
    try {
        execSync(`git push local ${branch}`, { cwd: root, stdio: "inherit" });
    }
    catch {
        console.error("Failed to push to local. Is the remote configured and the server running?");
        process.exit(1);
    }
    // Get the local remote URL to derive the API base
    const remoteUrl = gitSilent("remote get-url local", root);
    if (!remoteUrl) {
        console.error("Could not determine local remote URL.");
        process.exit(1);
    }
    // Derive API base URL from the remote.
    // Formats: http://localhost:3000/org/repo.git  or  ssh://git@localhost:3000/org/repo.git
    const apiBase = deriveApiBase(remoteUrl);
    if (!apiBase) {
        console.error(`Could not derive API URL from remote: ${remoteUrl}`);
        process.exit(1);
    }
    // Create PR via Forgejo API
    const payload = JSON.stringify({
        title,
        body: body ?? "",
        head: branch,
        base: "main",
    });
    console.log(`Creating PR: "${title}"...`);
    try {
        const result = execSync(`curl -s -X POST "${apiBase}/pulls" -H "Content-Type: application/json" -d '${payload.replace(/'/g, "'\\''")}'`, { cwd: root, encoding: "utf-8" });
        const parsed = JSON.parse(result);
        if (parsed.html_url) {
            console.log(`\nPR created: ${parsed.html_url}`);
        }
        else if (parsed.message) {
            console.error(`Forgejo API error: ${parsed.message}`);
            process.exit(1);
        }
        else {
            console.log("PR created. Response:");
            console.log(result);
        }
    }
    catch (err) {
        console.error("Failed to create PR via Forgejo API.");
        console.error(err instanceof Error ? err.message : err);
        process.exit(1);
    }
}
/**
 * Derive the Forgejo API repo endpoint from a remote URL.
 *
 * Examples:
 *   http://localhost:3000/orqa/orqastudio.git  => http://localhost:3000/api/v1/repos/orqa/orqastudio
 *   ssh://git@localhost:3000/orqa/orqastudio.git => http://localhost:3000/api/v1/repos/orqa/orqastudio
 * @param remoteUrl - The git remote URL.
 * @returns The API base URL, or null if the URL is unrecognised.
 */
function deriveApiBase(remoteUrl) {
    // HTTP(S) URL
    const httpMatch = remoteUrl.match(/^(https?:\/\/[^/]+)\/([^/]+)\/([^/]+?)(?:\.git)?$/);
    if (httpMatch) {
        return `${httpMatch[1]}/api/v1/repos/${httpMatch[2]}/${httpMatch[3]}`;
    }
    // SSH URL: ssh://git@host:port/org/repo.git or git@host:org/repo.git
    const sshMatch = remoteUrl.match(/^ssh:\/\/[^@]+@([^/]+)\/([^/]+)\/([^/]+?)(?:\.git)?$/);
    if (sshMatch) {
        const host = sshMatch[1]; // may include :port
        return `http://${host}/api/v1/repos/${sshMatch[2]}/${sshMatch[3]}`;
    }
    const scpMatch = remoteUrl.match(/^[^@]+@([^:]+):([^/]+)\/([^/]+?)(?:\.git)?$/);
    if (scpMatch) {
        return `http://${scpMatch[1]}/api/v1/repos/${scpMatch[2]}/${scpMatch[3]}`;
    }
    return null;
}
async function cmdSync(root) {
    const remotes = [
        { name: "origin", label: "GitHub" },
        { name: "local", label: "Local server" },
    ];
    // Get current branch
    const branch = git("rev-parse --abbrev-ref HEAD", root);
    let allOk = true;
    for (const remote of remotes) {
        // Verify remote exists
        const url = gitSilent(`remote get-url ${remote.name}`, root);
        if (!url) {
            console.log(`  ${remote.label} (${remote.name}): skipped — remote not configured`);
            continue;
        }
        process.stdout.write(`  ${remote.label} (${remote.name}): pushing ${branch}... `);
        try {
            execSync(`git push ${remote.name} ${branch}`, {
                cwd: root,
                stdio: ["pipe", "pipe", "pipe"],
            });
            console.log("ok");
        }
        catch (err) {
            console.log("FAILED");
            if (err instanceof Error && "stderr" in err) {
                const stderr = err.stderr;
                console.error(`    ${stderr.toString().trim()}`);
            }
            allOk = false;
        }
    }
    if (!allOk) {
        process.exit(1);
    }
    console.log("\nAll remotes synced.");
}
async function cmdAudit(root) {
    let issues = 0;
    // Check remotes
    const remotes = [
        { name: "origin", label: "GitHub" },
        { name: "local", label: "Local server" },
    ];
    console.log("Remote configuration:");
    for (const remote of remotes) {
        const url = gitSilent(`remote get-url ${remote.name}`, root);
        if (!url) {
            console.log(`  ${remote.label} (${remote.name}): NOT CONFIGURED`);
            issues++;
            continue;
        }
        console.log(`  ${remote.label} (${remote.name}): ${url}`);
        // Reachability check
        const reachable = gitSilent(`ls-remote --exit-code ${remote.name}`, root);
        if (reachable === null) {
            console.log(`    reachability: UNREACHABLE`);
            issues++;
        }
        else {
            console.log(`    reachability: ok`);
        }
    }
    // Check uncommitted changes
    console.log("\nWorking tree:");
    const porcelain = gitSilent("status --porcelain", root);
    if (!porcelain) {
        console.log("  Clean — no uncommitted changes");
    }
    else {
        const lineCount = porcelain.split("\n").filter(Boolean).length;
        console.log(`  ${lineCount} uncommitted change(s)`);
        issues++;
    }
    // Check stashes
    const stashList = gitSilent("stash list", root);
    if (stashList) {
        const stashCount = stashList.split("\n").filter(Boolean).length;
        console.log(`  ${stashCount} stash(es) — stashes should be committed or dropped`);
        issues++;
    }
    else {
        console.log("  No stashes");
    }
    // Check branch sync with remotes
    console.log("\nBranch sync (main):");
    for (const remote of remotes) {
        const url = gitSilent(`remote get-url ${remote.name}`, root);
        if (!url)
            continue;
        // Fetch latest refs (quiet)
        gitSilent(`fetch ${remote.name} main --quiet`, root);
        const localRef = gitSilent("rev-parse main", root);
        const remoteRef = gitSilent(`rev-parse ${remote.name}/main`, root);
        if (!localRef || !remoteRef) {
            console.log(`  ${remote.label}: could not compare (missing ref)`);
            issues++;
            continue;
        }
        if (localRef === remoteRef) {
            console.log(`  ${remote.label}: up to date`);
        }
        else {
            // Check ahead/behind
            const aheadBehind = gitSilent(`rev-list --left-right --count main...${remote.name}/main`, root);
            if (aheadBehind) {
                const [ahead, behind] = aheadBehind.split(/\s+/);
                console.log(`  ${remote.label}: ${ahead} ahead, ${behind} behind`);
            }
            else {
                console.log(`  ${remote.label}: out of sync`);
            }
            issues++;
        }
    }
    // Summary
    console.log();
    if (issues === 0) {
        console.log("All checks passed.");
    }
    else {
        console.log(`${issues} issue(s) found.`);
    }
}
// ── Entry point ──────────────────────────────────────────────────────
/**
 * Dispatch the git command: status, log, diff, pr subcommands.
 * @param args - CLI arguments after "git".
 */
export async function runGitCommand(args) {
    const subcommand = args[0];
    if (!subcommand || subcommand === "--help" || subcommand === "-h") {
        console.log(USAGE);
        return;
    }
    const root = getRoot();
    switch (subcommand) {
        case "status":
            await cmdStatus(root);
            break;
        case "pr":
            await cmdPr(root, args.slice(1));
            break;
        case "sync":
            await cmdSync(root);
            break;
        case "audit":
            await cmdAudit(root);
            break;
        case "license":
        case "readme": {
            const { runRepoCommand } = await import("./repo.js");
            await runRepoCommand([subcommand, ...args.slice(1)]);
            break;
        }
        case "hosting": {
            const { runHostingCommand } = await import("./hosting.js");
            await runHostingCommand(args.slice(1));
            break;
        }
        default:
            console.error(`Unknown git subcommand: ${subcommand}`);
            console.error("Run 'orqa git --help' for available subcommands.");
            process.exit(1);
    }
}
//# sourceMappingURL=git.js.map