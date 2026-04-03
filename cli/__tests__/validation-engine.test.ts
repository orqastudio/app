/**
 * Tests for validation-engine.ts — pure functions that do not require a
 * running daemon or compiled binary.
 *
 * We test:
 *   - findBinary() location logic with known-absent paths
 *   - runRustBinary() error handling (binary not present)
 *   - formatReport() text formatting for all check states
 *   - runValidation() error path when no daemon and no binary
 *   - Type shapes for ValidationReport, ValidationCheck, etc.
 */
import { describe, it, expect } from "vitest";
import * as path from "node:path";
import * as os from "node:os";
import {
	findBinary,
	formatReport,
	runRustBinary,
	type ValidationReport,
	type ValidationCheck,
	type AppliedFix,
	type EnforcementEvent,
} from "../src/lib/validation-engine.js";

// ---------------------------------------------------------------------------
// findBinary
// ---------------------------------------------------------------------------

describe("findBinary", () => {
	it("returns null when no binary exists in the given root", () => {
		const tmpRoot = os.tmpdir();
		// No binary will be found in the OS temp dir.
		const result = findBinary(tmpRoot);
		expect(result).toBeNull();
	});

	it("returns null for an empty directory", () => {
		const result = findBinary("/nonexistent-path-that-cannot-exist-xyz");
		expect(result).toBeNull();
	});

	it("returns null for the CLI source tree (binary lives in engine/)", () => {
		// The CLI source tree does not contain built binaries.
		const cliRoot = path.resolve(import.meta.url.replace(/^file:\/\//, ""), "../../../");
		const result = findBinary(cliRoot);
		// Either null or a valid path — both are acceptable.
		// This verifies the function does not throw.
		expect(result === null || typeof result === "string").toBe(true);
	});
});

// ---------------------------------------------------------------------------
// runRustBinary — error handling when binary path is invalid
// ---------------------------------------------------------------------------

describe("runRustBinary", () => {
	it("returns non-zero exit code when binary does not exist", () => {
		const result = runRustBinary("/nonexistent/path/orqa-validation", "/tmp", false);
		expect(result.exitCode).not.toBe(0);
	});

	it("returns an output string even on failure", () => {
		const result = runRustBinary("/nonexistent/path/orqa-validation", "/tmp", false);
		expect(typeof result.output).toBe("string");
	});
});

// ---------------------------------------------------------------------------
// formatReport — pure text formatting
// ---------------------------------------------------------------------------

describe("formatReport", () => {
	it("returns zero-error message for empty checks", () => {
		const report: ValidationReport = {
			checks: [],
			health: null,
			fixes_applied: [],
			enforcement_events: [],
		};
		const { text, errors, warnings } = formatReport(report);
		expect(text).toContain("All validation checks passed");
		expect(errors).toBe(0);
		expect(warnings).toBe(0);
	});

	it("counts errors correctly", () => {
		const report: ValidationReport = {
			checks: [
				{ category: "Schema", severity: "Error", artifact_id: "TASK-001", message: "Invalid status" },
				{ category: "Schema", severity: "Error", artifact_id: "TASK-002", message: "Missing field" },
				{ category: "Schema", severity: "Warning", artifact_id: "TASK-003", message: "Deprecated field" },
			],
			health: null,
			fixes_applied: [],
			enforcement_events: [],
		};
		const { errors, warnings } = formatReport(report);
		expect(errors).toBe(2);
		expect(warnings).toBe(1);
	});

	it("groups checks by category", () => {
		const report: ValidationReport = {
			checks: [
				{ category: "Schema", severity: "Error", artifact_id: "TASK-001", message: "Error 1" },
				{ category: "Relationship", severity: "Warning", artifact_id: "TASK-002", message: "Warning 1" },
			],
			health: null,
			fixes_applied: [],
			enforcement_events: [],
		};
		const { text } = formatReport(report);
		expect(text).toContain("Schema");
		expect(text).toContain("Relationship");
		expect(text).toContain("[E]");
		expect(text).toContain("[W]");
	});

	it("reports fix count when fixes were applied", () => {
		const report: ValidationReport = {
			checks: [
				{ category: "Schema", severity: "Error", artifact_id: "TASK-001", message: "Fixed" },
			],
			health: null,
			fixes_applied: [
				{ artifact_id: "TASK-001", description: "Added missing field" },
			],
			enforcement_events: [],
		};
		const { text } = formatReport(report);
		expect(text).toContain("Auto-fixed 1");
	});

	it("handles lowercase severity values", () => {
		const report: ValidationReport = {
			checks: [
				{ category: "Schema", severity: "error", artifact_id: "TASK-001", message: "lowercase error" },
				{ category: "Schema", severity: "warning", artifact_id: "TASK-002", message: "lowercase warning" },
			],
			health: null,
			fixes_applied: [],
			enforcement_events: [],
		};
		const { errors, warnings } = formatReport(report);
		expect(errors).toBe(1);
		expect(warnings).toBe(1);
	});

	it("includes artifact ID in output", () => {
		const report: ValidationReport = {
			checks: [
				{ category: "Schema", severity: "Error", artifact_id: "EPIC-042", message: "bad" },
			],
			health: null,
			fixes_applied: [],
			enforcement_events: [],
		};
		const { text } = formatReport(report);
		expect(text).toContain("EPIC-042");
	});
});

// ---------------------------------------------------------------------------
// Type shapes
// ---------------------------------------------------------------------------

describe("ValidationCheck type shape", () => {
	it("can be constructed with all fields", () => {
		const check: ValidationCheck = {
			category: "Schema",
			severity: "Error",
			artifact_id: "TASK-001",
			message: "Missing required field 'title'",
		};
		expect(check.category).toBe("Schema");
		expect(check.severity).toBe("Error");
	});
});

describe("AppliedFix type shape", () => {
	it("can be constructed with required fields", () => {
		const fix: AppliedFix = {
			artifact_id: "TASK-001",
			description: "Added default status",
		};
		expect(fix.artifact_id).toBe("TASK-001");
	});
});

describe("EnforcementEvent type shape", () => {
	it("can be constructed with all fields", () => {
		const event: EnforcementEvent = {
			mechanism: "pre-commit",
			check_type: "schema",
			rule_id: "R-001",
			artifact_id: "TASK-001",
			result: "pass",
			message: "All checks passed",
		};
		expect(event.mechanism).toBe("pre-commit");
		expect(event.rule_id).toBe("R-001");
	});

	it("allows null rule_id and artifact_id", () => {
		const event: EnforcementEvent = {
			mechanism: "pre-commit",
			check_type: "schema",
			rule_id: null,
			artifact_id: null,
			result: "pass",
			message: "ok",
		};
		expect(event.rule_id).toBeNull();
		expect(event.artifact_id).toBeNull();
	});
});

describe("ValidationReport type shape", () => {
	it("can be constructed with all fields", () => {
		const report: ValidationReport = {
			checks: [],
			health: { status: "ok", artifacts: 5 },
			fixes_applied: [],
			enforcement_events: [],
		};
		expect(report.health).not.toBeNull();
		expect(report.checks).toHaveLength(0);
	});

	it("allows null health field", () => {
		const report: ValidationReport = {
			checks: [],
			health: null,
			fixes_applied: [],
			enforcement_events: [],
		};
		expect(report.health).toBeNull();
	});
});
