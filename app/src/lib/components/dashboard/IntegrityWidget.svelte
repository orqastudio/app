<script lang="ts">
	import { SvelteSet } from "svelte/reactivity";
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { CardRoot, CardHeader, CardTitle, CardContent, CardAction } from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { ScrollArea } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { SelectMenu } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell, HStack } from "@orqastudio/svelte-components/pure";
	import { ArtifactLink } from "@orqastudio/svelte-components/connected";
	import { getStores } from "@orqastudio/sdk";

	const { artifactGraphSDK, toast } = getStores();
	import type { IntegrityCheck, IntegrityCategory, IntegritySeverity } from "@orqastudio/types";
	import { assertNever } from "@orqastudio/types";

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
				default:
					assertNever(sortColumn);
			}
			return sortAsc ? cmp : -cmp;
		});
		return sorted;
	});

	/**
	 * Returns a numeric rank for an integrity severity level for sort ordering.
	 * @param s - The severity level.
	 * @returns 0 for Error, 1 for Warning, 2 for Info.
	 */
	function severityRank(s: IntegritySeverity): number {
		if (s === "Error") return 0;
		if (s === "Warning") return 1;
		return 2; // Info
	}

	/**
	 * Toggles sort direction if the same column is clicked again, or sets the new sort column.
	 * @param col - The column to sort by.
	 */
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

	/** Refreshes the artifact graph and runs a full integrity scan, storing the resulting health snapshot. */
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
	<div class="mb-4 rounded-lg border border-border px-3 py-2">
		<HStack gap={2}>
			<Icon name="shield-check" size="md" />
			<span class="text-sm text-muted-foreground">Pipeline Health</span>
			<span class="text-xs text-success">All clear</span>
			{#if loading}
				<LoadingSpinner size="sm" />
			{/if}
		</HStack>
	</div>
{:else}
<CardRoot gap={2}>
	<CardHeader compact>
		<CardTitle size="sm">
			<HStack gap={2}>
				<Icon name="shield-alert" size="md" />
				Pipeline Health
				{#if loading}
					<LoadingSpinner size="sm" />
				{/if}
			</HStack>
		</CardTitle>
		<!-- Error/Warning counts in Card.Action as badges -->
		{#if scanned && (errorCount > 0 || warningCount > 0)}
			<CardAction>
				<HStack gap={1}>
					{#if errorCount > 0}
						<Badge variant="destructive" size="xs">
							{errorCount} Error{errorCount !== 1 ? "s" : ""}
						</Badge>
					{/if}
					{#if warningCount > 0}
						<Badge variant="warning" size="xs">
							{warningCount} Warning{warningCount !== 1 ? "s" : ""}
						</Badge>
					{/if}
					{#if infoCount > 0}
						<Badge variant="secondary" size="xs">
							{infoCount} Info
						</Badge>
					{/if}
				</HStack>
			</CardAction>
		{/if}
	</CardHeader>
	<CardContent compact>
		{#if !scanned && loading}
			<HStack justify="center">
				<LoadingSpinner />
			</HStack>
		{:else if error}
			<p class="text-sm text-destructive">{error}</p>
		{:else if !scanned}
			<p class="text-sm text-muted-foreground">
				Waiting for artifact graph...
			</p>
		{:else}
			<!-- Filters: category selector left, severity pills right-aligned -->
			<HStack gap={3} justify="between">
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
				<!-- Severity filter pills -->
				<HStack gap={1}>
					<Button
						variant={severityFilter === "all" ? "secondary" : "ghost"}
						size="sm"
						onclick={() => (severityFilter = "all")}
					>All</Button>
					<Button
						variant={severityFilter === "Error" ? "secondary" : "ghost"}
						size="sm"
						onclick={() => (severityFilter = "Error")}
					>Errors</Button>
					<Button
						variant={severityFilter === "Warning" ? "secondary" : "ghost"}
						size="sm"
						onclick={() => (severityFilter = "Warning")}
					>Warnings</Button>
					<Button
						variant={severityFilter === "Info" ? "secondary" : "ghost"}
						size="sm"
						onclick={() => (severityFilter = "Info")}
					>Info</Button>
				</HStack>
			</HStack>

			<!-- Data table -->
			<ScrollArea maxHeight="md">
				<Table>
					<TableHeader>
						<TableRow>
							<TableHead
								width="xs"
								sortable
								sorted={sortColumn === "severity" ? (sortAsc ? "asc" : "desc") : false}
								onclick={() => toggleSort("severity")}
							></TableHead>
							<TableHead
								align="right"
								sortable
								sorted={sortColumn === "category" ? (sortAsc ? "asc" : "desc") : false}
								onclick={() => toggleSort("category")}
							>Category</TableHead>
							<TableHead
								sortable
								sorted={sortColumn === "artifact" ? (sortAsc ? "asc" : "desc") : false}
								onclick={() => toggleSort("artifact")}
							>Artifact</TableHead>
							<TableHead
								sortable
								sorted={sortColumn === "message" ? (sortAsc ? "asc" : "desc") : false}
								onclick={() => toggleSort("message")}
							>Message</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{#each tableChecks as check, i (check.artifact_id + check.category + check.message + i)}
							<TableRow interactive>
								<TableCell>
									{#if check.severity === "Error"}
										<Icon name="circle-alert" size="sm" />
									{:else if check.severity === "Warning"}
										<Icon name="triangle-alert" size="sm" />
									{:else}
										<Icon name="info" size="sm" />
									{/if}
								</TableCell>
								<TableCell>
									{categoryLabels[check.category]}
								</TableCell>
								<TableCell>
									<ArtifactLink id={check.artifact_id} />
								</TableCell>
								<TableCell>
									{check.message}
									{#if check.auto_fixable}
										<span class="ml-1 text-[10px] text-success">(auto-fixable)</span>
									{/if}
								</TableCell>
							</TableRow>
						{/each}
					</TableBody>
				</Table>
			</ScrollArea>
		{/if}
	</CardContent>
</CardRoot>
{/if}
