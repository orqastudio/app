import { invoke, extractErrorMessage } from "$lib/ipc/invoke";
import type {
	GovernanceScanResult,
	GovernanceAnalysis,
	Recommendation,
	RecommendationStatus,
} from "$lib/types/governance";

class GovernanceStore {
	scanResult = $state<GovernanceScanResult | null>(null);
	analysis = $state<GovernanceAnalysis | null>(null);
	recommendations = $state<Recommendation[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);
	wizardStep = $state(0);
	wizardVisible = $state(false);

	async scan(projectId: number): Promise<void> {
		this.loading = true;
		this.error = null;
		try {
			this.scanResult = await invoke<GovernanceScanResult>("governance_scan", { projectId });
		} catch (err) {
			this.error = extractErrorMessage(err);
		} finally {
			this.loading = false;
		}
	}

	async checkExistingAnalysis(projectId: number): Promise<void> {
		this.loading = true;
		this.error = null;
		try {
			this.analysis = await invoke<GovernanceAnalysis | null>("governance_analysis_get", {
				projectId,
			});
		} catch (err) {
			this.error = extractErrorMessage(err);
		} finally {
			this.loading = false;
		}
	}

	async analyze(projectId: number, scanResult: GovernanceScanResult): Promise<void> {
		this.loading = true;
		this.error = null;
		try {
			this.analysis = await invoke<GovernanceAnalysis>("governance_analyze", {
				projectId,
				scanResult,
			});
		} catch (err) {
			this.error = extractErrorMessage(err);
		} finally {
			this.loading = false;
		}
	}

	async loadRecommendations(projectId: number): Promise<void> {
		this.loading = true;
		this.error = null;
		try {
			this.recommendations = await invoke<Recommendation[]>("recommendations_list", {
				projectId,
			});
		} catch (err) {
			this.error = extractErrorMessage(err);
		} finally {
			this.loading = false;
		}
	}

	async approve(id: number): Promise<void> {
		this.error = null;
		try {
			const updated = await invoke<Recommendation>("recommendation_update", {
				id,
				status: "approved" satisfies RecommendationStatus,
			});
			this.recommendations = this.recommendations.map((r) => (r.id === id ? updated : r));
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}

	async reject(id: number): Promise<void> {
		this.error = null;
		try {
			const updated = await invoke<Recommendation>("recommendation_update", {
				id,
				status: "rejected" satisfies RecommendationStatus,
			});
			this.recommendations = this.recommendations.map((r) => (r.id === id ? updated : r));
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}

	async apply(id: number): Promise<void> {
		this.error = null;
		try {
			const updated = await invoke<Recommendation>("recommendation_apply", { id });
			this.recommendations = this.recommendations.map((r) => (r.id === id ? updated : r));
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}

	async applyAll(projectId: number): Promise<void> {
		this.loading = true;
		this.error = null;
		try {
			const updated = await invoke<Recommendation[]>("recommendations_apply_all", { projectId });
			const updatedMap = new Map(updated.map((r) => [r.id, r]));
			this.recommendations = this.recommendations.map((r) => updatedMap.get(r.id) ?? r);
		} catch (err) {
			this.error = extractErrorMessage(err);
		} finally {
			this.loading = false;
		}
	}

	dismissWizard(): void {
		this.wizardVisible = false;
	}

	showWizard(): void {
		this.wizardVisible = true;
		this.wizardStep = 0;
	}

	nextStep(): void {
		this.wizardStep += 1;
	}

	prevStep(): void {
		if (this.wizardStep > 0) {
			this.wizardStep -= 1;
		}
	}
}

export const governanceStore = new GovernanceStore();
