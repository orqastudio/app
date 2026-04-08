<!-- LogSpacer — virtual height spacer for virtualised log tables.
     Its height is set to the total computed height of all rows, making the
     scrollbar reflect the full dataset size without rendering every row.
     Rows are absolutely positioned inside it via their pre-computed offsets.

     This component encapsulates the dynamic inline style (virtualiser-computed
     total height) within the library where raw HTML is permitted. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		height,
		children,
	}: {
		/** Total pixel height of all virtualised rows. Set by the virtualiser. */
		height: number;
		children?: Snippet;
	} = $props();
</script>

<div data-slot="log-spacer" class="relative w-full" style="height: {height}px;">
	{@render children?.()}
</div>
