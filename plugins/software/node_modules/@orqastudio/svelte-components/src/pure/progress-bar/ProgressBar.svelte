<script lang="ts">
	let {
		label,
		current,
		total,
		colorClass = "bg-primary",
	}: {
		label: string;
		current: number;
		total: number;
		colorClass?: string;
	} = $props();

	const percentage = $derived(total > 0 ? Math.round((current / total) * 100) : 0);
</script>

<div class="flex flex-col gap-1">
	<div class="flex items-center justify-between text-xs">
		<span class="text-muted-foreground">{label}</span>
		<span class="tabular-nums text-foreground">{current}/{total}</span>
	</div>
	<div class="h-1.5 w-full rounded-full bg-muted">
		<div
			class="h-full rounded-full transition-all {colorClass}"
			style="width: {percentage}%"
		></div>
	</div>
</div>
