<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardContent } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { artifactGraphSDK, navigationStore } = getStores();
	import { PipelineStages, type PipelineStage } from "@orqastudio/svelte-components/pure";
	import { LESSON_STAGES } from "$lib/config/lesson-stages";

	// Stage definitions are sourced from config — the widget drives the pipeline visual
	// from LESSON_STAGES so the stage list is defined in one place.
	const stageDefinitions = LESSON_STAGES;

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

	/**
	 * Navigates to the lessons activity view.
	 * @param status - Optional status filter (reserved for future use; not yet applied by the view).
	 */
	function navigateToLessons(status?: string) {
		navigationStore.setActivity("lessons");
		// Status filter not yet supported by the artifact list view — navigate to lessons
		void status;
	}
</script>

{#if hasData}
	<CardRoot class="gap-2 h-full">
		<CardHeader class="pb-2">
			<CardTitle class="text-sm font-semibold">
				<div class="flex items-center gap-2">
					<Icon name="trending-up" size="md" />
					Lesson Velocity
				</div>
			</CardTitle>
		</CardHeader>
		<CardContent class="pt-0">
			<PipelineStages
				stages={pipelineStages}
				onStageClick={(key) => navigateToLessons(key)}
			/>

			<!-- Summary text -->
			<div class="mt-3 flex items-center gap-1.5 text-xs text-muted-foreground">
				{#if awaitingPromotion === 0}
					<Icon name="check-circle-2" size="sm" />
				{:else}
					<span class="inline-block h-1.5 w-1.5 rounded-full bg-amber-500"></span>
				{/if}
				<span>{summaryText}</span>
			</div>
		</CardContent>
	</CardRoot>
{/if}
