<script lang="ts">
	import { Icon } from "../icon/index.js";
	import { SmallBadge } from "../small-badge/index.js";
	import { HStack } from "../layout/index.js";
	import { Text } from "../typography/index.js";
	import type { BadgeVariant } from "../badge/index.js";

	let {
		icon,
		label,
		items,
		badgeVariant = "secondary",
	}: {
		icon: string;
		label: string;
		items: string[];
		badgeVariant?: BadgeVariant;
	} = $props();
</script>

{#if items.length > 0}
	<HStack gap={1.5} wrap>
		<HStack gap={1}>
			<Icon name={icon} size="sm" />
			<Text variant="caption">{label}</Text>
		</HStack>
		{#each items as item, i (i)}
			<SmallBadge variant={badgeVariant}>{item}</SmallBadge>
		{/each}
	</HStack>
{/if}
