<script lang="ts">
	import { getStores } from "@orqastudio/sdk";
	import { Icon } from "@orqastudio/svelte-components/pure";

	const { navigationStore } = getStores();

	let { items }: { items: string[] } = $props();

	function handleHome() {
		navigationStore.closeArtifact();
	}

	/**
	 * Navigate to an intermediate breadcrumb at the given index.
	 * items[0] is the section label, items[1..n-1] are folder segments,
	 * items[n-1] is the leaf (non-clickable). Clicking a folder segment
	 * closes the artifact viewer and returns to the list.
	 */
	function handleSegmentClick(index: number) {
		// Only the first segment (section label) has a meaningful navigation target:
		// return to the artifact list for this category.
		// Deeper folder segments don't correspond to selectable routes in the current
		// navigation model, so they also return to the list root.
		if (index < items.length - 1) {
			navigationStore.closeArtifact();
		}
	}
</script>

<nav class="flex items-center gap-1 text-sm">
	<button
		class="flex items-center text-muted-foreground hover:text-foreground"
		onclick={handleHome}
	>
		<Icon name="home" size="sm" />
	</button>

	{#each items as item, index (index)}
		<Icon name="chevron-right" size="xs" />
		{#if index === items.length - 1}
			<span class="font-medium text-foreground">{item}</span>
		{:else}
			<button
				class="text-muted-foreground hover:text-foreground"
				onclick={() => handleSegmentClick(index)}
			>
				{item}
			</button>
		{/if}
	{/each}
</nav>
