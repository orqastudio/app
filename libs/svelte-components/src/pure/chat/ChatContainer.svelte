<!--
  Layout shell for a full-height chat view.
  Slot layout: header (pinned top) → scrollable message area → input (pinned bottom).
-->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { ScrollArea } from "../scroll-area/index.js";

	let {
		header,
		input,
		children,
	}: {
		/** Optional header bar pinned to the top of the container. */
		header?: Snippet;
		/** Input area pinned to the bottom of the container. */
		input?: Snippet;
		/** Scrollable message list content. */
		children?: Snippet;
	} = $props();
</script>

<div class="flex h-full flex-col">
	{#if header}
		{@render header()}
	{/if}

	<div class="relative flex-1 overflow-hidden">
		<ScrollArea full>
			<div class="space-y-4 p-4">
				{#if children}
					{@render children()}
				{/if}
			</div>
		</ScrollArea>
	</div>

	{#if input}
		{@render input()}
	{/if}
</div>
