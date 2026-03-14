<script lang="ts">
	import { SvelteSet } from "svelte/reactivity";
	import * as Card from "$lib/components/ui/card";
	import { Button } from "$lib/components/ui/button";
	import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";
	import ShieldAlertIcon from "@lucide/svelte/icons/shield-alert";
	import WrenchIcon from "@lucide/svelte/icons/wrench";
	import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
	import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";
	import ArrowUpDownIcon from "@lucide/svelte/icons/arrow-up-down";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ArtifactLink from "$lib/components/artifact/ArtifactLink.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { toast } from "$lib/stores/toast.svelte";
	import type { IntegrityCheck, IntegrityCategory, IntegritySeverity } from "$lib/types/artifact-graph";

	let checks = $state<IntegrityCheck[]>([]);
	let loading = $state(false);
	let fixing = $state(false);
	let scanned = $state(false);
	let error = $state<string | null>(null);

	// Filter state
	let severityFilter = $state<"all" | "Error" | "Warning">("all");
	let categoryFilter = $state<IntegrityCategory | "all">("all");

	// Sort state
	type SortColumn = "severity" | "category" | "artifact" | "message";
	let sortColumn = $state<SortColumn>("severity");
	let sortAsc = $state(true);

	const errorCount = $derived(checks.filter((c) => c.severity === "Error").length);
	const warningCount = $derived(checks.filter((c) => c.severity === "Warning").length);
	const fixableCount = $derived(checks.filter((c) => c.auto_fixable).length);

	const healthColor = $derived.by(() => {
		if (!scanned) return "text-muted-foreground";
		if (errorCount > 0) return "text-destructive";
		if (warningCount > 0) return "text-warning";
		return "text-green-500";
	});

	const categoryLabels: Record<IntegrityCategory, string> = {
		BrokenLink: "Broken Links",
		MissingInverse: "Missing Inverses",
		NullTarget: "Null Targets",
		ResearchGap: "Research Gaps",
		PlanningPlacement: "Untriaged Items",
		DependencyViolation: "Dependency Violations",
		CircularDependency: "Circular Dependencies",
		SupersessionSymmetry: "Supersession Gaps",
		MilestoneGate: "Milestone Gate Violations",
		IdeaPromotionValidity: "Promotion Issues",
		IdeaDeliveryTracking: "Undelivered Ideas",
	};

	/** Unique categories present in current checks, for the filter dropdown. */
	const presentCategories = $derived.by(() => {
		const cats = new SvelteSet<IntegrityCategory>();
		for (const c of checks) cats.add(c.category);
		return [...cats].sort();
	});

	/** Filtered and sorted checks for the table. */
	const tableChecks = $derived.by(() => {
		let filtered = checks;
		if (severityFilter !== "all") {
			filtered = filtered.filter((c) => c.severity === severityFilter);
		}
		if (categoryFilter !== "all") {
			filtered = filtered.filter((c) => c.category === categoryFilter);
		}
		const sorted = [...filtered].sort((a, b) => {
			let cmp = 0;
			switch (sortColumn) {
				case "severity":
					cmp = severityRank(a.severity) - severityRank(b.severity);
					break;
				case "category":
					cmp = (categoryLabels[a.category] ?? a.category).localeCompare(
						categoryLabels[b.category] ?? b.category,
					);
					break;
				case "artifact":
					cmp = a.artifact_id.localeCompare(b.artifact_id);
					break;
				case "message":
					cmp = a.message.localeCompare(b.message);
					break;
			}
			return sortAsc ? cmp : -cmp;
		});
		return sorted;
	});

	function severityRank(s: IntegritySeverity): number {
		return s === "Error" ? 0 : 1;
	}

	function toggleSort(col: SortColumn) {
		if (sortColumn === col) {
			sortAsc = !sortAsc;
		} else {
			sortColumn = col;
			sortAsc = true;
		}
	}

	// Auto-scan when the graph is ready
	$effect(() => {
		if (artifactGraphSDK.graph.size > 0 && !scanned && !loading) {
			void scan();
		}
	});

	async function scan() {
		loading = true;
		error = null;
		try {
			// Refresh the graph from disk before scanning to avoid stale results
			await artifactGraphSDK.refresh();
			checks = await artifactGraphSDK.runIntegrityScan();
			scanned = true;
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
			const appliedFixes = await artifactGraphSDK.applyAutoFixes(fixableChecks);
			toast.success(`${appliedFixes.length} fix${appliedFixes.length !== 1 ? "es" : ""} applied`);
			// Refresh graph after fixes wrote to disk, then re-scan
			await artifactGraphSDK.refresh();
			checks = await artifactGraphSDK.runIntegrityScan();
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
		} finally {
			fixing = false;
		}
	}
</script>

