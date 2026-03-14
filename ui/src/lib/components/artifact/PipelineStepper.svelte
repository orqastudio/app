<script lang="ts">
	import CheckIcon from "@lucide/svelte/icons/check";

	interface Stage {
		key: string;
		label: string;
	}

	let {
		stages,
		status,
	}: {
		stages: Stage[];
		status: string;
	} = $props();

	const currentIndex = $derived(
		stages.findIndex((s) => s.key === status?.toLowerCase()),
	);
</script>

{#if stages.length > 0 && currentIndex >= 0}
	<div class="mb-5">
		<!-- Row 1: circles and connector lines, vertically centered on circles -->
		<div class="flex items-center gap-0">
			{#each stages as stage, i (stage.key)}
				{@const isPast = i < currentIndex}
				{@const isCurrent = i === currentIndex}

				<!-- Connector line before this stage (not before the first) -->
				{#if i > 0}
					<div
						class="h-px flex-1 min-w-3 {i <= currentIndex
							? 'bg-primary/40'
							: 'bg-muted-foreground/15'}"
					></div>
				{/if}

				<!-- Circle indicator only -->
				<div class="flex items-center justify-center">
					{#if isPast}
						<div
							class="flex h-4 w-4 items-center justify-center rounded-full bg-primary/20"
						>
							<CheckIcon class="h-2.5 w-2.5 text-primary/70" />
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
