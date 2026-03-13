<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import * as Collapsible from "$lib/components/ui/collapsible";
	import { Button } from "$lib/components/ui/button";
	import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";
	import ShieldAlertIcon from "@lucide/svelte/icons/shield-alert";
	import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
	import WrenchIcon from "@lucide/svelte/icons/wrench";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
	import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";
	import CheckCircle2Icon from "@lucide/svelte/icons/check-circle-2";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ArtifactLink from "$lib/components/artifact/ArtifactLink.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import type { IntegrityCheck, IntegrityCategory, AppliedFix } from "$lib/types/artifact-graph";

	let checks = $state<IntegrityCheck[]>([]);
	let loading = $state(false);
	let fixing = $state(false);
	let scanned = $state(false);
	let error = $state<string | null>(null);
	let appliedFixes = $state<AppliedFix[]>([]);

	const errorCount = $derived(checks.filter((c) => c.severity === "Error").length);
	const warningCount = $derived(checks.filter((c) => c.severity === "Warning").length);
	const fixableCount = $derived(checks.filter((c) => c.auto_fixable).length);

	const healthColor = $derived.by(() => {
		if (!scanned) return "text-muted-foreground";
		if (errorCount > 0) return "text-destructive";
		if (warningCount > 0) return "text-warning";
		return "text-green-500";
	});

	const grouped = $derived.by(() => {
		const map = new Map<IntegrityCategory, IntegrityCheck[]>();
		for (const check of checks) {
			const existing = map.get(check.category) ?? [];
			existing.push(check);
			map.set(check.category, existing);
		}
		return map;
	});

	const categoryLabels: Record<IntegrityCategory, string> = {
		BrokenLink: "Broken Links",
		MissingInverse: "Missing Inverses",
		NullTarget: "Null Targets",
		ResearchGap: "Research Gaps",
	};

	let expandedCategories = $state<Set<IntegrityCategory>>(new Set());

	function toggleCategory(cat: IntegrityCategory) {
		const next = new Set(expandedCategories);
		if (next.has(cat)) {
			next.delete(cat);
		} else {
			next.add(cat);
		}
		expandedCategories = next;
	}

	async function scan() {
		loading = true;
		error = null;
		appliedFixes = [];
		try {
			checks = await artifactGraphSDK.runIntegrityScan();
			scanned = true;
			// Store a health snapshot after each scan
			const errors = checks.filter((c) => c.severity === "Error").length;
			const warnings = checks.filter((c) => c.severity === "Warning").length;
			await artifactGraphSDK.storeHealthSnapshot(errors, warnings).catch(() => {
				// Non-critical — don't block the UI if snapshot storage fails
			});
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
		} finally {
			loading = false;
		}
	}

	async function fix() {
		fixing = true;
		error = null;
		try {
			const fixableChecks = checks.filter((c) => c.auto_fixable);
			appliedFixes = await artifactGraphSDK.applyAutoFixes(fixableChecks);
			// Re-scan to show remaining issues
			checks = await artifactGraphSDK.runIntegrityScan();
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
		} finally {
			fixing = false;
		}
	}
</script>

