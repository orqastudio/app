<script lang="ts">
	import { resolveStatus, statusColorClass, type StatusConfig } from "./status-utils.js";
	import { resolveIcon } from "../icon/icon-utils.js";

	let {
		status,
		statuses,
		mode = "badge",
	}: {
		/** Status key to render (e.g. "active", "completed", "blocked") */
		status: string;
		/** Custom status config — overrides defaults */
		statuses?: StatusConfig[];
		/** Rendering mode */
		mode?: "badge" | "dot" | "inline";
	} = $props();

	const config = $derived(resolveStatus(status, statuses));
	const Icon = $derived(resolveIcon(config.icon));
	const colorClass = $derived(statusColorClass(status, statuses));
	const isSpinning = $derived(config.spin === true);
</script>

{#if mode === "dot"}
	<Icon class="inline-block h-3.5 w-3.5 shrink-0 {colorClass} {isSpinning ? 'status-spin' : ''}" />
{:else if mode === "inline"}
	<span class="inline-flex items-center gap-1 text-xs {colorClass}">
		<Icon class="h-3.5 w-3.5 shrink-0 {isSpinning ? 'status-spin' : ''}" />
		<span>{config.label}</span>
	</span>
{:else}
	<span
		class="border-border bg-muted/30 text-muted-foreground inline-flex items-center gap-1.5 rounded border px-1.5 py-0.5 text-xs"
	>
		<Icon class="h-3 w-3 shrink-0 {colorClass} {isSpinning ? 'status-spin' : ''}" />{config.label}
	</span>
{/if}

<style>
	:global(.status-spin) {
		animation: status-spin 4s linear infinite;
	}
	@keyframes status-spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
</style>
