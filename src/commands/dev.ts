/**
 * Dev environment commands — delegates to the debug controller.
 *
 * orqa dev                Start the full dev environment (Vite + Tauri)
 * orqa dev stop           Stop gracefully
 * orqa dev kill           Force-kill all processes
 * orqa dev restart        Restart Vite + Tauri (not the controller)
 * orqa dev restart-tauri  Restart Tauri only
 * orqa dev restart-vite   Restart Vite only
 * orqa dev status         Show process status
 */

import { execSync } from "node:child_process";
import * as fs from "node:fs";
import * as path from "node:path";

const USAGE = `
Usage: orqa dev [subcommand]

Subcommands:
  (none)          Start the full dev environment (Vite + Tauri)
  stop            Stop gracefully
  kill            Force-kill all processes
  restart         Restart Vite + Tauri (not the controller)
  restart-tauri   Restart Tauri only
  restart-vite    Restart Vite only
  status          Show process status
`.trim();

export async function runDevCommand(args: string[]): Promise<void> {
	if (args[0] === "--help" || args[0] === "-h") {
		console.log(USAGE);
		return;
	}

	const root = process.cwd();
	const appDir = path.join(root, "app");
	const devScript = path.join(root, "tools/debug/dev.mjs");

	if (!fs.existsSync(devScript)) {
		console.error("Dev script not found. Are you in the dev repo root?");
		process.exit(1);
	}

	const sub = args[0] ?? "dev";

	try {
		execSync(`node ${devScript} ${sub}`, { cwd: appDir, stdio: "inherit" });
	} catch {
		// Dev server exits with non-zero on stop/kill — expected
	}
}
