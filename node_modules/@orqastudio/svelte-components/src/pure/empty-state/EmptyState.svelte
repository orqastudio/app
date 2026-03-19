<script lang="ts">
	import { Icon } from "../icon/index.js";
	import { Button } from "../button/index.js";

	let {
		icon,
		title,
		description,
		action,
	}: {
		icon?: string;
		title: string;
		description?: string;
		action?: { label: string; onclick: () => void };
	} = $props();
</script>

<div class="flex flex-col items-center justify-center py-12 text-center">
	{#if icon}
		<div class="mb-4"><Icon name={icon} size="xl" /></div>
	{/if}
	<h3 class="text-lg font-semibold">{title}</h3>
	{#if description}
		<p class="mt-1 text-sm text-muted-foreground">{description}</p>
	{/if}
	{#if action}
		<Button variant="default" size="sm" class="mt-4" onclick={action.onclick}>
			{action.label}
		</Button>
	{/if}
</div>
