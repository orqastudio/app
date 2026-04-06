import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import type {
	EnforcementRule,
	EnforcementViolation,
	StoredEnforcementViolation,
} from "@orqastudio/types";

/**
 * Reactive store managing enforcement rules and active violations, plus persistent violation history from the backend.
 */
export class EnforcementStore {
	rules = $state<EnforcementRule[]>([]);
	violations = $state<EnforcementViolation[]>([]);
	/** Violation history loaded from SQLite (persistent across sessions). */
	violationHistory = $state<StoredEnforcementViolation[]>([]);
	loading = $state(false);
	historyLoading = $state(false);
	historyError = $state<string | null>(null);
	error = $state<string | null>(null);

	blockCount = $derived(this.violations.filter((v) => v.action === "Block").length);
	warnCount = $derived(this.violations.filter((v) => v.action === "Warn").length);

	/**
	 * Fetches the current enforcement rules from the backend and updates reactive state.
	 */
	async loadRules(): Promise<void> {
		this.loading = true;
		this.error = null;
		try {
			this.rules = await invoke<EnforcementRule[]>("enforcement_rules_list");
		} catch (err) {
			this.error = extractErrorMessage(err);
		} finally {
			this.loading = false;
		}
	}

	/**
	 * Instructs the backend to reload rules from disk, then refreshes the local rules list.
	 */
	async reloadRules(): Promise<void> {
		this.loading = true;
		this.error = null;
		try {
			await invoke<number>("enforcement_rules_reload");
			this.rules = await invoke<EnforcementRule[]>("enforcement_rules_list");
		} catch (err) {
			this.error = extractErrorMessage(err);
		} finally {
			this.loading = false;
		}
	}

	/**
	 * Appends a new enforcement violation to the in-memory violations list.
	 * @param v - The violation event received from the enforcement engine.
	 */
	addViolation(v: EnforcementViolation): void {
		this.violations = [...this.violations, v];
	}

	/**
	 * Clears all in-memory enforcement violations for the current session.
	 */
	clearViolations(): void {
		this.violations = [];
	}

	/**
	 * Loads the persistent violation history from the backend database and updates reactive state.
	 */
	async loadViolationHistory(): Promise<void> {
		this.historyLoading = true;
		this.historyError = null;
		try {
			this.violationHistory = await invoke<StoredEnforcementViolation[]>(
				"enforcement_violations_list",
			);
		} catch (err) {
			this.historyError = extractErrorMessage(err);
		} finally {
			this.historyLoading = false;
		}
	}
}
