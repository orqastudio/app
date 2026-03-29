/**
 * Build command — production builds for the OrqaStudio app.
 *
 * orqa build            Build the full app (Tauri production bundle)
 * orqa build rust       Build only the Rust backend
 * orqa build app        Build only the frontend
 */

import { execSync } from "node:child_process";
import * as fs from "node:fs";
import * as path from "node:path";
import { getRoot } from "../lib/root.js";

const USAGE = `
Usage: orqa build [target]

Build the OrqaStudio app for production.

Targets:
  (none)    Full production build (Tauri bundles frontend + backend)
  rust      Build only the Rust backend (release mode)
  app       Build only the frontend (Vite production build)

Options:
  --help, -h   Show this help message
`.trim();

interface BuildTarget {
	name: string;
	key: string;
	dir: string;
	command: string;
}

function getTargets(root: string): BuildTarget[] {
	return [
		{
			name: "Full Tauri production build",
			key: "full",
			dir: path.join(root, "app"),
			command: "npx tauri build",
		},
		{
			name: "Rust backend (release)",
			key: "rust",
			dir: path.join(root, "app/src-tauri"),
			command: "cargo build --release",
		},
		{
			name: "Frontend (Vite)",
			key: "app",
			dir: path.join(root, "app"),
			command: "npx vite build",
		},
	];
}

/**
 * Dispatch the build command: full app, Rust only, or frontend only.
 * @param args - CLI arguments after "build".
 */
export async function runBuildCommand(args: string[]): Promise<void> {
	if (args[0] === "--help" || args[0] === "-h") {
		console.log(USAGE);
		return;
	}

	const root = getRoot();
	const target = args[0];
	const targets = getTargets(root);

	if (!target) {
		// Default: full Tauri build
		const full = targets[0];
		runBuild(full, root);
		return;
	}

	const match = targets.find((t) => t.key === target);
	if (!match) {
		console.error(`Unknown build target: ${target}`);
		console.error(USAGE);
		process.exit(1);
	}

	runBuild(match, root);
}

function runBuild(target: BuildTarget, root: string): void {
	if (!fs.existsSync(target.dir)) {
		console.error(`Build directory not found: ${path.relative(root, target.dir)}`);
		process.exit(1);
	}

	console.log(`=== ${target.name} ===`);
	try {
		execSync(target.command, { cwd: target.dir, stdio: "inherit" });
	} catch {
		process.exit(1);
	}
}
