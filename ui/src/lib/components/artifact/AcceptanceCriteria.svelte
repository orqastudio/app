<script lang="ts">
	import SquareIcon from "@lucide/svelte/icons/square";
	import SquareCheckIcon from "@lucide/svelte/icons/square-check";

	let {
		criteria,
		status = "",
	}: {
		criteria: string[];
		status?: string;
	} = $props();

	const isDone = $derived(status === "completed");
</script>

{#if criteria.length > 0}
	<div class="mt-3 rounded-lg border border-border bg-muted/30 px-4 py-3">
		<p class="mb-2 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
			Acceptance Criteria
		</p>
		<ul class="space-y-1.5">
			{#each criteria as item, i (i)}
				<li class="flex items-start gap-2 text-sm {isDone ? 'text-muted-foreground' : 'text-foreground'}">
					{#if isDone}
						<SquareCheckIcon class="mt-0.5 h-3.5 w-3.5 shrink-0 text-success" />
					{:else}
						<SquareIcon class="mt-0.5 h-3.5 w-3.5 shrink-0 text-muted-foreground/50" />
					{/if}
					<span class={isDone ? 'line-through' : ''}>{item}</span>
				</li>
			{/each}
		</ul>
	</div>
{/if}
