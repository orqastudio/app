<script lang="ts">
	import { SvelteSet } from "svelte/reactivity";
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { CardRoot, CardHeader, CardTitle, CardContent, CardAction } from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { ScrollArea } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { SelectMenu } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import ArtifactLink from "$lib/components/artifact/ArtifactLink.svelte";
	import { getStores } from "@orqastudio/sdk";

	const { artifactGraphSDK, toast } = getStores();
	import type { IntegrityCheck, IntegrityCategory, IntegritySeverity } from "@orqastudio/types";

	let checks = $state<IntegrityCheck[]>([]);
	let loading = $state(false);
	let scanned = $state(false);
	let error = $state<string | null>(null);

	// Filter state
	let severityFilter = $state<"all" | "Error" | "Warning" | "Info">("all");
	let categoryFilter = $state<IntegrityCategory | "all">("all");

	// Sort state
	type SortColumn = "severity" | "category" | "artifact" | "message";
	let sortColumn = $state<SortColumn>("severity");
	let sortAsc = $state(true);

	const errorCount = $derived(checks.filter((c) => c.severity === "Error").length);
	const warningCount = $derived(checks.filter((c) => c.severity === "Warning").length);
	const infoCount = $derived(checks.filter((c) => c.severity === "Info").length);


	const categoryLabels: Record<IntegrityCategory, string> = {
		BrokenLink: "Broken Links",
		TypeConstraintViolation: "Type Constraint Violations",
		RequiredRelationshipMissing: "Required Relationships Missing",
		CardinalityViolation: "Cardinality Violations",
		CircularDependency: "Circular Dependencies",
		InvalidStatus: "Invalid Statuses",
		BodyTextRefWithoutRelationship: "Body Refs Without Relationships",
		ParentChildInconsistency: "Parent-Child Inconsistencies",
		DeliveryPathMismatch: "Delivery Path Mismatches",
		MissingType: "Missing Type Field",
		MissingStatus: "Missing Status Field",
		DuplicateRelationship: "Duplicate Relationships",
		FilenameMismatch: "Filename Mismatches",
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
		if (s === "Error") return 0;
		if (s === "Warning") return 1;
		return 2; // Info
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

	// toast is available for future use (e.g., if we expose a manual rescan button)
	void toast;
</script>

{#if scanned && checks.length === 0 && !error}
	<!-- Collapsed "all clear" state — minimal footprint -->
	<div class="mb-4 flex items-center gap-2 rounded-lg border border-border px-3 py-2">
		<Icon name="shield-check" size="md" />
		<span class="text-sm text-muted-foreground">Pipeline Health</span>
		<span class="text-xs text-green-600 dark:text-green-400">All clear</span>
		{#if loading}
			<LoadingSpinner size="sm" />
		{/if}
	</div>
{:else}
<CardRoot class="gap-2">
	<CardHeader class="pb-2">
		<CardTitle class="text-sm font-semibold">
			<div class="flex items-center gap-2">
				<Icon name="shield-alert" size="md" />
				Pipeline Health
				{#if loading}
					<LoadingSpinner size="sm" />
				{/if}
			</div>
		</CardTitle>
		<!-- Error/Warning counts in Card.Action as badges -->
		{#if scanned && (errorCount > 0 || warningCount > 0)}
			<CardAction>
				<div class="flex items-center gap-1.5">
					{#if errorCount > 0}
						<Badge variant="destructive" class="text-[10px] px-1.5 py-0">
							{errorCount} Error{errorCount !== 1 ? "s" : ""}
						</Badge>
					{/if}
					{#if warningCount > 0}
						<Badge variant="warning" class="text-[10px] px-1.5 py-0">
							{warningCount} Warning{warningCount !== 1 ? "s" : ""}
						</Badge>
					{/if}
					{#if infoCount > 0}
						<Badge variant="secondary" class="text-[10px] px-1.5 py-0">
							{infoCount} Info
						</Badge>
					{/if}
				</div>
			</CardAction>
		{/if}
	</CardHeader>
	<CardContent class="pt-0">
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
			<!-- Filters: category selector left, severity pills right-aligned -->
			<div class="mb-3 flex items-center justify-between gap-3">
				<SelectMenu
					items={[
						{ value: "all", label: "All categories" },
						...presentCategories.map((cat) => ({ value: cat, label: categoryLabels[cat] })),
					]}
					selected={categoryFilter}
					onSelect={(v) => (categoryFilter = v as typeof categoryFilter)}
					triggerLabel={categoryFilter === "all" ? "All categories" : categoryLabels[categoryFilter as IntegrityCategory]}
					triggerSize="sm"
					align="start"
				/>
				<!-- Severity filter pills — active uses colored text + subtle bg, not solid fill -->
				<div class="flex items-center gap-1">
					<Button
						variant={severityFilter === "all" ? "secondary" : "ghost"}
						size="sm"
						class="h-7 px-2 text-xs {severityFilter === 'all' ? 'text-cyan-600 dark:text-cyan-400 bg-cyan-50 dark:bg-cyan-950/40 hover:bg-cyan-100 dark:hover:bg-cyan-950/60' : ''}"
						onclick={() => (severityFilter = "all")}
					>All</Button>
					<Button
						variant={severityFilter === "Error" ? "secondary" : "ghost"}
						size="sm"
						class="h-7 px-2 text-xs {severityFilter === 'Error' ? 'text-destructive bg-destructive/10 hover:bg-destructive/15' : ''}"
						onclick={() => (severityFilter = "Error")}
					>Errors</Button>
					<Button
						variant={severityFilter === "Warning" ? "secondary" : "ghost"}
						size="sm"
						class="h-7 px-2 text-xs {severityFilter === 'Warning' ? 'text-warning bg-warning/10 hover:bg-warning/15' : ''}"
						onclick={() => (severityFilter = "Warning")}
					>Warnings</Button>
					<Button
						variant={severityFilter === "Info" ? "secondary" : "ghost"}
						size="sm"
						class="h-7 px-2 text-xs {severityFilter === 'Info' ? 'text-muted-foreground bg-muted hover:bg-muted/80' : ''}"
						onclick={() => (severityFilter = "Info")}
					>Info</Button>
				</div>
			</div>

			<!-- Data table -->
			<ScrollArea class="h-64 rounded border border-border">
				<table class="w-full text-xs">
					<thead class="sticky top-0 bg-muted/80 backdrop-blur">
						<tr>
							<th class="w-8 px-2 py-1.5 text-left">
								<button
									class="flex items-center gap-0.5 text-muted-foreground hover:text-foreground"
									onclick={() => toggleSort("severity")}
								>
									<Icon name="arrow-up-down" size="xs" />
								</button>
							</th>
							<th class="px-2 py-1.5 text-right">
								<button
									class="flex items-center justify-end gap-0.5 w-full text-muted-foreground hover:text-foreground"
									onclick={() => toggleSort("category")}
								>
									Category
									{#if sortColumn === "category"}
										<Icon name="arrow-up-down" size="xs" />
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
										<Icon name="arrow-up-down" size="xs" />
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
										<Icon name="arrow-up-down" size="xs" />
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
										<Icon name="circle-alert" size="sm" />
									{:else if check.severity === "Warning"}
										<Icon name="triangle-alert" size="sm" />
									{:else}
										<Icon name="info" size="sm" />
									{/if}
								</td>
								<td class="whitespace-nowrap px-2 py-1.5 text-muted-foreground text-right">
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
			</ScrollArea>
		{/if}
	</CardContent>
</CardRoot>
{/if}
