<script lang="ts">
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import HomeIcon from "@lucide/svelte/icons/home";
	import { navigationStore } from "$lib/stores/navigation.svelte";

	let { items }: { items: string[] } = $props();

	function handleClick(index: number) {
		if (index === 0) {
			navigationStore.closeArtifact();
		}
	}
</script>

<nav class="flex items-center gap-1 text-sm">
	<button
		class="flex items-center text-muted-foreground hover:text-foreground"
		onclick={() => handleClick(0)}
	>
		<HomeIcon class="h-3.5 w-3.5" />
	</button>

	{#each items as item, index}
		<ChevronRightIcon class="h-3 w-3 text-muted-foreground" />
		{#if index === items.length - 1}
			<span class="font-medium text-foreground">{item}</span>
		{:else}
			<button
				class="text-muted-foreground hover:text-foreground"
				onclick={() => handleClick(index)}
			>
				{item}
			</button>
		{/if}
	{/each}
</nav>
