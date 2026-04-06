<script lang="ts" module>
	import type { Snippet } from "svelte";

	export type DropdownMenuItem = {
		label: string;
		onclick: () => void;
		icon?: import("svelte").Component;
		disabled?: boolean;
		destructive?: boolean;
	};

	export type DropdownMenuSeparator = { separator: true };

	export type DropdownMenuEntry = DropdownMenuItem | DropdownMenuSeparator;

	/**
	 * Type guard that narrows a dropdown menu entry to a separator.
	 * @param entry - A menu entry that may be either an action item or a separator
	 * @returns True when the entry is a separator, narrowing the type accordingly
	 */
	function isSeparator(entry: DropdownMenuEntry): entry is DropdownMenuSeparator {
		return "separator" in entry;
	}

	type DropdownMenuBaseProps = {
		align?: "start" | "center" | "end";
		side?: "top" | "bottom" | "left" | "right";
		trigger: Snippet<[{ props: Record<string, unknown> }]>;
	};

	type DropdownMenuWithItems = DropdownMenuBaseProps & {
		items: DropdownMenuEntry[];
		children?: never;
	};

	type DropdownMenuWithContent = DropdownMenuBaseProps & {
		children: Snippet;
		items?: never;
	};

	export type DropdownMenuProps = DropdownMenuWithItems | DropdownMenuWithContent;
</script>

<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";
	import DropdownMenuContent from "./dropdown-menu-content.svelte";
	import DropdownMenuItemComponent from "./dropdown-menu-item.svelte";
	import DropdownMenuSeparatorComponent from "./dropdown-menu-separator.svelte";

	let { align = "end", side = "bottom", trigger, items, children }: DropdownMenuProps = $props();
</script>

<DropdownMenuPrimitive.Root>
	<DropdownMenuPrimitive.Trigger>
		{#snippet child({ props })}
			{@render trigger({ props })}
		{/snippet}
	</DropdownMenuPrimitive.Trigger>
	<DropdownMenuContent {align} {side}>
		{#if children}
			{@render children()}
		{:else if items}
			{#each items as entry, i (i)}
				{#if isSeparator(entry)}
					<DropdownMenuSeparatorComponent />
				{:else}
					<DropdownMenuItemComponent
						onclick={entry.onclick}
						disabled={entry.disabled}
						variant={entry.destructive ? "destructive" : "default"}
					>
						{#if entry.icon}
							<entry.icon class="h-4 w-4" />
						{/if}
						{entry.label}
					</DropdownMenuItemComponent>
				{/if}
			{/each}
		{/if}
	</DropdownMenuContent>
</DropdownMenuPrimitive.Root>
