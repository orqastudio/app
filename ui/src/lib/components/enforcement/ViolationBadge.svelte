<script lang="ts">
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import AlertTriangleIcon from "@lucide/svelte/icons/alert-triangle";
	import { Tooltip, TooltipContent, TooltipTrigger } from "$lib/components/ui/tooltip";

	let {
		action,
		ruleName,
	}: {
		action: "Block" | "Warn";
		ruleName: string;
	} = $props();
</script>

<Tooltip>
	<TooltipTrigger>
		{#if action === "Block"}
			<span
				class="inline-flex items-center gap-1 rounded px-1.5 py-0.5 text-xs font-medium bg-destructive/15 text-destructive border border-destructive/30"
				aria-label="Blocked by rule: {ruleName}"
			>
				<ShieldIcon class="h-3 w-3 shrink-0" />
				Block
			</span>
		{:else}
			<span
				class="inline-flex items-center gap-1 rounded px-1.5 py-0.5 text-xs font-medium bg-warning/15 text-warning border border-warning/30"
				aria-label="Warning from rule: {ruleName}"
			>
				<AlertTriangleIcon class="h-3 w-3 shrink-0" />
				Warn
			</span>
		{/if}
	</TooltipTrigger>
	<TooltipContent>
		<p class="text-xs">Rule: <span class="font-mono">{ruleName}</span></p>
	</TooltipContent>
</Tooltip>
