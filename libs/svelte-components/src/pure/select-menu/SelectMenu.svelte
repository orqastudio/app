<script lang="ts">
	import {
		DropdownMenuRoot,
		DropdownMenuContent,
		DropdownMenuItem,
		DropdownMenuTrigger,
	} from "../dropdown-menu/index.js";
	import CheckIcon from "@lucide/svelte/icons/check";
	import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";

	let {
		items,
		selected,
		onSelect,
		triggerLabel,
		triggerSize = "sm",
		align = "end",
	}: {
		items: Array<{ value: string; label: string }>;
		selected: string;
		onSelect: (value: string) => void;
		triggerLabel: string;
		triggerSize?: "sm" | "default";
		align?: "start" | "end";
	} = $props();

	// Inline trigger classes: outline button, small size, compact gap, xs text.
	const triggerClass = $derived(
		triggerSize === "sm"
			? "focus-visible:border-ring focus-visible:ring-ring/50 inline-flex shrink-0 items-center justify-center gap-1 rounded-md border bg-background px-3 text-xs font-medium shadow-xs transition-all outline-none hover:bg-accent hover:text-accent-foreground focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 h-8"
			: "focus-visible:border-ring focus-visible:ring-ring/50 inline-flex shrink-0 items-center justify-center gap-1 rounded-md border bg-background px-4 text-sm font-medium shadow-xs transition-all outline-none hover:bg-accent hover:text-accent-foreground focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 h-9",
	);
</script>

<DropdownMenuRoot>
	<DropdownMenuTrigger class={triggerClass}>
		{triggerLabel}
		<ChevronDownIcon class="h-3 w-3" />
	</DropdownMenuTrigger>
	<DropdownMenuContent {align}>
		{#each items as item (item.value)}
			<DropdownMenuItem onclick={() => onSelect(item.value)}>
				{item.label}
				{#if item.value === selected}
					<CheckIcon class="mr-1 ml-auto h-3.5 w-3.5" />
				{/if}
			</DropdownMenuItem>
		{/each}
	</DropdownMenuContent>
</DropdownMenuRoot>
