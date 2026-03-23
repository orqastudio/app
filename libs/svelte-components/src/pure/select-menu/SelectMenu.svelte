<script lang="ts">
	import {
		DropdownMenuRoot,
		DropdownMenuContent,
		DropdownMenuItem,
		DropdownMenuTrigger,
	} from "../dropdown-menu/index.js";
	import { Button } from "../button/index.js";
	import type { ButtonSize } from "../button/button.svelte";
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

	const buttonSize: ButtonSize = $derived(triggerSize === "sm" ? "sm" : "default");
</script>

<DropdownMenuRoot>
	<DropdownMenuTrigger>
		<Button variant="outline" size={buttonSize} class="gap-1 text-xs">
			{triggerLabel}
			<ChevronDownIcon class="h-3 w-3" />
		</Button>
	</DropdownMenuTrigger>
	<DropdownMenuContent {align}>
		{#each items as item (item.value)}
			<DropdownMenuItem onclick={() => onSelect(item.value)}>
				{item.label}
				{#if item.value === selected}
					<CheckIcon class="ml-auto mr-1 h-3.5 w-3.5" />
				{/if}
			</DropdownMenuItem>
		{/each}
	</DropdownMenuContent>
</DropdownMenuRoot>
