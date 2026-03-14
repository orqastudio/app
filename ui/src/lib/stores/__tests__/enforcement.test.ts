import { describe, it, expect, vi, beforeEach } from "vitest";
import { mockInvoke } from "./setup";

// Must import after mocks are set up
import { enforcementStore } from "../enforcement.svelte";
import type {
	EnforcementRule,
	EnforcementViolation,
	StoredEnforcementViolation,
} from "$lib/types/enforcement";

beforeEach(() => {
	mockInvoke.mockReset();
	// Reset store state between tests
	enforcementStore.rules = [];
	enforcementStore.violations = [];
	enforcementStore.violationHistory = [];
	enforcementStore.loading = false;
	enforcementStore.historyLoading = false;
	enforcementStore.historyError = null;
	enforcementStore.error = null;
});

describe("EnforcementStore", () => {
	describe("initial state", () => {
		it("starts with empty rules and violations", () => {
			expect(enforcementStore.rules).toEqual([]);
			expect(enforcementStore.violations).toEqual([]);
			expect(enforcementStore.loading).toBe(false);
			expect(enforcementStore.error).toBeNull();
		});

		it("blockCount and warnCount are zero initially", () => {
			expect(enforcementStore.blockCount).toBe(0);
			expect(enforcementStore.warnCount).toBe(0);
		});
	});

	describe("loadRules", () => {
		it("loads rules from backend", async () => {
			const mockRules: EnforcementRule[] = [
				{ name: "RULE-001", scope: "project", entries: [], prose: "blocked" },
			];
			mockInvoke.mockResolvedValueOnce(mockRules);

			await enforcementStore.loadRules();

			expect(mockInvoke).toHaveBeenCalledWith("enforcement_rules_list", undefined);
			expect(enforcementStore.rules).toEqual(mockRules);
			expect(enforcementStore.loading).toBe(false);
			expect(enforcementStore.error).toBeNull();
		});

		it("sets loading state during load", async () => {
			let loadingDuringInvoke = false;
			mockInvoke.mockImplementation(() => {
				loadingDuringInvoke = enforcementStore.loading;
				return Promise.resolve([]);
			});

			await enforcementStore.loadRules();

			expect(loadingDuringInvoke).toBe(true);
			expect(enforcementStore.loading).toBe(false);
		});

		it("sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Network error"));

			await enforcementStore.loadRules();

			expect(enforcementStore.error).toBe("Network error");
			expect(enforcementStore.loading).toBe(false);
		});
	});

	describe("reloadRules", () => {
		it("calls reload then list", async () => {
			const mockRules: EnforcementRule[] = [
				{ name: "RULE-001", scope: "project", entries: [], prose: "warning" },
			];
			mockInvoke
				.mockResolvedValueOnce(5) // enforcement_rules_reload returns count
				.mockResolvedValueOnce(mockRules); // enforcement_rules_list

			await enforcementStore.reloadRules();

			expect(mockInvoke).toHaveBeenCalledTimes(2);
			expect(mockInvoke).toHaveBeenNthCalledWith(1, "enforcement_rules_reload", undefined);
			expect(mockInvoke).toHaveBeenNthCalledWith(2, "enforcement_rules_list", undefined);
			expect(enforcementStore.rules).toEqual(mockRules);
		});

		it("sets error if reload fails", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Reload failed"));

			await enforcementStore.reloadRules();

			expect(enforcementStore.error).toBe("Reload failed");
			expect(enforcementStore.loading).toBe(false);
		});
	});

	describe("violations", () => {
		it("addViolation appends to the list", () => {
			const v: EnforcementViolation = { rule_name: "r1", action: "Block", tool_name: "write_file", detail: "blocked", timestamp: "2026-01-01T00:00:00Z" };
			enforcementStore.addViolation(v);

			expect(enforcementStore.violations).toHaveLength(1);
			expect(enforcementStore.violations[0]).toEqual(v);
		});

		it("clearViolations resets the list", () => {
			enforcementStore.addViolation({ rule_name: "r1", action: "Block", tool_name: "write_file", detail: "x", timestamp: "2026-01-01T00:00:00Z" });
			enforcementStore.addViolation({ rule_name: "r2", action: "Warn", tool_name: "write_file", detail: "y", timestamp: "2026-01-01T00:00:00Z" });

			enforcementStore.clearViolations();

			expect(enforcementStore.violations).toEqual([]);
		});

		it("blockCount counts Block violations", () => {
			enforcementStore.violations = [
				{ rule_name: "r1", action: "Block", tool_name: "write_file", detail: "x", timestamp: "2026-01-01T00:00:00Z" },
				{ rule_name: "r2", action: "Warn", tool_name: "write_file", detail: "y", timestamp: "2026-01-01T00:00:00Z" },
				{ rule_name: "r3", action: "Block", tool_name: "write_file", detail: "z", timestamp: "2026-01-01T00:00:00Z" },
			];

			expect(enforcementStore.blockCount).toBe(2);
			expect(enforcementStore.warnCount).toBe(1);
		});
	});

	describe("loadViolationHistory", () => {
		it("loads violation history from backend", async () => {
			const mockHistory: StoredEnforcementViolation[] = [
				{
					id: 1,
					project_id: 42,
					rule_name: "RULE-006",
					action: "block",
					tool_name: "write_file",
					detail: "/some/path.ts",
					created_at: "2026-03-14T10:00:00",
				},
				{
					id: 2,
					project_id: 42,
					rule_name: "RULE-007",
					action: "warn",
					tool_name: "bash",
					detail: null,
					created_at: "2026-03-14T09:00:00",
				},
			];
			mockInvoke.mockResolvedValueOnce(mockHistory);

			await enforcementStore.loadViolationHistory();

			expect(mockInvoke).toHaveBeenCalledWith("enforcement_violations_list", undefined);
			expect(enforcementStore.violationHistory).toEqual(mockHistory);
			expect(enforcementStore.historyLoading).toBe(false);
			expect(enforcementStore.historyError).toBeNull();
		});

		it("sets historyLoading during load", async () => {
			let loadingDuringInvoke = false;
			mockInvoke.mockImplementation(() => {
				loadingDuringInvoke = enforcementStore.historyLoading;
				return Promise.resolve([]);
			});

			await enforcementStore.loadViolationHistory();

			expect(loadingDuringInvoke).toBe(true);
			expect(enforcementStore.historyLoading).toBe(false);
		});

		it("sets historyError on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("DB unavailable"));

			await enforcementStore.loadViolationHistory();

			expect(enforcementStore.historyError).toBe("DB unavailable");
			expect(enforcementStore.historyLoading).toBe(false);
		});

		it("clears previous historyError on successful reload", async () => {
			enforcementStore.historyError = "old error";
			mockInvoke.mockResolvedValueOnce([]);

			await enforcementStore.loadViolationHistory();

			expect(enforcementStore.historyError).toBeNull();
		});
	});
});
