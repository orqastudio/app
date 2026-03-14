<script lang="ts">
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import HomeIcon from "@lucide/svelte/icons/home";

	type BreadcrumbItem = {
		label: string;
		onClick: () => void;
	};

	let { items }: { items: BreadcrumbItem[] } = $props();
</script>

<nav class="flex items-center gap-1 text-sm" aria-label="Roadmap navigation">
	{#each items as item, i (i)}
		{#if i > 0}
			<ChevronRightIcon class="h-3.5 w-3.5 shrink-0 text-muted-foreground/60" />
		{/if}
		{#if i === items.length - 1}
			<span class="font-medium text-foreground truncate max-w-[240px]">{item.label}</span>
		{:else}
			<button
				class="flex items-center gap-1 text-muted-foreground hover:text-foreground transition-colors truncate max-w-[200px]"
				onclick={item.onClick}
			>
				{#if i === 0}
					<HomeIcon class="h-3.5 w-3.5 shrink-0" />
				{/if}
				<span>{item.label}</span>
			</button>
		{/if}
	{/each}
</nav>
