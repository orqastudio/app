/**
 * Debug commands — dev environment + debug tooling.
 *
 * orqa debug                Start the full dev environment (Vite + Tauri)
 * orqa debug stop           Stop gracefully
 * orqa debug kill           Force-kill all processes
 * orqa debug restart        Restart Vite + Tauri (not the controller)
 * orqa debug restart-tauri  Restart Tauri only
 * orqa debug restart-vite   Restart Vite only
 * orqa debug status         Show process status
 * orqa debug icons          Generate brand icons from SVG sources
 * orqa debug tool           Run the debug-tool submodule
 */
import { execSync } from "node:child_process";
import * as path from "node:path";
import * as fs from "node:fs";
import { getRoot } from "../lib/root.js";
const USAGE = `
Usage: orqa debug [subcommand]

Subcommands:
  (none)            Start the full dev environment (Vite + Tauri)
  stop              Stop gracefully
  kill              Force-kill all processes
  restart           Restart Vite + Tauri (not the controller)
  restart-tauri     Restart Tauri only
  restart-vite      Restart Vite only
  status            Show process status
  icons [--deploy]  Generate brand icons from SVG sources
  tool [args...]    Run the debug-tool submodule
`.trim();
export async function runDebugCommand(args) {
    if (args[0] === "--help" || args[0] === "-h") {
        console.log(USAGE);
        return;
    }
    const root = getRoot();
    const sub = args[0] ?? "dev";
    // Debug tool subcommand — delegates to debug-tool submodule
    if (sub === "tool") {
        await cmdDebugTool(root, args.slice(1));
        return;
    }
    // Icons command — runs brand icon generator
    if (sub === "icons") {
        const brandScript = path.join(root, "libs/brand/scripts/generate-icons.mjs");
        if (!fs.existsSync(brandScript)) {
            console.error("Brand icon script not found. Are you in the dev repo root?");
            process.exit(1);
        }
        const iconArgs = args.slice(1).join(" ");
        try {
            execSync(`node "${brandScript}" ${iconArgs}`, {
                cwd: path.join(root, "libs/brand"),
                stdio: "inherit",
            });
        }
        catch {
            process.exit(1);
        }
        return;
    }
    // All other commands delegate to the dev controller
    const appDir = path.join(root, "app");
    const devScript = path.join(root, "tools/debug/dev.mjs");
    if (!fs.existsSync(devScript)) {
        console.error("Dev script not found. Are you in the dev repo root?");
        process.exit(1);
    }
    // Start the validation daemon and refresh plugin content before starting dev.
    if (sub === "dev") {
        // Refresh plugin content so .orqa/ is in sync with plugin source
        try {
            const { runPluginCommand } = await import("./plugin.js");
            await runPluginCommand(["refresh"]);
        }
        catch {
            // Non-fatal — content may be stale but dev can still start
        }
        const { runDaemonCommand } = await import("./daemon.js");
        await runDaemonCommand(["start"]).catch(() => {
            // Daemon may already be running or binary not built — non-fatal.
        });
    }
    try {
        execSync(`node ${devScript} ${sub}`, { cwd: appDir, stdio: "inherit" });
    }
    catch {
        // Dev server exits with non-zero on stop/kill — expected
    }
}
async function cmdDebugTool(root, args) {
    const debugToolPaths = [
        path.join(root, "debug-tool", "debug-tool.sh"),
        path.join(root, "node_modules", ".bin", "orqa-debug"),
    ];
    let debugToolPath = null;
    for (const p of debugToolPaths) {
        if (fs.existsSync(p)) {
            debugToolPath = p;
            break;
        }
    }
    if (!debugToolPath) {
        console.error("Debug tool not found. Ensure debug-tool submodule is initialized.");
        process.exit(1);
    }
    const cmd = `"${debugToolPath}" ${args.join(" ")}`;
    try {
        execSync(cmd, { encoding: "utf-8", stdio: "inherit" });
    }
    catch {
        process.exit(1);
    }
}
//# sourceMappingURL=debug.js.map