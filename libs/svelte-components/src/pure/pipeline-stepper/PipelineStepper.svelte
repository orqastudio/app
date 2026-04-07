<!-- Pipeline stepper — horizontal progress indicator for artifact lifecycle stages.
     Renders circles connected by lines, with clickable transitions for reachable stages.
     Library internals use raw divs for pixel-level circle geometry. -->
<script lang="ts">
	import { Icon } from "../icon/index.js";
	import { HStack, Stack } from "../layout/index.js";
	import { Panel } from "../panel/index.js";

	export interface PipelineStepperStage {
		key: string;
		label: string;
	}

	let {
		stages,
		status,
		reachableKeys,
		transitioning = false,
		onTransition,
	}: {
		stages: PipelineStepperStage[];
		/** Current status key. */
		status: string;
		/** Stage keys the current status can transition to. */
		reachableKeys: string[];
		/** Whether a transition is in progress (disables buttons). */
		transitioning?: boolean;
		/** Called when the user requests a transition to a stage key. */
		onTransition?: (key: string) => void;
	} = $props();

	const currentIndex = $derived(stages.findIndex((s) => s.key === status?.toLowerCase()));
</script>

{#if stages.length > 0 && currentIndex >= 0}
	<Panel padding="normal">
		<Stack gap={1}>
			<!-- Row 1: circles and connector lines, vertically centered on circles -->
			<HStack gap={0}>
				{#each stages as stage, i (stage.key)}
					{@const isPast = i < currentIndex}
					{@const isCurrent = i === currentIndex}
					{@const isReachable = reachableKeys.includes(stage.key)}

					<!-- Connector line before this stage (not before the first) -->
					{#if i > 0}
						<div
							class="h-px min-w-3 flex-1 {i <= currentIndex
								? 'bg-primary/40'
								: 'bg-muted-foreground/15'}"
						></div>
					{/if}

					<!-- Circle indicator -->
					<div class="flex items-center justify-center">
						{#if isReachable}
							<button
								class="border-primary/50 bg-primary/5 hover:bg-primary/20 h-4 w-4 rounded-full border p-0 disabled:pointer-events-none disabled:opacity-50"
								onclick={() => onTransition?.(stage.key)}
								disabled={transitioning}
								aria-label="Transition to {stage.label}"
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
					{@const isReachable = reachableKeys.includes(stage.key)}

					<!-- Spacer matching connector line width -->
					{#if i > 0}
						<div class="min-w-3 flex-1"></div>
					{/if}

					<!-- Label -->
					<div class="flex items-center justify-center">
						{#if isCurrent}
							<span class="text-primary text-[10px] leading-tight font-semibold whitespace-nowrap">
								{stage.label}
							</span>
						{:else if isReachable}
							<button
								class="text-primary/60 text-[9px] leading-tight whitespace-nowrap hover:underline disabled:pointer-events-none disabled:opacity-50"
								onclick={() => onTransition?.(stage.key)}
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
