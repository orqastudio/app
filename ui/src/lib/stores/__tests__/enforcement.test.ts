import { describe, it, expect, vi, beforeEach } from "vitest";
import { mockInvoke } from "./setup";

// Must import after mocks are set up
import { enforcementStore } from "../enforcement.svelte";
import type { EnforcementRule, EnforcementViolation } from "$lib/types/enforcement";

beforeEach(() => {
	mockInvoke.mockReset();
	// Reset store state between tests
	enforcementStore.rules = [];
	enforcementStore.violations = [];
	enforcementStore.loading = false;
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
				{ id: "r1", source_rule: "RULE-001", event: "file", action: "Block", paths: ["src/**"], pattern: null, message: "blocked", skills: null },
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
				{ id: "r1", source_rule: "RULE-001", event: "file", action: "Warn", paths: [], pattern: null, message: "warning", skills: null },
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
			const v: EnforcementViolation = { rule_id: "r1", action: "Block", message: "blocked", path: "src/foo.ts" };
			enforcementStore.addViolation(v);

			expect(enforcementStore.violations).toHaveLength(1);
			expect(enforcementStore.violations[0]).toEqual(v);
		});

		it("clearViolations resets the list", () => {
			enforcementStore.addViolation({ rule_id: "r1", action: "Block", message: "x", path: "a.ts" });
			enforcementStore.addViolation({ rule_id: "r2", action: "Warn", message: "y", path: "b.ts" });

			enforcementStore.clearViolations();

			expect(enforcementStore.violations).toEqual([]);
		});

		it("blockCount counts Block violations", () => {
			enforcementStore.violations = [
				{ rule_id: "r1", action: "Block", message: "x", path: "a.ts" },
				{ rule_id: "r2", action: "Warn", message: "y", path: "b.ts" },
				{ rule_id: "r3", action: "Block", message: "z", path: "c.ts" },
			];

			expect(enforcementStore.blockCount).toBe(2);
			expect(enforcementStore.warnCount).toBe(1);
		});
	});
});
