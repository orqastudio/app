<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardContent, HStack, Stack, Text, Caption, Dot } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { artifactGraphSDK, navigationStore, pluginRegistry } = getStores();
	import { PipelineStages, type PipelineStage } from "@orqastudio/svelte-components/pure";

	/**
	 * Map a hex color to an inline style string for a dot indicator.
	 * Returns a CSS background-color inline style value.
	 * @param hex - A hex color string (e.g. "#3b82f6").
	 * @returns A Tailwind-compatible arbitrary value class or a fallback class.
	 */
	function hexToDotClass(hex: string): string {
		// Use arbitrary Tailwind value syntax so dot colors come from plugin manifests.
		return `bg-[${hex}]`;
	}

	/**
	 * Stage definitions derived from the plugin registry workflow registration.
	 * Falls back to an empty array when the lesson workflow has no pipeline_stages.
	 */
	const stageDefinitions = $derived(pluginRegistry.getPipelineStages("lesson"));

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
				// Derive a Tailwind dot color from the hex color declared in the plugin manifest.
				dotColorClass: hexToDotClass(def.color),
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
	<CardRoot full>
		<CardHeader compact>
			<CardTitle>
				<HStack gap={2}>
					<Icon name="trending-up" size="md" />
					Lesson Velocity
				</HStack>
			</CardTitle>
		</CardHeader>
		<CardContent>
			<PipelineStages
				stages={pipelineStages}
				onStageClick={(key) => navigateToLessons(key)}
			/>

			<!-- Summary text -->
			<HStack gap={1} marginTop={3}>
				{#if awaitingPromotion === 0}
					<Icon name="check-circle-2" size="sm" />
				{:else}
					<Dot size="sm" color="warning" />
				{/if}
				<Caption>{summaryText}</Caption>
			</HStack>
		</CardContent>
	</CardRoot>
{/if}
