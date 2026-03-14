<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
	import CheckCircle2Icon from "@lucide/svelte/icons/check-circle-2";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";

	// -------------------------------------------------------------------------
	// Lesson pipeline stage counts
	// -------------------------------------------------------------------------

	interface LessonStage {
		key: string;
		label: string;
		status: string;
		dotClass: string;
	}

	const stages: LessonStage[] = [
		{
			key: "active",
			label: "Active",
			status: "active",
			dotClass: "bg-blue-500",
		},
		{
			key: "recurring",
			label: "Recurring",
			status: "recurring",
			dotClass: "bg-amber-500",
		},
		{
			key: "promoted",
			label: "Promoted",
			status: "promoted",
			dotClass: "bg-purple-500",
		},
	];

	const stageCounts = $derived.by((): Record<string, number> => {
		const counts: Record<string, number> = {};
		for (const stage of stages) {
			counts[stage.key] = 0;
		}
		for (const node of artifactGraphSDK.byType("lesson")) {
			const s = node.status ?? "";
			if (s in counts) {
				counts[s]++;
			}
		}
		return counts;
	});

	const awaitingPromotion = $derived(
		(stageCounts["recurring"] ?? 0)
	);

	const hasData = $derived(artifactGraphSDK.graph.size > 0);

	const summaryText = $derived.by((): string => {
		if (awaitingPromotion === 0) {
			const promoted = stageCounts["promoted"] ?? 0;
			if (promoted > 0) return "All recurring lessons promoted";
			return "No lessons awaiting promotion";
		}
		return `${awaitingPromotion} ${awaitingPromotion === 1 ? "lesson" : "lessons"} awaiting promotion`;
	});

	// -------------------------------------------------------------------------
	// Navigation
	// -------------------------------------------------------------------------

	function navigateToLessons(status?: string) {
		navigationStore.setActivity("lessons");
		// Status filter not yet supported by the artifact list view — navigate to lessons
		void status;
	}
</script>

{#if hasData}
	<Card.Root>
		<Card.Header class="pb-3">
			<Card.Title class="text-base">
				<div class="flex items-center gap-2">
					<TrendingUpIcon class="h-4 w-4 text-muted-foreground" />
					Lesson Velocity
				</div>
			</Card.Title>
		</Card.Header>
		<Card.Content>
			<!-- Horizontal stage row -->
			<div class="flex items-stretch gap-2">
				{#each stages as stage, i (stage.key)}
					<!-- Stage pill -->
					<Tooltip.Root>
						<Tooltip.Trigger>
							{#snippet child({ props })}
								<button
									{...props}
									class="flex flex-1 flex-col items-center gap-1.5 rounded-lg border border-border bg-muted/30 px-3 py-3 transition-colors hover:bg-accent/50"
									onclick={() => navigateToLessons(stage.status)}
								>
									<span class="flex h-2.5 w-2.5 rounded-full {stage.dotClass}"></span>
									<span class="text-xs font-medium text-foreground">{stage.label}</span>
									<span class="text-lg font-semibold tabular-nums text-foreground">
										{stageCounts[stage.key] ?? 0}
									</span>
								</button>
							{/snippet}
						</Tooltip.Trigger>
						<Tooltip.Content side="bottom">
							<p class="text-xs">{stageCounts[stage.key] ?? 0} {stage.label.toLowerCase()} {stageCounts[stage.key] === 1 ? "lesson" : "lessons"}</p>
						</Tooltip.Content>
					</Tooltip.Root>

					<!-- Arrow connector between stages -->
					{#if i < stages.length - 1}
						<div class="flex shrink-0 items-center text-muted-foreground/40">
							<svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
								<polyline points="4,4 10,8 4,12" stroke="currentColor" stroke-width="1.5" fill="none" />
							</svg>
						</div>
					{/if}
				{/each}
			</div>

			<!-- Summary text -->
			<div class="mt-3 flex items-center gap-1.5 text-xs text-muted-foreground">
				{#if awaitingPromotion === 0}
					<CheckCircle2Icon class="h-3.5 w-3.5 text-emerald-500" />
				{:else}
					<span class="inline-block h-1.5 w-1.5 rounded-full bg-amber-500"></span>
				{/if}
				<span>{summaryText}</span>
			</div>
		</Card.Content>
	</Card.Root>
{/if}
