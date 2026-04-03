/**
 * Shared access to the orqa-validation Rust binary / daemon.
 *
 * Both `orqa check` and `orqa enforce` use the same underlying validation
 * engine (engine/validation/).  This module provides the common helpers:
 *
 *   findBinary()     — locate the compiled orqa-validation binary
 *   callDaemon()     — POST to the running daemon's /validation/scan endpoint
 *   runRustBinary()  — exec the binary directly and capture JSON output
 *   runValidation()  — daemon-first, binary-fallback orchestration
 */
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
/**
 * Find the Rust validation binary. Checks common build locations relative to
 * the project root, including workspace-level and crate-level target dirs.
 * @param projectRoot - Absolute path to the project root.
 * @returns Absolute path to the binary, or null if not found.
 */
export declare function findBinary(projectRoot: string): string | null;
/**
 * Call the running daemon's /validation/scan endpoint.
 * Returns the raw JSON response string, or null if the daemon is unreachable.
 * @param targetPath - Path to the file or directory to validate.
 * @param autoFix - Whether to automatically apply fixes.
 * @returns The raw JSON response string, or null if the daemon is unreachable.
 */
export declare function callDaemon(targetPath: string, autoFix: boolean): Promise<string | null>;
/**
 * Run the Rust validator binary directly and capture its JSON output.
 * @param binaryPath - Absolute path to the orqa-validation binary.
 * @param targetPath - Path to the file or directory to validate.
 * @param autoFix - Whether to automatically apply fixes.
 * @returns Exit code and captured output string from the binary.
 */
export declare function runRustBinary(binaryPath: string, targetPath: string, autoFix: boolean): {
    exitCode: number;
    output: string;
};
/**
 * Run validation: daemon-first, binary-fallback.
 *
 * Returns a parsed `ValidationReport` and the process exit code.
 * Throws if neither the daemon nor the binary are available.
 * @param projectRoot - Absolute path to the project root.
 * @param targetPath - Path to the file or directory to validate.
 * @param autoFix - Whether to automatically apply fixes.
 * @returns Parsed validation report and process exit code.
 */
export declare function runValidation(projectRoot: string, targetPath: string, autoFix?: boolean): Promise<{
    report: ValidationReport;
    exitCode: number;
}>;
/**
 * Format a validation report as human-readable text output.
 * Returns the formatted string and counts of errors/warnings.
 * @param report - The validation report to format.
 * @returns Object with formatted text and error/warning counts.
 */
export declare function formatReport(report: ValidationReport): {
    text: string;
    errors: number;
    warnings: number;
};
//# sourceMappingURL=validation-engine.d.ts.map