{#if scanned && checks.length === 0 && !error}
	<!-- Collapsed "all clear" state — minimal footprint -->
	<div class="mb-4 flex items-center gap-2 rounded-lg border border-border px-3 py-2">
		<ShieldCheckIcon class="h-4 w-4 text-green-500" />
		<span class="text-sm text-muted-foreground">Pipeline Health</span>
		<span class="text-xs text-green-600 dark:text-green-400">All clear</span>
		{#if loading}
			<LoadingSpinner size="sm" />
		{/if}
	</div>
{:else}
<Card.Root>
	<Card.Header class="pb-3">
		<div class="flex items-center justify-between">
			<Card.Title class="text-base">
				<div class="flex items-center gap-2">
					<ShieldAlertIcon class="h-4 w-4 {healthColor}" />
					Pipeline Health
					{#if loading}
						<LoadingSpinner size="sm" />
					{/if}
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
			</div>
		</div>
	</Card.Header>
	<Card.Content>
		{#if !scanned && loading}
			<div class="flex items-center justify-center py-4">
				<LoadingSpinner />
			</div>
		{:else if error}
			<p class="text-sm text-destructive">{error}</p>
		{:else if !scanned}
			<p class="text-sm text-muted-foreground">
				Waiting for artifact graph...
			</p>
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

				<!-- Filters -->
				<div class="mb-3 flex items-center gap-3">
					<select
						class="h-7 rounded border border-border bg-background px-2 text-xs"
						bind:value={categoryFilter}
					>
						<option value="all">All categories</option>
						{#each presentCategories as cat (cat)}
							<option value={cat}>{categoryLabels[cat]}</option>
						{/each}
					</select>
					<div class="flex items-center gap-1">
						<button
							class="rounded px-2 py-0.5 text-xs transition-colors {severityFilter === 'all' ? 'bg-accent text-accent-foreground' : 'text-muted-foreground hover:text-foreground'}"
							onclick={() => (severityFilter = "all")}
						>All</button>
						<button
							class="rounded px-2 py-0.5 text-xs transition-colors {severityFilter === 'Error' ? 'bg-destructive/20 text-destructive' : 'text-muted-foreground hover:text-foreground'}"
							onclick={() => (severityFilter = "Error")}
						>Errors</button>
						<button
							class="rounded px-2 py-0.5 text-xs transition-colors {severityFilter === 'Warning' ? 'bg-warning/20 text-warning' : 'text-muted-foreground hover:text-foreground'}"
							onclick={() => (severityFilter = "Warning")}
						>Warnings</button>
					</div>
				</div>

				<!-- Data table -->
				<div class="max-h-64 overflow-auto rounded border border-border">
					<table class="w-full text-xs">
						<thead class="sticky top-0 bg-muted/80 backdrop-blur">
							<tr>
								<th class="w-8 px-2 py-1.5 text-left">
									<button
										class="flex items-center gap-0.5 text-muted-foreground hover:text-foreground"
										onclick={() => toggleSort("severity")}
									>
										<ArrowUpDownIcon class="h-3 w-3" />
									</button>
								</th>
								<th class="px-2 py-1.5 text-left">
									<button
										class="flex items-center gap-0.5 text-muted-foreground hover:text-foreground"
										onclick={() => toggleSort("category")}
									>
										Category
										{#if sortColumn === "category"}
											<ArrowUpDownIcon class="h-3 w-3" />
										{/if}
									</button>
								</th>
								<th class="px-2 py-1.5 text-left">
									<button
										class="flex items-center gap-0.5 text-muted-foreground hover:text-foreground"
										onclick={() => toggleSort("artifact")}
									>
										Artifact
										{#if sortColumn === "artifact"}
											<ArrowUpDownIcon class="h-3 w-3" />
										{/if}
									</button>
								</th>
								<th class="px-2 py-1.5 text-left">
									<button
										class="flex items-center gap-0.5 text-muted-foreground hover:text-foreground"
										onclick={() => toggleSort("message")}
									>
										Message
										{#if sortColumn === "message"}
											<ArrowUpDownIcon class="h-3 w-3" />
										{/if}
									</button>
								</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-border">
							{#each tableChecks as check, i (check.artifact_id + check.category + check.message + i)}
								<tr class="hover:bg-accent/30">
									<td class="px-2 py-1.5">
										{#if check.severity === "Error"}
											<CircleAlertIcon class="h-3.5 w-3.5 text-destructive" />
										{:else}
											<TriangleAlertIcon class="h-3.5 w-3.5 text-warning" />
										{/if}
									</td>
									<td class="whitespace-nowrap px-2 py-1.5 text-muted-foreground">
										{categoryLabels[check.category]}
									</td>
									<td class="px-2 py-1.5">
										<ArtifactLink id={check.artifact_id} />
									</td>
									<td class="px-2 py-1.5 text-muted-foreground">
										{check.message}
										{#if check.auto_fixable}
											<span class="ml-1 text-[10px] text-green-600 dark:text-green-400">(auto-fixable)</span>
										{/if}
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
		{/if}
	</Card.Content>
</Card.Root>
{/if}
