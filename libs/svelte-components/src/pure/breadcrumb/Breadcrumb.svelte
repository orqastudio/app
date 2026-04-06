<script lang="ts" module>
	export type BreadcrumbItem = string | { label: string; onClick?: () => void };

	/**
	 * Normalise a breadcrumb item to the canonical object shape with a label and optional click handler.
	 * @param item - A breadcrumb segment as either a plain string or a structured object
	 * @returns An object with a label string and an optional onClick callback
	 */
	function normalise(item: BreadcrumbItem): { label: string; onClick?: () => void } {
		return typeof item === "string" ? { label: item } : item;
	}
</script>

<script lang="ts">
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import HomeIcon from "@lucide/svelte/icons/home";

	let {
		items,
		showHome = true,
		onHome,
		maxWidth,
	}: {
		/** Breadcrumb segments — strings or {label, onClick} objects */
		items: BreadcrumbItem[];
		/** Show a home icon as first element. Default true. */
		showHome?: boolean;
		/** Callback when home icon is clicked */
		onHome?: () => void;
		/** Max width for truncation of segments (e.g. "200px") */
		maxWidth?: string;
	} = $props();

	const normalised = $derived(items.map(normalise));
</script>

<nav class="flex items-center gap-1 text-sm" aria-label="Breadcrumb">
	{#if showHome && onHome}
		<button
			class="text-muted-foreground hover:text-foreground flex items-center transition-colors"
			onclick={onHome}
		>
			<HomeIcon class="h-3.5 w-3.5" />
		</button>
		{#if normalised.length > 0}
			<ChevronRightIcon class="text-muted-foreground/60 h-3 w-3 shrink-0" />
		{/if}
	{/if}

	{#each normalised as item, i (i)}
		{#if i > 0}
			<ChevronRightIcon class="text-muted-foreground/60 h-3 w-3 shrink-0" />
		{/if}
		{#if i === normalised.length - 1}
			<span
				class="text-foreground truncate font-medium"
				style={maxWidth ? `max-width: ${maxWidth}` : undefined}
			>
				{item.label}
			</span>
		{:else if item.onClick}
			<button
				class="text-muted-foreground hover:text-foreground truncate transition-colors"
				style={maxWidth ? `max-width: ${maxWidth}` : undefined}
				onclick={item.onClick}
			>
				{item.label}
			</button>
		{:else}
			<span
				class="text-muted-foreground truncate"
				style={maxWidth ? `max-width: ${maxWidth}` : undefined}
			>
				{item.label}
			</span>
		{/if}
	{/each}
</nav>
