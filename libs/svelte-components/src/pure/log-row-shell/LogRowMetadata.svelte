<!-- LogRowMetadata — expanded metadata panel rendered below a log row when
     the row is clicked. Contains a scrollable code block for the raw event
     metadata JSON. Indented to align under the message column (past the
     timestamp, level, source, and category columns = 90+52+8+80+120 = 350px). -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		children,
	}: {
		children?: Snippet;
	} = $props();
</script>

<div data-slot="log-row-metadata" class="border-border bg-muted/40 border-b py-2 pl-[90px]">
	{@render children?.()}
</div>
