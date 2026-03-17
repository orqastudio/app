/**
 * Validation command — wraps @orqastudio/integrity-validator.
 *
 * orqa validate [path]
 */

import { execSync } from "node:child_process";
import * as path from "node:path";

const USAGE = `
Usage: orqa validate [path]

Run integrity validation on the specified path (defaults to current directory).
Wraps @orqastudio/integrity-validator for standalone CLI use.

Options:
  --json       Output results as JSON
  --help, -h   Show this help message
`.trim();

export async function runValidateCommand(args: string[]): Promise<void> {
	if (args.includes("--help") || args.includes("-h")) {
		console.log(USAGE);
		return;
	}

	const targetPath = args.find((a) => !a.startsWith("--")) ?? process.cwd();
	const jsonOutput = args.includes("--json");

	// Try to find the integrity validator
	const validatorPaths = [
		path.join(process.cwd(), "node_modules", ".bin", "orqa-integrity"),
		path.join(process.cwd(), "libs", "integrity-validator", "dist", "cli.js"),
	];

	let validatorPath: string | null = null;
	for (const p of validatorPaths) {
		try {
			const { existsSync } = await import("node:fs");
			if (existsSync(p)) {
				validatorPath = p;
				break;
			}
		} catch {
			continue;
		}
	}

	if (!validatorPath) {
		// Fall back to npx
		const cmd = `npx @orqastudio/integrity-validator ${targetPath}${jsonOutput ? " --json" : ""}`;
		try {
			const output = execSync(cmd, { encoding: "utf-8", stdio: "pipe" });
			process.stdout.write(output);
		} catch (err: unknown) {
			if (err && typeof err === "object" && "stdout" in err) {
				process.stdout.write(String((err as { stdout: string }).stdout));
			}
			process.exit(1);
		}
		return;
	}

	// Run the validator directly
	const cmd = `node "${validatorPath}" ${targetPath}${jsonOutput ? " --json" : ""}`;
	try {
		const output = execSync(cmd, { encoding: "utf-8", stdio: "pipe" });
		process.stdout.write(output);
	} catch (err: unknown) {
		if (err && typeof err === "object" && "stdout" in err) {
			process.stdout.write(String((err as { stdout: string }).stdout));
		}
		process.exit(1);
	}
}
