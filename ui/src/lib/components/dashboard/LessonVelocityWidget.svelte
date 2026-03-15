<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
	import CheckCircle2Icon from "@lucide/svelte/icons/check-circle-2";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import PipelineStages, { type PipelineStage } from "$lib/components/shared/PipelineStages.svelte";

	// -------------------------------------------------------------------------
	// Lesson pipeline stage definitions
	// Uses the same dot-with-center visual as PipelineWidget (dotColorClass).
	// -------------------------------------------------------------------------

	const stageDefinitions = [
		{ key: "active",    label: "Active",    dotColorClass: "bg-blue-500"   },
		{ key: "recurring", label: "Recurring", dotColorClass: "bg-amber-500"  },
		{ key: "promoted",  label: "Promoted",  dotColorClass: "bg-purple-500" },
	] as const;

	const stageCounts = $derived.by((): Record<string, number> => {
		const counts: Record<string, number> = {};
		for (const s of stageDefinitions) {
			counts[s.key] = 0;
		}
		for (const node of artifactGraphSDK.byType("lesson")) {
			const s = node.status ?? "";
			if (s in counts) {
				counts[s]++;
			}
		}
		return counts;
	});

	const pipelineStages = $derived.by((): PipelineStage[] =>
		stageDefinitions.map((def) => {
			const count = stageCounts[def.key] ?? 0;
			return {
				key: def.key,
				label: def.label,
				count,
				dotColorClass: def.dotColorClass,
				tooltipTitle: `${count} ${def.label.toLowerCase()} ${count === 1 ? "lesson" : "lessons"}`,
			};
		})
	);

	const awaitingPromotion = $derived(stageCounts["recurring"] ?? 0);

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
	<Card.Root class="gap-2 h-full">
		<Card.Header class="pb-2">
			<Card.Title class="text-sm font-semibold">
				<div class="flex items-center gap-2">
					<TrendingUpIcon class="h-4 w-4 text-muted-foreground" />
					Lesson Velocity
				</div>
			</Card.Title>
		</Card.Header>
		<Card.Content class="pt-0">
			<PipelineStages
				stages={pipelineStages}
				onStageClick={(key) => navigateToLessons(key)}
			/>

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
