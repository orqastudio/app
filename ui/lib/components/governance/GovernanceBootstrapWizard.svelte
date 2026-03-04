<script lang="ts">
	import XIcon from "@lucide/svelte/icons/x";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import SparklesIcon from "@lucide/svelte/icons/sparkles";
	import CheckCircleIcon from "@lucide/svelte/icons/check-circle";
	import { Button } from "$lib/components/ui/button";
	import * as Card from "$lib/components/ui/card";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import GovernanceScanPanel from "./GovernanceScanPanel.svelte";
	import RecommendationList from "./RecommendationList.svelte";
	import { governanceStore } from "$lib/stores/governance.svelte";

	interface Props {
		projectId: number;
	}

	const { projectId }: Props = $props();

	const TOTAL_STEPS = 4;

	// Step 0: Scan results
	// Step 1: Analyze with Claude
	// Step 2: Review recommendations
	// Step 3: Done

	const stepTitles = [
		"Governance Scan",
		"Claude Analysis",
		"Review Recommendations",
		"Bootstrap Complete",
	];

	const stepDescriptions = [
		"Scanned your project for existing governance files.",
		"Claude is analyzing your governance coverage.",
		"Review and approve recommendations to improve your governance.",
		"Your governance has been bootstrapped successfully.",
	];

	async function handleAnalyze() {
		if (!governanceStore.scanResult) return;
		await governanceStore.analyze(projectId, governanceStore.scanResult);
		if (!governanceStore.error) {
			await governanceStore.loadRecommendations(projectId);
			governanceStore.nextStep();
		}
	}

	async function handleApprove(id: number) {
		await governanceStore.approve(id);
	}

	async function handleReject(id: number) {
		await governanceStore.reject(id);
	}

	async function handleApply(id: number) {
		await governanceStore.apply(id);
	}

	async function handleApplyAll() {
		await governanceStore.applyAll(projectId);
	}

	function handleFinish() {
		governanceStore.dismissWizard();
	}

	const appliedCount = $derived(
		governanceStore.recommendations.filter((r) => r.status === "applied").length,
	);
</script>

<!-- Overlay backdrop -->
<div
	class="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm"
	role="dialog"
	aria-modal="true"
	aria-labelledby="wizard-title"
