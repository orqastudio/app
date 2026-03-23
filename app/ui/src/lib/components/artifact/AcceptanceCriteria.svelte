<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";

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
						<Icon name="square-check" size="sm" />
					{:else}
						<Icon name="square" size="sm" />
					{/if}
					<span class={isDone ? 'line-through' : ''}>{item}</span>
				</li>
			{/each}
		</ul>
	</div>
{/if}
