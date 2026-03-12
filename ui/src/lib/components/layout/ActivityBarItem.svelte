<script lang="ts">
	import type { Component } from "svelte";
	import * as Tooltip from "$lib/components/ui/tooltip";

	let {
		icon: Icon,
		label,
		active = false,
		onclick,
	}: {
		icon: Component;
		label: string;
		active?: boolean;
		onclick: () => void;
	} = $props();
</script>

<Tooltip.Root>
	<Tooltip.Trigger>
		{#snippet child({ props })}
			<button
				{...props}
				class="mb-1 flex h-10 w-10 items-center justify-center rounded-md transition-colors
					{active
					? 'bg-accent text-accent-foreground'
					: 'text-muted-foreground hover:bg-accent/50 hover:text-foreground'}"
				{onclick}
			>
				<Icon class="h-5 w-5" />
			</button>
		{/snippet}
	</Tooltip.Trigger>
	<Tooltip.Content side="right">
		<p>{label}</p>
	</Tooltip.Content>
</Tooltip.Root>
