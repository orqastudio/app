<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import CheckCircle2Icon from "@lucide/svelte/icons/check-circle-2";
	import ScaleIcon from "@lucide/svelte/icons/scale";
	import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";

	// -------------------------------------------------------------------------
	// Pending decisions
	// -------------------------------------------------------------------------

	interface PendingDecision {
		id: string;
		title: string;
		daysSinceCreated: number | null;
		path: string;
	}

	function daysSince(dateStr: string | null | undefined): number | null {
		if (!dateStr) return null;
		const created = new Date(dateStr);
		if (isNaN(created.getTime())) return null;
		const now = new Date();
		const diffMs = now.getTime() - created.getTime();
		return Math.floor(diffMs / (1000 * 60 * 60 * 24));
	}

	const pendingDecisions = $derived.by((): PendingDecision[] => {
		return artifactGraphSDK
			.byType("decision")
			.filter((node) => node.status === "proposed")
			.map((node) => {
				const fm = node.frontmatter as Record<string, unknown>;
				const createdField =
					typeof fm.created === "string" ? fm.created : null;
				return {
					id: node.id,
					title: node.title,
					daysSinceCreated: daysSince(createdField),
					path: node.path,
				};
			})
			.sort((a, b) => {
				// Sort by age descending (oldest first)
				const aAge = a.daysSinceCreated ?? 0;
				const bAge = b.daysSinceCreated ?? 0;
				return bAge - aAge;
			});
	});

	const hasData = $derived(artifactGraphSDK.graph.size > 0);
	const isWarning = $derived(pendingDecisions.length > 3);

	// -------------------------------------------------------------------------
	// Navigation
	// -------------------------------------------------------------------------

	function openDecision(path: string) {
		navigationStore.navigateToPath(path);
	}

	function openDecisions() {
		navigationStore.setActivity("decisions");
	}

	function dayLabel(days: number | null): string {
		if (days === null) return "";
		if (days === 0) return "today";
		if (days === 1) return "1 day ago";
		return `${days} days ago`;
	}
</script>

{#if hasData}
	<Card.Root>
		<Card.Header class="pb-3">
			<div class="flex items-center justify-between">
				<Card.Title class="text-base">
					<div class="flex items-center gap-2">
						<ScaleIcon class="h-4 w-4 text-muted-foreground" />
						Decision Queue
					</div>
				</Card.Title>
				{#if isWarning}
					<TriangleAlertIcon class="h-4 w-4 text-amber-500" />
				{/if}
			</div>
		</Card.Header>
		<Card.Content>
			{#if pendingDecisions.length === 0}
				<div class="flex items-center gap-2 py-4 text-sm text-muted-foreground">
					<CheckCircle2Icon class="h-4 w-4 text-emerald-500 shrink-0" />
					<span>No pending decisions — good to go</span>
				</div>
			{:else}
				<div class="space-y-2">
					{#each pendingDecisions as decision (decision.id)}
						<button
							class="flex w-full items-start justify-between gap-2 rounded-md px-2 py-2 text-left transition-colors hover:bg-accent/50"
							onclick={() => openDecision(decision.path)}
						>
							<div class="flex min-w-0 items-start gap-2">
								<span class="mt-1.5 inline-block h-1.5 w-1.5 shrink-0 rounded-full bg-zinc-400"></span>
								<div class="min-w-0">
									<p class="truncate text-sm font-medium">{decision.title}</p>
									<p class="text-xs text-muted-foreground">{decision.id}</p>
								</div>
							</div>
							{#if decision.daysSinceCreated !== null}
								<span class="shrink-0 text-xs text-muted-foreground tabular-nums">
									{dayLabel(decision.daysSinceCreated)}
								</span>
							{/if}
						</button>
					{/each}
				</div>

				{#if isWarning}
					<div class="mt-3 flex items-center gap-1.5 rounded-md bg-amber-50 px-2 py-1.5 text-xs text-amber-700 dark:bg-amber-950/30 dark:text-amber-400">
						<TriangleAlertIcon class="h-3.5 w-3.5 shrink-0" />
						<span>{pendingDecisions.length} decisions pending — consider accepting or superseding</span>
					</div>
				{/if}

				<button
					class="mt-2 w-full text-center text-xs text-muted-foreground underline underline-offset-2 hover:text-foreground"
					onclick={openDecisions}
				>
					View all decisions
				</button>
			{/if}
		</Card.Content>
	</Card.Root>
{/if}
