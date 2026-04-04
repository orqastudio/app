<!--
  Layout shell for a full-height chat view.
  Slot layout: header (pinned top) → scrollable message area → input (pinned bottom).
-->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { ScrollArea } from "../scroll-area/index.js";
	import { Stack } from "../layout/index.js";

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

<Stack gap={0} align="stretch" full>
	{#if header}
		{@render header()}
	{/if}

	<div class="relative flex-1 overflow-hidden">
		<ScrollArea full>
			<div class="p-4">
				<Stack gap={4} align="stretch">
					{#if children}
						{@render children()}
					{/if}
				</Stack>
			</div>
		</ScrollArea>
	</div>

	{#if input}
		{@render input()}
	{/if}
</Stack>
