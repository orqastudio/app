<!-- Centers content both horizontally and vertically using flexbox. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		full = false,
		children,
	}: {
		/** When true, expands to fill the full height of the parent. */
		full?: boolean;
		children?: Snippet;
	} = $props();
</script>

<div class="flex items-center justify-center {full ? 'h-full' : ''}">
	{@render children?.()}
</div>
