/**
 * Shared access to the orqa-validation Rust binary / daemon.
 *
 * Both `orqa check` and `orqa enforce` use the same underlying validation
 * engine (libs/validation/).  This module provides the common helpers:
 *
 *   findBinary()     — locate the compiled orqa-validation binary
 *   callDaemon()     — POST to the running daemon's /validate endpoint
 *   runRustBinary()  — exec the binary directly and capture JSON output
 *   runValidation()  — daemon-first, binary-fallback orchestration
 */

import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export interface ValidationCheck {
	category: string;
	severity: string;
	artifact_id: string;
	message: string;
}

export interface AppliedFix {
	artifact_id: string;
	description: string;
}

export interface EnforcementEvent {
	mechanism: string;
	check_type: string;
	rule_id: string | null;
	artifact_id: string | null;
	result: string;
	message: string;
}

export interface ValidationReport {
	checks: ValidationCheck[];
	health: Record<string, unknown> | null;
	fixes_applied: AppliedFix[];
	enforcement_events: EnforcementEvent[];
}

// ---------------------------------------------------------------------------
// Binary discovery
// ---------------------------------------------------------------------------

/**
 * Find the Rust validation binary. Checks common build locations relative to
 * the project root, including workspace-level and crate-level target dirs.
 */
export function findBinary(projectRoot: string): string | null {
	const candidates = [
		join(projectRoot, "libs", "validation", "target", "release", "orqa-validation"),
		join(projectRoot, "libs", "validation", "target", "release", "orqa-validation.exe"),
		join(projectRoot, "libs", "validation", "target", "debug", "orqa-validation"),
		join(projectRoot, "libs", "validation", "target", "debug", "orqa-validation.exe"),
		join(projectRoot, "target", "release", "orqa-validation"),
		join(projectRoot, "target", "release", "orqa-validation.exe"),
		join(projectRoot, "target", "debug", "orqa-validation"),
		join(projectRoot, "target", "debug", "orqa-validation.exe"),
		join(projectRoot, "app", "backend", "target", "release", "orqa-validation"),
		join(projectRoot, "app", "backend", "target", "release", "orqa-validation.exe"),
		join(projectRoot, "app", "backend", "target", "debug", "orqa-validation"),
		join(projectRoot, "app", "backend", "target", "debug", "orqa-validation.exe"),
	];
	for (const c of candidates) {
		if (existsSync(c)) return c;
	}
	return null;
}

// ---------------------------------------------------------------------------
// Daemon communication
// ---------------------------------------------------------------------------

const DAEMON_PORT = 10258;

/**
 * Call the running daemon's /validate endpoint.
 * Returns the raw JSON response string, or null if the daemon is unreachable.
 */
export async function callDaemon(
	targetPath: string,
	autoFix: boolean,
): Promise<string | null> {
	try {
		const controller = new AbortController();
		const timeout = setTimeout(() => controller.abort(), 500);
		try {
			const response = await fetch(
				`http://127.0.0.1:${DAEMON_PORT}/validate`,
				{
					method: "POST",
					headers: { "Content-Type": "application/json" },
					body: JSON.stringify({ path: targetPath, fix: autoFix }),
					signal: controller.signal,
				},
			);
			if (!response.ok) return null;
			return await response.text();
		} finally {
			clearTimeout(timeout);
		}
	} catch {
		return null;
	}
}

// ---------------------------------------------------------------------------
// Direct binary execution
// ---------------------------------------------------------------------------

/**
 * Run the Rust validator binary directly and capture its JSON output.
 */
export function runRustBinary(
	binaryPath: string,
	targetPath: string,
	autoFix: boolean,
): { exitCode: number; output: string } {
	const args = [targetPath];
	if (autoFix) args.push("--fix");

	try {
		const output = execFileSync(binaryPath, args, {
			encoding: "utf-8",
			timeout: 60000,
			windowsHide: true,
		});
		return { exitCode: 0, output };
	} catch (e: unknown) {
		const err = e as { status?: number; stdout?: string; stderr?: string };
		return {
			exitCode: err.status ?? 2,
			output: err.stdout ?? err.stderr ?? String(e),
		};
	}
}

// ---------------------------------------------------------------------------
// High-level orchestration
// ---------------------------------------------------------------------------

/**
 * Run validation: daemon-first, binary-fallback.
 *
 * Returns a parsed `ValidationReport` and the process exit code.
 * Throws if neither the daemon nor the binary are available.
 */
export async function runValidation(
	projectRoot: string,
	targetPath: string,
	autoFix = false,
): Promise<{ report: ValidationReport; exitCode: number }> {
	// Try daemon first (low-latency, keeps graph in memory).
	const daemonOutput = await callDaemon(targetPath, autoFix);

	const { exitCode, output } =
		daemonOutput !== null
			? { exitCode: 0, output: daemonOutput }
			: (() => {
					const binary = findBinary(projectRoot);
					if (binary === null) {
						throw new Error(
							"orqa-validation binary not found and daemon is not running.\n" +
								"Build with: cargo build --manifest-path libs/validation/Cargo.toml --release\n" +
								"Or start the daemon with: orqa daemon start",
						);
					}
					return runRustBinary(binary, targetPath, autoFix);
				})();

	let parsed: Partial<ValidationReport>;
	try {
		parsed = JSON.parse(output);
	} catch {
		// Binary produced non-JSON output (e.g. a crash). Return it as a single
		// error check so the caller can display it.
		return {
			report: {
				checks: [
					{
						category: "RuntimeError",
						severity: "Error",
						artifact_id: "",
						message: output.trim(),
					},
				],
				health: null,
				fixes_applied: [],
				enforcement_events: [],
			},
			exitCode: exitCode || 2,
		};
	}

	return {
		report: {
			checks: parsed.checks ?? [],
			health: parsed.health ?? null,
			fixes_applied: parsed.fixes_applied ?? [],
			enforcement_events: parsed.enforcement_events ?? [],
		},
		exitCode,
	};
}

// ---------------------------------------------------------------------------
// Text formatting
// ---------------------------------------------------------------------------

/**
 * Format a validation report as human-readable text output.
 * Returns the formatted string and counts of errors/warnings.
 */
export function formatReport(report: ValidationReport): {
	text: string;
	errors: number;
	warnings: number;
} {
	const lines: string[] = [];
	const checks = report.checks;

	if (checks.length === 0) {
		return {
			text: "All validation checks passed. 0 errors, 0 warnings.",
			errors: 0,
			warnings: 0,
		};
	}

	const byCategory = new Map<string, ValidationCheck[]>();
	for (const c of checks) {
		const list = byCategory.get(c.category) ?? [];
		list.push(c);
		byCategory.set(c.category, list);
	}

	for (const [category, findings] of byCategory) {
		lines.push(`\n${category} (${findings.length}):`);
		for (const f of findings) {
			const icon =
				f.severity === "Error" || f.severity === "error" ? "E" : "W";
			lines.push(`  [${icon}] ${f.artifact_id}: ${f.message}`);
		}
	}

	const errors = checks.filter(
		(c) => c.severity === "Error" || c.severity === "error",
	).length;
	const warnings = checks.length - errors;
	lines.push(`\n${errors} error(s), ${warnings} warning(s).`);

	if (report.fixes_applied.length > 0) {
		lines.push(`Auto-fixed ${report.fixes_applied.length} issue(s).`);
	}

	return { text: lines.join("\n"), errors, warnings };
}
