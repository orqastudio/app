import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import type {
	EnforcementRule,
	EnforcementViolation,
	StoredEnforcementViolation,
} from "@orqastudio/types";

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

	addViolation(v: EnforcementViolation): void {
		this.violations.push(v);
	}

	clearViolations(): void {
		this.violations = [];
	}

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
