<!-- ContextMenu — absolutely-positioned context action panel for row-level actions.
     Anchors to the right side of a SelectRow (right: 28px, top: 0) and floats
     above adjacent content via z-30. Used for rename/delete actions in session rows. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let { children }: { children?: Snippet } = $props();
</script>

<div
	data-slot="context-menu"
	class="border-border bg-surface-raised absolute top-0 right-7 z-30 min-w-[100px] overflow-hidden rounded-md border shadow-lg"
>
	{@render children?.()}
</div>
