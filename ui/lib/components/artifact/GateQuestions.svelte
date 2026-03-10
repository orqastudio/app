<script lang="ts">
	let {
		questions,
	}: {
		questions: string[];
	} = $props();

	const label = $derived(questions.length === 1 ? "Gate Question" : "Gate Questions");
</script>

{#if questions.length > 0}
	<div>
		<p class="mb-2 text-xs font-medium text-muted-foreground">{label}</p>
		<blockquote class="space-y-2 border-l-2 border-muted-foreground/30 pl-3">
			{#each questions as question, i (i)}
				<p class="text-xs italic text-foreground">"{question}"</p>
			{/each}
		</blockquote>
	</div>
{/if}
