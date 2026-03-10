<script lang="ts">
	let {
		questions,
	}: {
		questions: string[];
	} = $props();

	const label = $derived(questions.length === 1 ? "Gate Question" : "Gate Questions");
</script>

{#if questions.length > 0}
	<div class="rounded border border-border bg-muted/40 px-3 py-2">
		<p class="text-xs font-medium text-muted-foreground">{label}</p>
		<div class="mt-3 space-y-1.5">
			{#each questions as question, i (i)}
				<blockquote class="border-l-2 border-muted-foreground/30 pl-3 text-sm italic text-foreground">
					"{question}"
				</blockquote>
			{/each}
		</div>
	</div>
{/if}
