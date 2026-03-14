<script lang="ts">
	import SquareIcon from "@lucide/svelte/icons/square";

	let {
		criteria,
	}: {
		criteria: string[];
	} = $props();
</script>

{#if criteria.length > 0}
	<div class="mt-6 rounded-lg border border-border bg-muted/30 px-4 py-3">
		<p class="mb-2 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
			Acceptance Criteria
		</p>
		<ul class="space-y-1.5">
			{#each criteria as item, i (i)}
				<li class="flex items-start gap-2 text-sm text-foreground">
					<SquareIcon class="mt-0.5 h-3.5 w-3.5 shrink-0 text-muted-foreground/50" />
					<span>{item}</span>
				</li>
			{/each}
		</ul>
	</div>
{/if}
