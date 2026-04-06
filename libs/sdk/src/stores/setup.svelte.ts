import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import { logger } from "../logger.js";
import type { ClaudeCliInfo, SetupStatus, SetupStepStatus } from "@orqastudio/types";

const log = logger("setup");

const STEP_IDS = ["claude_cli", "claude_auth", "sidecar", "embedding_model", "complete"] as const;

/**
 * Reactive store that tracks setup wizard state and coordinates prerequisite checks.
 */
export class SetupStore {
	setupComplete = $state(true);
	currentStep = $state(0);
	loading = $state(false);
	error = $state<string | null>(null);
	cliInfo = $state<ClaudeCliInfo | null>(null);
	embeddingStatus = $state<SetupStepStatus | null>(null);
	sidecarStarted = $state(false);

	/**
	 * Returns the string identifier of the current setup step.
	 * @returns The step ID string for the current step index.
	 */
	get stepId(): string {
		return STEP_IDS[this.currentStep] ?? "complete";
	}

	/**
	 * Returns the total number of setup steps.
	 * @returns The count of steps in the setup sequence.
	 */
	get totalSteps(): number {
		return STEP_IDS.length;
	}

	/**
	 * Checks the overall setup status from the backend and updates setupComplete.
	 */
	async checkSetupStatus(): Promise<void> {
		this.loading = true;
		this.error = null;

		try {
			const status = await invoke<SetupStatus>("get_setup_status");
			this.setupComplete = status.setup_complete;
			if (status.setup_complete) {
				log.info("setup complete");
			} else {
				log.info("setup incomplete", { currentStep: this.stepId });
			}
		} catch (err) {
			this.error = extractErrorMessage(err);
			this.setupComplete = false;
		} finally {
			this.loading = false;
		}
	}

	/**
	 * Checks whether the Claude CLI is installed and available, updating cliInfo.
	 */
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

	/**
	 * Checks whether the Claude CLI is authenticated, updating cliInfo with auth status.
	 */
	async checkAuth(): Promise<void> {
		this.error = null;

		try {
			const info = await invoke<ClaudeCliInfo>("check_claude_auth");
			this.cliInfo = info;
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}

	/**
	 * Checks whether the embedding model is available, updating embeddingStatus.
	 */
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

	/**
	 * Triggers reauthentication of the Claude CLI and updates cliInfo with the result.
	 */
	async reauthenticate(): Promise<void> {
		this.error = null;

		try {
			const info = await invoke<ClaudeCliInfo>("reauthenticate_claude");
			this.cliInfo = info;
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}

	/**
	 * Marks setup as complete in the backend and updates the local setupComplete flag.
	 */
	async completeSetup(): Promise<void> {
		this.error = null;

		try {
			await invoke<void>("complete_setup");
			this.setupComplete = true;
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}

	/**
	 * Advances to the next setup step if one exists.
	 */
	nextStep(): void {
		if (this.currentStep < STEP_IDS.length - 1) {
			this.currentStep++;
		}
	}

	/**
	 * Resets all store state back to initial values, clearing step progress and errors.
	 */
	reset(): void {
		this.currentStep = 0;
		this.loading = false;
		this.error = null;
		this.cliInfo = null;
		this.embeddingStatus = null;
		this.sidecarStarted = false;
	}
}
