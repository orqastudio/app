<!-- BackgroundImage — full-coverage background image container.
     Fills its positioned parent with an absolutely-positioned background image layer.
     Use to layer a decorative image beneath content without polluting structural components.
     The content slot renders above the image via a relative-positioned wrapper. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		src,
		overlay = false,
		children,
	}: {
		/** URL of the background image. */
		src: string;
		/** When true, renders a semi-transparent dark overlay above the image to improve text legibility. */
		overlay?: boolean;
		children?: Snippet;
	} = $props();
</script>

<div
	class="relative flex-1 overflow-hidden"
	style:background-image="url({src})"
	style:background-size="cover"
	style:background-position="center"
>
	{#if overlay}
		<div class="bg-background/70 absolute inset-0"></div>
	{/if}
	<div class="relative z-10 flex h-full w-full items-center justify-center px-4">
		{@render children?.()}
	</div>
</div>
