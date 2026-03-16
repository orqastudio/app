import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import type { ClaudeCliInfo, SetupStatus, SetupStepStatus } from "@orqastudio/types";

const STEP_IDS = ["claude_cli", "claude_auth", "sidecar", "embedding_model", "complete"] as const;

export class SetupStore {
	setupComplete = $state(true);
	currentStep = $state(0);
	loading = $state(false);
	error = $state<string | null>(null);
	cliInfo = $state<ClaudeCliInfo | null>(null);
	embeddingStatus = $state<SetupStepStatus | null>(null);
	sidecarStarted = $state(false);

	get stepId(): string {
		return STEP_IDS[this.currentStep] ?? "complete";
	}

	get totalSteps(): number {
		return STEP_IDS.length;
	}

	async checkSetupStatus(): Promise<void> {
		this.loading = true;
		this.error = null;

		try {
			const status = await invoke<SetupStatus>("get_setup_status");
			this.setupComplete = status.setup_complete;
		} catch (err) {
			this.error = extractErrorMessage(err);
			this.setupComplete = false;
		} finally {
			this.loading = false;
		}
	}

	async checkCli(): Promise<void> {
		this.error = null;

		try {
			const info = await invoke<ClaudeCliInfo>("check_claude_cli");
			this.cliInfo = info;
		} catch (err) {
			this.error = extractErrorMessage(err);
			this.cliInfo = null;
		}
	}

	async checkAuth(): Promise<void> {
		this.error = null;

		try {
			const info = await invoke<ClaudeCliInfo>("check_claude_auth");
			this.cliInfo = info;
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}

	async checkEmbeddingModel(): Promise<void> {
		this.error = null;

		try {
			const status = await invoke<SetupStepStatus>("check_embedding_model");
			this.embeddingStatus = status;
		} catch (err) {
			this.error = extractErrorMessage(err);
			this.embeddingStatus = null;
		}
	}

	async reauthenticate(): Promise<void> {
		this.error = null;

		try {
			const info = await invoke<ClaudeCliInfo>("reauthenticate_claude");
			this.cliInfo = info;
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}

	async completeSetup(): Promise<void> {
		this.error = null;

		try {
			await invoke<void>("complete_setup");
			this.setupComplete = true;
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}

	nextStep(): void {
		if (this.currentStep < STEP_IDS.length - 1) {
			this.currentStep++;
		}
	}

	reset(): void {
		this.currentStep = 0;
		this.loading = false;
		this.error = null;
		this.cliInfo = null;
		this.embeddingStatus = null;
		this.sidecarStarted = false;
	}
}
