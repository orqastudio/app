<script lang="ts">
	import StatusIndicator from "../status-indicator/StatusIndicator.svelte";
	import { cn } from "../../utils/cn.js";
	import { getStores } from "@orqastudio/sdk";

	const { navigationStore } = getStores();

	let {
		label,
		description,
		status,
		badge,
		path,
		active = false,
		onclick,
	}: {
		label: string;
		description?: string;
		status?: string;
		badge?: string;
		/** Artifact path — when provided, active state is derived internally. */
		path?: string;
		/** Manual active override — only used when path is not provided. */
		active?: boolean;
		onclick: () => void;
	} = $props();

	// Derive active from the navigation store when path is provided.
	// Each instance derives its own active state independently —
	// only the item whose active state actually CHANGES re-renders.
	const isActive = $derived(path ? navigationStore.selectedArtifactPath === path : active);
</script>

<button
	class={cn(
		"hover:bg-accent/50 flex w-full flex-col gap-0.5 rounded px-2 py-1.5 text-left",
		isActive && "bg-accent",
	)}
	{onclick}
>
	<span class="flex items-center gap-1.5 truncate text-sm font-medium">
		{#if status}
			<StatusIndicator {status} mode="dot" />
		{:else if badge}
			<span
				class="bg-muted text-muted-foreground shrink-0 rounded px-1 py-0.5 text-[10px] font-normal"
				>{badge}</span
			>
		{/if}
		<span class="truncate">{label}</span>
	</span>
	{#if description}
		<p class="text-muted-foreground truncate text-xs">{description}</p>
	{/if}
</button>
