<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	let {
		left,
		center,
		right,
		class: className = "",
	}: {
		left?: Snippet;
		center?: Snippet;
		right?: Snippet;
		class?: string;
	} = $props();
</script>

<div class={cn("flex h-10 items-center gap-2 border-b border-border px-3", className)}>
	<div class="flex items-center gap-2">
		{#if left}
			{@render left()}
		{/if}
	</div>
	<div class="flex flex-1 items-center justify-center gap-2">
		{#if center}
			{@render center()}
		{/if}
	</div>
	<div class="flex items-center gap-2">
		{#if right}
			{@render right()}
		{/if}
	</div>
</div>
