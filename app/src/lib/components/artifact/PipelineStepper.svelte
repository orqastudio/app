<!-- Renders a horizontal pipeline progress indicator for artifact lifecycle stages.
     The circle and connector-line elements use raw divs because their specific pixel sizes
     (h-px, h-4, w-4, rounded-full, etc.) are not expressible via ORQA layout primitives. -->
<script lang="ts">
	import { getStores } from "@orqastudio/sdk";
	import { Icon, HStack, Stack, Panel } from "@orqastudio/svelte-components/pure";

	const { artifactGraphSDK, projectStore } = getStores();

	interface Stage {
		key: string;
		label: string;
	}

	let {
		stages,
		status,
		path = "",
	}: {
		stages: Stage[];
		status: string;
		/** Relative path from project root — required for status transitions. */
		path?: string;
	} = $props();

	const currentIndex = $derived(stages.findIndex((s) => s.key === status?.toLowerCase()));

	/**
	 * Keys reachable from the current status — driven by the `transitions` array
	 * on the matching status definition in project config.
	 *
	 * Falls back to an empty array when config is absent or the current status
	 * has no defined transitions, preventing stale hardcoded maps from
	 * diverging from the project's actual workflow.
	 */
	const reachableKeys = $derived.by((): string[] => {
		const statusKey = status?.toLowerCase();
		if (!statusKey) return [];
		const def = projectStore.projectSettings?.statuses?.find((s) => s.key === statusKey);
		return def?.transitions ?? [];
	});

	let transitioning = $state(false);

	/**
	 * Trigger a status transition to the given target key via the artifact graph SDK.
	 * @param targetKey - The status key to transition the current artifact to.
	 */
	async function handleTransition(targetKey: string) {
		if (!path || transitioning) return;
		transitioning = true;
		try {
			await artifactGraphSDK.updateField(path, "status", targetKey);
		} finally {
			transitioning = false;
		}
	}
</script>

{#if stages.length > 0 && currentIndex >= 0}
	<Panel padding="normal">
		<Stack gap={1}>
			<!-- Row 1: circles and connector lines, vertically centered on circles -->
			<HStack gap={0}>
				{#each stages as stage, i (stage.key)}
					{@const isPast = i < currentIndex}
					{@const isCurrent = i === currentIndex}
					{@const isReachable = path && reachableKeys.includes(stage.key)}

					<!-- Connector line before this stage (not before the first) -->
					{#if i > 0}
						<div
							class="h-px min-w-3 flex-1 {i <= currentIndex
								? 'bg-primary/40'
								: 'bg-muted-foreground/15'}"
						></div>
					{/if}

					<!-- Circle indicator — specific sizes (h-4, w-4, rounded-full) require raw divs. -->
					<div class="flex items-center justify-center">
						{#if isReachable}
							<button
								class="border-primary/50 bg-primary/5 hover:bg-primary/20 h-4 w-4 rounded-full border p-0 disabled:pointer-events-none disabled:opacity-50"
								onclick={() => handleTransition(stage.key)}
								disabled={transitioning}
							></button>
						{:else if isPast}
							<div class="bg-primary/20 flex h-4 w-4 items-center justify-center rounded-full">
								<Icon name="check" size="md" />
							</div>
						{:else if isCurrent}
							<div
								class="bg-primary/15 ring-primary/50 flex h-5 w-5 items-center justify-center rounded-full ring-1"
							>
								<div class="bg-primary h-2 w-2 rounded-full"></div>
							</div>
						{:else}
							<div class="border-muted-foreground/20 h-3.5 w-3.5 rounded-full border"></div>
						{/if}
					</div>
				{/each}
			</HStack>

			<!-- Row 2: labels, positioned to align under their circles -->
			<HStack gap={0} align="start">
				{#each stages as stage, i (stage.key)}
					{@const isPast = i < currentIndex}
					{@const isCurrent = i === currentIndex}
					{@const isReachable = path && reachableKeys.includes(stage.key)}

					<!-- Spacer matching connector line width -->
					{#if i > 0}
						<div class="min-w-3 flex-1"></div>
					{/if}

					<!-- Label only -->
					<div class="flex items-center justify-center">
						{#if isCurrent}
							<span class="text-primary text-[10px] leading-tight font-semibold whitespace-nowrap">
								{stage.label}
							</span>
						{:else if isReachable}
							<button
								class="text-primary/60 text-[9px] leading-tight whitespace-nowrap hover:underline disabled:pointer-events-none disabled:opacity-50"
								onclick={() => handleTransition(stage.key)}
								disabled={transitioning}
							>
								{stage.label}
							</button>
						{:else if isPast}
							<span class="text-muted-foreground/60 text-[9px] leading-tight whitespace-nowrap">
								{stage.label}
							</span>
						{:else}
							<span class="text-muted-foreground/40 text-[9px] leading-tight whitespace-nowrap">
								{stage.label}
							</span>
						{/if}
					</div>
				{/each}
			</HStack>
		</Stack>
	</Panel>
{/if}
