<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		logo,
		menuBar,
		right,
	}: {
		logo?: Snippet;
		menuBar?: Snippet;
		right?: Snippet;
	} = $props();
</script>

<div class="toolbar-drag relative z-50 flex h-10 items-center border-b border-border bg-background">
	{#if logo}
		<div class="flex h-10 w-12 shrink-0 items-center justify-center border-r border-border">
			{@render logo()}
		</div>
	{/if}

	{#if menuBar}
		{@render menuBar()}
	{/if}

	<div class="flex-1"></div>

	{#if right}
		{@render right()}
	{/if}
</div>

<style>
	.toolbar-drag {
		-webkit-app-region: drag;
	}
	.toolbar-drag :global(button) {
		cursor: default;
		-webkit-app-region: no-drag;
	}
</style>
