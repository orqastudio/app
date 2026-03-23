<script lang="ts">
	import { Icon } from "../icon/index.js";
	import { SmallBadge } from "../small-badge/index.js";
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
	<div class="flex flex-wrap items-center gap-1.5">
		<span class="flex items-center gap-1 text-xs text-muted-foreground">
			<Icon name={icon} size="sm" />
			{label}
		</span>
		{#each items as item, i (i)}
			<SmallBadge variant={badgeVariant}>{item}</SmallBadge>
		{/each}
	</div>
{/if}
