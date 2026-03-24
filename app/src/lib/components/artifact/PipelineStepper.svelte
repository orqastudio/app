<script lang="ts">
	import { getStores } from "@orqastudio/sdk";
	import { Icon } from "@orqastudio/svelte-components/pure";

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

	const currentIndex = $derived(
		stages.findIndex((s) => s.key === status?.toLowerCase()),
	);

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
		const def = projectStore.projectSettings?.statuses?.find(
			(s) => s.key === statusKey,
		);
		return def?.transitions ?? [];
	});

	let transitioning = $state(false);

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
	<div class="mb-5">
		<!-- Row 1: circles and connector lines, vertically centered on circles -->
		<div class="flex items-center gap-0">
			{#each stages as stage, i (stage.key)}
				{@const isPast = i < currentIndex}
				{@const isCurrent = i === currentIndex}
				{@const isReachable = path && reachableKeys.includes(stage.key)}

				<!-- Connector line before this stage (not before the first) -->
				{#if i > 0}
					<div
						class="h-px flex-1 min-w-3 {i <= currentIndex
							? 'bg-primary/40'
							: 'bg-muted-foreground/15'}"
					></div>
				{/if}

				<!-- Circle indicator -->
				<div class="flex items-center justify-center">
					{#if isReachable}
						<button
							onclick={() => handleTransition(stage.key)}
							disabled={transitioning}
							title="Transition to {stage.label}"
							class="flex h-4 w-4 items-center justify-center rounded-full border border-primary/50 bg-primary/5 transition-colors hover:bg-primary/20 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-primary disabled:pointer-events-none disabled:opacity-50"
						></button>
					{:else if isPast}
						<div
							class="flex h-4 w-4 items-center justify-center rounded-full bg-primary/20"
						>
							<Icon name="check" size="md" />
						</div>
					{:else if isCurrent}
						<div
							class="flex h-5 w-5 items-center justify-center rounded-full bg-primary/15 ring-1 ring-primary/50"
						>
							<div class="h-2 w-2 rounded-full bg-primary"></div>
						</div>
					{:else}
						<div
							class="h-3.5 w-3.5 rounded-full border border-muted-foreground/20"
						></div>
					{/if}
				</div>
			{/each}
		</div>

		<!-- Row 2: labels, positioned to align under their circles -->
		<div class="mt-1 flex items-start gap-0">
			{#each stages as stage, i (stage.key)}
				{@const isPast = i < currentIndex}
				{@const isCurrent = i === currentIndex}
				{@const isReachable = path && reachableKeys.includes(stage.key)}

				<!-- Spacer matching connector line width -->
				{#if i > 0}
					<div class="flex-1 min-w-3"></div>
				{/if}

				<!-- Label only -->
				<div class="flex items-center justify-center">
					{#if isCurrent}
						<span class="text-[10px] font-semibold leading-tight whitespace-nowrap text-primary">
							{stage.label}
						</span>
					{:else if isReachable}
						<button
							onclick={() => handleTransition(stage.key)}
							disabled={transitioning}
							class="text-[9px] leading-tight whitespace-nowrap text-primary/60 underline-offset-2 hover:underline disabled:pointer-events-none disabled:opacity-50"
						>
							{stage.label}
						</button>
					{:else if isPast}
						<span class="text-[9px] leading-tight whitespace-nowrap text-muted-foreground/60">
							{stage.label}
						</span>
					{:else}
						<span class="text-[9px] leading-tight whitespace-nowrap text-muted-foreground/40">
							{stage.label}
						</span>
					{/if}
				</div>
			{/each}
		</div>
	</div>
{/if}