<Card.Root class="mb-4">
	<Card.Header class="pb-3">
		<div class="flex items-center justify-between">
			<Card.Title class="text-base">
				<div class="flex items-center gap-2">
					{#if scanned && errorCount === 0 && warningCount === 0}
						<ShieldCheckIcon class="h-4 w-4 text-green-500" />
					{:else}
						<ShieldAlertIcon class="h-4 w-4 {healthColor}" />
					{/if}
					Pipeline Health
				</div>
			</Card.Title>
			<div class="flex items-center gap-1">
				{#if scanned && fixableCount > 0}
					<Button
						variant="ghost"
						size="sm"
						onclick={fix}
						disabled={fixing || loading}
					>
						{#if fixing}
							<LoadingSpinner size="sm" />
						{:else}
							<WrenchIcon class="mr-1.5 h-3.5 w-3.5" />
						{/if}
						Fix ({fixableCount})
					</Button>
				{/if}
				<Button
					variant="ghost"
					size="sm"
					onclick={scan}
					disabled={loading || fixing}
				>
					{#if loading}
						<LoadingSpinner size="sm" />
					{:else}
						<RefreshCwIcon class="mr-1.5 h-3.5 w-3.5" />
					{/if}
					Scan
				</Button>
			</div>
		</div>
	</Card.Header>
	<Card.Content>
		{#if !scanned && !loading}
			<p class="text-sm text-muted-foreground">
				Click Scan to check pipeline integrity.
			</p>
		{:else if loading && !scanned}
			<div class="flex items-center justify-center py-4">
				<LoadingSpinner />
			</div>
		{:else if error}
			<p class="text-sm text-destructive">{error}</p>
		{:else}
			<!-- Applied fixes banner -->
			{#if appliedFixes.length > 0}
				<div class="mb-3 rounded-md border border-green-200 bg-green-50 p-2 dark:border-green-800 dark:bg-green-950">
					<p class="mb-1 text-xs font-medium text-green-700 dark:text-green-300">
						{appliedFixes.length} fix{appliedFixes.length !== 1 ? "es" : ""} applied
					</p>
					<ul class="space-y-0.5">
						{#each appliedFixes as appliedFix (appliedFix.artifact_id + appliedFix.description)}
							<li class="flex items-start gap-1.5 text-xs text-green-600 dark:text-green-400">
								<CheckCircle2Icon class="mt-0.5 h-3 w-3 shrink-0" />
								<span>
									<ArtifactLink id={appliedFix.artifact_id} />
									<span class="ml-1 text-muted-foreground">{appliedFix.description}</span>
								</span>
							</li>
						{/each}
					</ul>
				</div>
			{/if}

			{#if checks.length === 0}
				<div class="flex items-center gap-2 text-sm text-green-600 dark:text-green-400">
					<CheckCircle2Icon class="h-4 w-4" />
					All clear — no integrity issues found.
				</div>
			{:else}
				<!-- Summary -->
				<div class="mb-3 flex items-center gap-4 text-sm">
					{#if errorCount > 0}
						<span class="flex items-center gap-1 text-destructive">
							<CircleAlertIcon class="h-3.5 w-3.5" />
							{errorCount} error{errorCount !== 1 ? "s" : ""}
						</span>
					{/if}
					{#if warningCount > 0}
						<span class="flex items-center gap-1 text-warning">
							<TriangleAlertIcon class="h-3.5 w-3.5" />
							{warningCount} warning{warningCount !== 1 ? "s" : ""}
						</span>
					{/if}
				</div>

				<!-- Grouped issues -->
				<div class="space-y-1">
					{#each [...grouped.entries()] as [category, categoryChecks] (category)}
						<Collapsible.Root open={expandedCategories.has(category)}>
							<Collapsible.Trigger
								class="flex w-full items-center gap-1.5 rounded px-1 py-0.5 text-xs font-medium text-muted-foreground hover:text-foreground hover:bg-accent/50 transition-colors"
								onclick={() => toggleCategory(category)}
							>
								<ChevronRightIcon
									class="h-3 w-3 transition-transform {expandedCategories.has(category) ? 'rotate-90' : ''}"
								/>
								{categoryLabels[category]}
								<span class="text-[10px] tabular-nums">({categoryChecks.length})</span>
							</Collapsible.Trigger>
							<Collapsible.Content>
								<div class="space-y-1 pl-5 pt-1">
									{#each categoryChecks as check (check.artifact_id + check.message)}
										<div class="flex items-start gap-2 text-xs">
											{#if check.severity === "Error"}
												<CircleAlertIcon class="mt-0.5 h-3 w-3 shrink-0 text-destructive" />
											{:else}
												<TriangleAlertIcon class="mt-0.5 h-3 w-3 shrink-0 text-warning" />
											{/if}
											<div class="min-w-0">
												<ArtifactLink id={check.artifact_id} />
												<span class="text-muted-foreground ml-1">{check.message}</span>
												{#if check.auto_fixable}
													<span class="ml-1 text-[10px] text-green-600 dark:text-green-400">(auto-fixable)</span>
												{/if}
											</div>
										</div>
									{/each}
								</div>
							</Collapsible.Content>
						</Collapsible.Root>
					{/each}
				</div>
			{/if}
		{/if}
	</Card.Content>
</Card.Root>
