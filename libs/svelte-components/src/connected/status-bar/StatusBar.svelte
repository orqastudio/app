<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		left,
		center,
		right,
	}: {
		left?: Snippet;
		center?: Snippet;
		right?: Snippet;
	} = $props();
</script>

<div
	class="border-border bg-muted/30 text-muted-foreground flex h-8 items-center border-t px-4 pb-1 text-xs"
>
	<div class="flex items-center gap-3">
		{#if left}
			{@render left()}
		{/if}
	</div>
	<div class="flex-1">
		{#if center}
			{@render center()}
		{/if}
	</div>
	<div class="flex items-center gap-3">
		{#if right}
			{@render right()}
		{/if}
	</div>
</div>
