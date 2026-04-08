<!-- TwoByTwoGrid — a 2-column 2-row CSS grid that fills its parent's height.
     Used for dashboard metric panels where exactly four cells must fill the
     available space with no gap. Children are responsible for their own borders
     via MetricGridCell. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		children,
	}: {
		children?: Snippet;
	} = $props();
</script>

<div class="grid h-full grid-cols-2 grid-rows-2 overflow-hidden">
	{@render children?.()}
</div>
