<!-- Callout — inline tonal banner, a semantic lego block.

Used for short, contextual messages placed next to content: warnings, info
hints, success confirmations, and promoted/archived annotations. Not a modal,
not a dialog, not a full error screen — those exist separately (AlertDialog,
ErrorDisplay). Callout is the thing you reach for when a row of text needs a
tinted background and an optional icon.

All visual state is closed-set. Tone, density, and border style are variants —
no free colors, paddings, or radii. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import type { Component } from "svelte";
	import { cn } from "../../utils/cn.js";
	import { Icon } from "../icon/index.js";

	// Tone maps to design token families. Each tone picks a matched
	// background/border/text triple from the palette.
	const toneMap: Record<string, string> = {
		info: "bg-info/10 border-info/30 text-info",
		warning: "bg-warning/10 border-warning/30 text-warning",
		success: "bg-success/10 border-success/30 text-success",
		destructive: "bg-destructive/10 border-destructive/30 text-destructive",
		muted: "bg-muted border-border text-muted-foreground",
	};

	// Density maps to padding presets. Compact is for inline row hints; normal
	// for standalone banners.
	const densityMap: Record<string, string> = {
		compact: "px-2 py-1",
		normal: "px-3 py-2",
	};

	// Border style — solid is the default; dashed communicates a soft warning
	// or a placeholder/unavailable state.
	const borderStyleMap: Record<string, string> = {
		solid: "border",
		dashed: "border border-dashed",
	};

	let {
		tone = "info",
		density = "normal",
		border = "solid",
		icon,
		iconName,
		children,
		role,
		"aria-label": ariaLabel,
	}: {
		/** Tone variant — picks background, border colour and text colour together. */
		tone?: "info" | "warning" | "success" | "destructive" | "muted";
		/** Padding density. */
		density?: "compact" | "normal";
		/** Border style. */
		border?: "solid" | "dashed";
		/** Custom icon component (Lucide or compatible). Overrides iconName. */
		icon?: Component;
		/** Named icon via the design system icon map. */
		iconName?: string;
		children?: Snippet;
		role?: string;
		"aria-label"?: string;
	} = $props();

	const toneClass = $derived(toneMap[tone] ?? toneMap.info);
	const densityClass = $derived(densityMap[density] ?? densityMap.normal);
	const borderClass = $derived(borderStyleMap[border] ?? borderStyleMap.solid);
</script>

<div
	class={cn(
		"flex items-center gap-2 overflow-hidden rounded-md",
		toneClass,
		densityClass,
		borderClass,
	)}
	{role}
	aria-label={ariaLabel}
>
	{#if icon}
		<svelte:component this={icon} class="h-4 w-4 shrink-0" />
	{:else if iconName}
		<Icon name={iconName} size="sm" />
	{/if}
	{#if children}
		<div class="min-w-0 flex-1">
			{@render children()}
		</div>
	{/if}
</div>
