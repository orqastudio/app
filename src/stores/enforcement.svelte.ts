import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import type { EnforcementRule, EnforcementViolation } from "@orqastudio/types";

class EnforcementStore {
	rules = $state<EnforcementRule[]>([]);
	violations = $state<EnforcementViolation[]>([]);
	loading = $state(false);
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
}

export const enforcementStore = new EnforcementStore();
