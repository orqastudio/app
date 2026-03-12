import { describe, it, expect, beforeEach } from "vitest";
import { mockInvoke } from "./setup";

import { governanceStore } from "../governance.svelte";
import type {
	GovernanceScanResult,
	GovernanceAnalysis,
	Recommendation,
} from "$lib/types/governance";

const fakeScanResult: GovernanceScanResult = {
	has_orqa: false,
	has_git: true,
	detected_stack: ["typescript", "rust"],
	artifact_counts: {},
};

const fakeAnalysis: GovernanceAnalysis = {
	id: 1,
	project_id: 10,
	maturity_level: "initial",
	summary: "Analysis summary",
	created_at: "2026-01-01T00:00:00Z",
};

const fakeRecommendation: Recommendation = {
	id: 1,
	project_id: 10,
	analysis_id: 1,
	category: "governance",
	title: "Add rules",
	description: "You should add governance rules",
	priority: "high",
	status: "pending",
	created_at: "2026-01-01T00:00:00Z",
};

beforeEach(() => {
	mockInvoke.mockReset();
	governanceStore.scanResult = null;
	governanceStore.analysis = null;
	governanceStore.recommendations = [];
	governanceStore.loading = false;
	governanceStore.error = null;
	governanceStore.wizardStep = 0;
	governanceStore.wizardVisible = false;
});

describe("GovernanceStore", () => {
	describe("initial state", () => {
		it("starts with null results and empty recommendations", () => {
			expect(governanceStore.scanResult).toBeNull();
			expect(governanceStore.analysis).toBeNull();
			expect(governanceStore.recommendations).toEqual([]);
			expect(governanceStore.loading).toBe(false);
			expect(governanceStore.error).toBeNull();
		});
	});

	describe("scan", () => {
		it("calls backend and stores result", async () => {
			mockInvoke.mockResolvedValueOnce(fakeScanResult);

			await governanceStore.scan(10);

			expect(mockInvoke).toHaveBeenCalledWith("governance_scan", { projectId: 10 });
			expect(governanceStore.scanResult).toEqual(fakeScanResult);
			expect(governanceStore.loading).toBe(false);
		});

		it("sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Scan error"));

			await governanceStore.scan(10);

			expect(governanceStore.error).toBe("Scan error");
		});
	});

	describe("checkExistingAnalysis", () => {
		it("loads existing analysis", async () => {
			mockInvoke.mockResolvedValueOnce(fakeAnalysis);

			await governanceStore.checkExistingAnalysis(10);

			expect(mockInvoke).toHaveBeenCalledWith("governance_analysis_get", { projectId: 10 });
			expect(governanceStore.analysis).toEqual(fakeAnalysis);
		});

		it("handles null when no analysis exists", async () => {
			mockInvoke.mockResolvedValueOnce(null);

			await governanceStore.checkExistingAnalysis(10);

			expect(governanceStore.analysis).toBeNull();
		});
	});

	describe("analyze", () => {
		it("sends scan result and stores analysis", async () => {
			mockInvoke.mockResolvedValueOnce(fakeAnalysis);

			await governanceStore.analyze(10, fakeScanResult);

			expect(mockInvoke).toHaveBeenCalledWith("governance_analyze", {
				projectId: 10,
				scanResult: fakeScanResult,
			});
			expect(governanceStore.analysis).toEqual(fakeAnalysis);
		});
	});

	describe("loadRecommendations", () => {
		it("loads recommendations from backend", async () => {
			mockInvoke.mockResolvedValueOnce([fakeRecommendation]);

			await governanceStore.loadRecommendations(10);

			expect(mockInvoke).toHaveBeenCalledWith("recommendations_list", { projectId: 10 });
			expect(governanceStore.recommendations).toEqual([fakeRecommendation]);
		});
	});

	describe("approve / reject", () => {
		it("approve updates recommendation status", async () => {
			governanceStore.recommendations = [{ ...fakeRecommendation }];
			const updated = { ...fakeRecommendation, status: "approved" as const };
			mockInvoke.mockResolvedValueOnce(updated);

			await governanceStore.approve(1);

			expect(mockInvoke).toHaveBeenCalledWith("recommendation_update", {
				id: 1,
				status: "approved",
			});
			expect(governanceStore.recommendations[0].status).toBe("approved");
		});

		it("reject updates recommendation status", async () => {
			governanceStore.recommendations = [{ ...fakeRecommendation }];
			const updated = { ...fakeRecommendation, status: "rejected" as const };
			mockInvoke.mockResolvedValueOnce(updated);

			await governanceStore.reject(1);

			expect(mockInvoke).toHaveBeenCalledWith("recommendation_update", {
				id: 1,
				status: "rejected",
			});
			expect(governanceStore.recommendations[0].status).toBe("rejected");
		});
	});

	describe("apply / applyAll", () => {
		it("apply updates single recommendation", async () => {
			governanceStore.recommendations = [{ ...fakeRecommendation }];
			const updated = { ...fakeRecommendation, status: "applied" as const };
			mockInvoke.mockResolvedValueOnce(updated);

			await governanceStore.apply(1);

			expect(mockInvoke).toHaveBeenCalledWith("recommendation_apply", { id: 1 });
			expect(governanceStore.recommendations[0].status).toBe("applied");
		});

		it("applyAll updates multiple recommendations", async () => {
			const rec2 = { ...fakeRecommendation, id: 2, title: "Second" };
			governanceStore.recommendations = [{ ...fakeRecommendation }, rec2];

			const updatedRecs = [
				{ ...fakeRecommendation, status: "applied" as const },
				{ ...rec2, status: "applied" as const },
			];
			mockInvoke.mockResolvedValueOnce(updatedRecs);

			await governanceStore.applyAll(10);

			expect(mockInvoke).toHaveBeenCalledWith("recommendations_apply_all", { projectId: 10 });
			expect(governanceStore.recommendations.every((r) => r.status === "applied")).toBe(true);
		});
	});

	describe("wizard controls", () => {
		it("showWizard sets visible and resets step", () => {
			governanceStore.wizardStep = 3;
			governanceStore.showWizard();

			expect(governanceStore.wizardVisible).toBe(true);
			expect(governanceStore.wizardStep).toBe(0);
		});

		it("dismissWizard hides wizard", () => {
			governanceStore.wizardVisible = true;
			governanceStore.dismissWizard();

			expect(governanceStore.wizardVisible).toBe(false);
		});

		it("nextStep and prevStep navigate", () => {
			governanceStore.nextStep();
			expect(governanceStore.wizardStep).toBe(1);

			governanceStore.nextStep();
			expect(governanceStore.wizardStep).toBe(2);

			governanceStore.prevStep();
			expect(governanceStore.wizardStep).toBe(1);
		});

		it("prevStep does not go below 0", () => {
			governanceStore.wizardStep = 0;
			governanceStore.prevStep();

			expect(governanceStore.wizardStep).toBe(0);
		});
	});
});