>
	<div class="relative w-full max-w-2xl px-4">
		<Card.Root class="max-h-[90vh] overflow-hidden">
			<!-- Header -->
			<Card.Header class="border-b pb-4">
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2">
						<ShieldIcon class="h-5 w-5 text-primary" />
						<Card.Title id="wizard-title">Governance Bootstrap</Card.Title>
					</div>
					<button
						class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none"
						onclick={() => governanceStore.dismissWizard()}
						aria-label="Dismiss wizard"
					>
						<XIcon class="h-4 w-4" />
					</button>
				</div>

				<!-- Step indicator -->
				<div class="mt-3 flex items-center gap-2">
					{#each Array(TOTAL_STEPS) as _, i}
						<div
							class="h-1.5 flex-1 rounded-full transition-colors {i < governanceStore.wizardStep
								? 'bg-primary'
								: i === governanceStore.wizardStep
									? 'bg-primary'
									: 'bg-muted'}"
						></div>
					{/each}
				</div>

				<div class="mt-2">
					<p class="text-sm font-medium">{stepTitles[governanceStore.wizardStep]}</p>
					<p class="text-xs text-muted-foreground">
						{stepDescriptions[governanceStore.wizardStep]}
					</p>
				</div>
			</Card.Header>

			<!-- Body -->
			<Card.Content class="max-h-[55vh] overflow-y-auto p-4">
				{#if governanceStore.error}
					<ErrorDisplay
						message={governanceStore.error}
						onRetry={() => {
							governanceStore.error = null;
						}}
					/>
				{:else if governanceStore.wizardStep === 0}
					<!-- Step 0: Scan results -->
					{#if governanceStore.loading}
						<LoadingSpinner />
					{:else if governanceStore.scanResult}
						<GovernanceScanPanel scanResult={governanceStore.scanResult} />
					{:else}
						<div class="flex flex-col items-center justify-center py-8">
							<LoadingSpinner />
							<p class="mt-3 text-sm text-muted-foreground">Scanning governance files...</p>
						</div>
					{/if}
				{:else if governanceStore.wizardStep === 1}
					<!-- Step 1: Analyzing with Claude -->
					{#if governanceStore.loading}
						<div class="flex flex-col items-center justify-center py-12">
							<LoadingSpinner size="lg" />
							<p class="mt-4 text-sm font-medium">Analyzing with Claude...</p>
							<p class="mt-1 text-xs text-muted-foreground">
								Claude is reading your governance files and generating recommendations.
							</p>
						</div>
					{:else if governanceStore.analysis}
						<div class="space-y-4">
							<div class="rounded-md border bg-muted/30 p-4">
								<p class="text-sm font-medium">Analysis Summary</p>
								<p class="mt-1 text-sm text-muted-foreground">{governanceStore.analysis.summary}</p>
							</div>

							{#if governanceStore.analysis.strengths.length > 0}
								<div>
									<p class="mb-2 text-sm font-medium text-green-700 dark:text-green-400">
										Strengths
									</p>
									<ul class="space-y-1">
										{#each governanceStore.analysis.strengths as strength}
											<li class="flex items-start gap-2 text-sm">
												<CheckCircleIcon class="mt-0.5 h-4 w-4 flex-shrink-0 text-green-600" />
												<span>{strength}</span>
											</li>
										{/each}
									</ul>
								</div>
							{/if}

							{#if governanceStore.analysis.gaps.length > 0}
								<div>
									<p class="mb-2 text-sm font-medium text-muted-foreground">Gaps Identified</p>
									<ul class="space-y-1">
										{#each governanceStore.analysis.gaps as gap}
											<li class="flex items-start gap-2 text-sm text-muted-foreground">
												<span class="mt-1.5 h-1.5 w-1.5 flex-shrink-0 rounded-full bg-muted-foreground"></span>
												<span>{gap}</span>
											</li>
										{/each}
									</ul>
								</div>
							{/if}
						</div>
					{/if}
				{:else if governanceStore.wizardStep === 2}
					<!-- Step 2: Review recommendations -->
					<RecommendationList
						recommendations={governanceStore.recommendations}
						loading={governanceStore.loading}
						onApprove={handleApprove}
						onReject={handleReject}
						onApply={handleApply}
						onApplyAll={handleApplyAll}
					/>
				{:else if governanceStore.wizardStep === 3}
					<!-- Step 3: Done -->
					<div class="flex flex-col items-center justify-center py-8 text-center">
						<div
							class="mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-green-100 dark:bg-green-900/30"
						>
							<CheckCircleIcon class="h-8 w-8 text-green-600" />
						</div>
						<p class="text-lg font-semibold">Governance Bootstrapped</p>
						<p class="mt-1 text-sm text-muted-foreground">
							{appliedCount} recommendation{appliedCount === 1 ? "" : "s"} applied to your project.
						</p>
						<p class="mt-1 text-xs text-muted-foreground">
							Governance artifacts have been written to your project's <code
								class="font-mono">.claude/</code
							> directory.
						</p>
					</div>
				{/if}
			</Card.Content>

			<!-- Footer -->
			<Card.Footer class="flex items-center justify-between border-t pt-4">
				<div>
					{#if governanceStore.wizardStep > 0 && governanceStore.wizardStep < 3}
						<Button
							variant="ghost"
							size="sm"
							onclick={() => governanceStore.prevStep()}
							disabled={governanceStore.loading}
						>
							Back
						</Button>
					{/if}
				</div>

				<div class="flex items-center gap-2">
					{#if governanceStore.wizardStep < 3}
						<Button
							variant="ghost"
							size="sm"
							onclick={() => governanceStore.dismissWizard()}
						>
							Skip
						</Button>
					{/if}

					{#if governanceStore.wizardStep === 0 && governanceStore.scanResult}
						<Button onclick={() => governanceStore.nextStep()} disabled={governanceStore.loading}>
							<SparklesIcon class="mr-1 h-4 w-4" />
							Analyze with Claude
						</Button>
					{:else if governanceStore.wizardStep === 1 && !governanceStore.loading && !governanceStore.analysis}
						<Button onclick={handleAnalyze} disabled={governanceStore.loading}>
							<SparklesIcon class="mr-1 h-4 w-4" />
							Analyze with Claude
						</Button>
					{:else if governanceStore.wizardStep === 1 && governanceStore.analysis}
						<Button onclick={() => governanceStore.nextStep()} disabled={governanceStore.loading}>
							Review Recommendations
						</Button>
					{:else if governanceStore.wizardStep === 2}
						<Button onclick={() => governanceStore.nextStep()} disabled={governanceStore.loading}>
							Finish
						</Button>
					{:else if governanceStore.wizardStep === 3}
						<Button onclick={handleFinish}>Close</Button>
					{/if}
				</div>
			</Card.Footer>
		</Card.Root>
	</div>
</div>
