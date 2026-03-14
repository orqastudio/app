<script lang="ts">
	import CheckIcon from "@lucide/svelte/icons/check";
	import { statusColor } from "$lib/components/shared/StatusIndicator.svelte";

	let {
		artifactType,
		status,
	}: {
		artifactType: string;
		status: string;
	} = $props();

	/** Lifecycle stages per artifact type. */
	const LIFECYCLE_STAGES: Record<string, string[]> = {
		task: ["todo", "in-progress", "done"],
		epic: ["draft", "ready", "in-progress", "review", "done"],
		idea: ["captured", "exploring", "shaped", "promoted"],
		milestone: ["planning", "active", "complete"],
		decision: ["proposed", "accepted", "superseded"],
		lesson: ["active", "recurring", "promoted"],
		research: ["draft", "complete", "surpassed"],
	};

	const stages = $derived(LIFECYCLE_STAGES[artifactType.toLowerCase()] ?? []);

	const currentIndex = $derived(
		stages.findIndex((s) => s === status?.toLowerCase()),
	);

	/** Humanize a stage name for display. */
	function humanizeStage(stage: string): string {
		return stage
			.replace(/-/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}
</script>

{#if stages.length > 0 && currentIndex >= 0}
	<div class="mb-4 flex items-center gap-1">
		{#each stages as stage, i (stage)}
			{@const isPast = i < currentIndex}
			{@const isCurrent = i === currentIndex}

			<!-- Stage dot + label -->
			<div class="flex flex-col items-center gap-1">
				<div class="flex items-center">
					{#if isPast}
						<div
							class="flex h-5 w-5 items-center justify-center rounded-full bg-emerald-500"
						>
							<CheckIcon class="h-3 w-3 text-white" />
						</div>
					{:else if isCurrent}
						<div
							class="flex h-5 w-5 items-center justify-center rounded-full ring-2 ring-offset-1 ring-offset-background {statusColor(status)}"
						>
							<div class="h-2.5 w-2.5 rounded-full {statusColor(status)}"></div>
						</div>
					{:else}
						<div
							class="h-5 w-5 rounded-full border-2 border-muted-foreground/30"
						></div>
					{/if}
				</div>
				<span
					class="text-[10px] leading-tight whitespace-nowrap {isCurrent
						? 'font-semibold text-foreground'
						: isPast
							? 'text-muted-foreground'
							: 'text-muted-foreground/50'}"
				>
					{humanizeStage(stage)}
				</span>
			</div>

			<!-- Connector line between stages -->
			{#if i < stages.length - 1}
				<div
					class="mt-[-16px] h-0.5 min-w-4 flex-1 {i < currentIndex
						? 'bg-emerald-500'
						: 'bg-muted-foreground/20'}"
				></div>
			{/if}
		{/each}
	</div>
{/if}
