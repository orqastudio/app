<script lang="ts">
	import { Icon } from "../icon/index.js";
	import { Button } from "../button/index.js";
	import { Stack, Center } from "../layout/index.js";
	import { Text } from "../typography/index.js";

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

<Center>
	<Stack align="center" gap={2}>
		{#if icon}
			<Icon name={icon} size="xl" />
		{/if}
		<Text variant="heading-sm">{title}</Text>
		{#if description}
			<Text variant="caption">{description}</Text>
		{/if}
		{#if action}
			<Button variant="default" size="sm" onclick={action.onclick}>
				{action.label}
			</Button>
		{/if}
	</Stack>
</Center>
