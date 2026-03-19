import { describe, it, expect, beforeEach } from "vitest";
import { mockInvoke } from "./setup.js";

import { setupStore } from "../../src/stores/setup.svelte.js";
import type { ClaudeCliInfo, SetupStatus, SetupStepStatus } from "@orqastudio/types";

beforeEach(() => {
	mockInvoke.mockReset();
	setupStore.reset();
	setupStore.setupComplete = true;
});

describe("SetupStore", () => {
	describe("initial state", () => {
		it("starts with setupComplete true and currentStep 0", () => {
			expect(setupStore.setupComplete).toBe(true);
			expect(setupStore.currentStep).toBe(0);
			expect(setupStore.loading).toBe(false);
			expect(setupStore.error).toBeNull();
			expect(setupStore.cliInfo).toBeNull();
			expect(setupStore.embeddingStatus).toBeNull();
			expect(setupStore.sidecarStarted).toBe(false);
		});

		it("stepId returns the first step", () => {
			expect(setupStore.stepId).toBe("claude_cli");
		});

		it("totalSteps returns 5", () => {
			expect(setupStore.totalSteps).toBe(5);
		});
	});

	describe("checkSetupStatus", () => {
		it("updates setupComplete from backend", async () => {
			const status: SetupStatus = { setup_complete: false, current_version: 1, stored_version: 0, steps: [] };
			mockInvoke.mockResolvedValueOnce(status);

			await setupStore.checkSetupStatus();

			expect(mockInvoke).toHaveBeenCalledWith("get_setup_status", undefined);
			expect(setupStore.setupComplete).toBe(false);
			expect(setupStore.loading).toBe(false);
		});

		it("sets error and setupComplete=false on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Check failed"));

			await setupStore.checkSetupStatus();

			expect(setupStore.error).toBe("Check failed");
			expect(setupStore.setupComplete).toBe(false);
		});
	});

	describe("checkCli", () => {
		it("sets cliInfo on success", async () => {
			const info: ClaudeCliInfo = {
				installed: true,
				version: "1.0.0",
				path: null,
				authenticated: true,
				subscription_type: null,
				rate_limit_tier: null,
				scopes: [],
				expires_at: null,
			};
			mockInvoke.mockResolvedValueOnce(info);

			await setupStore.checkCli();

			expect(mockInvoke).toHaveBeenCalledWith("check_claude_cli", undefined);
			expect(setupStore.cliInfo).toEqual(info);
		});

		it("sets error and cliInfo null on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("CLI not found"));

			await setupStore.checkCli();

			expect(setupStore.error).toBe("CLI not found");
			expect(setupStore.cliInfo).toBeNull();
		});
	});

	describe("checkAuth", () => {
		it("updates cliInfo with auth status", async () => {
			const info: ClaudeCliInfo = {
				installed: true,
				version: "1.0.0",
				path: null,
				authenticated: true,
				subscription_type: null,
				rate_limit_tier: null,
				scopes: [],
				expires_at: null,
			};
			mockInvoke.mockResolvedValueOnce(info);

			await setupStore.checkAuth();

			expect(mockInvoke).toHaveBeenCalledWith("check_claude_auth", undefined);
			expect(setupStore.cliInfo).toEqual(info);
		});
	});

	describe("checkEmbeddingModel", () => {
		it("sets embeddingStatus on success", async () => {
			const status: SetupStepStatus = { id: "embedding", label: "Embedding Model", status: "complete", detail: "Model loaded" };
			mockInvoke.mockResolvedValueOnce(status);

			await setupStore.checkEmbeddingModel();

			expect(mockInvoke).toHaveBeenCalledWith("check_embedding_model", undefined);
			expect(setupStore.embeddingStatus).toEqual(status);
		});

		it("sets error and null status on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Model unavailable"));

			await setupStore.checkEmbeddingModel();

			expect(setupStore.error).toBe("Model unavailable");
			expect(setupStore.embeddingStatus).toBeNull();
		});
	});

	describe("completeSetup", () => {
		it("sets setupComplete to true on success", async () => {
			setupStore.setupComplete = false;
			mockInvoke.mockResolvedValueOnce(undefined);

			await setupStore.completeSetup();

			expect(mockInvoke).toHaveBeenCalledWith("complete_setup", undefined);
			expect(setupStore.setupComplete).toBe(true);
		});

		it("sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Complete failed"));

			await setupStore.completeSetup();

			expect(setupStore.error).toBe("Complete failed");
		});
	});

	describe("nextStep", () => {
		it("advances the step", () => {
			expect(setupStore.stepId).toBe("claude_cli");

			setupStore.nextStep();
			expect(setupStore.stepId).toBe("claude_auth");

			setupStore.nextStep();
			expect(setupStore.stepId).toBe("sidecar");

			setupStore.nextStep();
			expect(setupStore.stepId).toBe("embedding_model");

			setupStore.nextStep();
			expect(setupStore.stepId).toBe("complete");
		});

		it("does not advance past the last step", () => {
			setupStore.currentStep = 4; // "complete"
			setupStore.nextStep();
			expect(setupStore.currentStep).toBe(4);
		});
	});

	describe("reset", () => {
		it("resets all mutable state", () => {
			setupStore.currentStep = 3;
			setupStore.loading = true;
			setupStore.error = "some error";
			setupStore.sidecarStarted = true;

			setupStore.reset();

			expect(setupStore.currentStep).toBe(0);
			expect(setupStore.loading).toBe(false);
			expect(setupStore.error).toBeNull();
			expect(setupStore.cliInfo).toBeNull();
			expect(setupStore.embeddingStatus).toBeNull();
			expect(setupStore.sidecarStarted).toBe(false);
		});
	});
});
