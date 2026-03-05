<script lang="ts">
	import type { GovernanceArea } from "$lib/types/governance";

	interface Props {
		areas: GovernanceArea[];
		coverageRatio: number;
	}

	const { areas, coverageRatio }: Props = $props();

	const coveredCount = $derived(areas.filter((a) => a.covered).length);
	const totalCount = $derived(areas.length);

	const coveragePercent = $derived(Math.round(coverageRatio * 100));
</script>

<div class="space-y-2">
	<div class="flex items-center justify-between text-sm">
		<span class="font-medium">Governance Coverage</span>
		<span class="text-muted-foreground">{coveredCount} of {totalCount} areas covered</span>
	</div>

	<div class="flex gap-1">
		{#each areas as area (area.name)}
			<div
				class="h-2 flex-1 rounded-sm transition-colors {area.covered
					? 'bg-green-500'
					: 'bg-muted'}"
				title="{area.name}: {area.covered ? 'covered' : 'not covered'}"
			></div>
		{/each}
	</div>

	<p class="text-xs text-muted-foreground">{coveragePercent}% coverage</p>
</div>
