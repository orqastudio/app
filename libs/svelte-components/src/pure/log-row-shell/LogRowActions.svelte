<!-- LogRowActions — absolute-positioned action buttons slot for virtualised log
     rows. Renders at the right edge of the row and is hidden until the row is
     hovered. Uses Tailwind's group-hover pattern; the row shell must have the
     "group" class (LogRowShell already applies it).

     Typically used to host a compact copy-to-clipboard button. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		children,
	}: {
		children?: Snippet;
	} = $props();
</script>

<!-- hidden by default; flex on group (row) hover. Height matches 24px log rows. -->
<span class="absolute top-0 right-1 hidden h-6 items-center group-hover:flex">
	{@render children?.()}
</span>
