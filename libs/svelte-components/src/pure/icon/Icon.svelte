<script lang="ts">
	import type { Component } from "svelte";
	import { cn } from "../../utils/cn.js";
	import { resolveIcon } from "./icon-utils.js";

	const SIZE_CLASSES = {
		xs: "h-3 w-3",
		sm: "h-3.5 w-3.5",
		md: "h-4 w-4",
		lg: "h-5 w-5",
		xl: "h-6 w-6",
	} as const;

	const TONE_CLASSES: Record<string, string> = {
		muted: "text-muted-foreground",
		info: "text-info",
		success: "text-success",
		warning: "text-warning",
		destructive: "text-destructive",
		foreground: "text-foreground",
	};

	// Circle variant: a 40px rounded-full container with a toned background.
	// Matches the bg-{tone}/10 pattern used across Callout and status indicators.
	const CIRCLE_BG_CLASSES: Record<string, string> = {
		muted: "bg-muted",
		info: "bg-info/10",
		success: "bg-success/10",
		warning: "bg-warning/10",
		destructive: "bg-destructive/10",
		foreground: "bg-foreground/10",
	};

	let {
		name,
		size = "md",
		registry,
		rotate90 = false,
		tone,
		circle = false,
	}: {
		/** Icon key to resolve (e.g. "target", "circle-dot", "shield") */
		name: string;
		/** Icon size */
		size?: keyof typeof SIZE_CLASSES;
		/** Optional custom icon registry to check before defaults */
		registry?: Record<string, Component>;
		/** When true, rotates the icon 90 degrees with a CSS transition (useful for collapsible chevrons). */
		rotate90?: boolean;
		/** Semantic color tone applied to the icon. Inherits from parent when omitted. */
		tone?: "muted" | "info" | "success" | "warning" | "destructive" | "foreground";
		/** When true, wraps the icon in a 40px circular toned background (h-10 w-10 rounded-full). */
		circle?: boolean;
	} = $props();

	const IconComponent = $derived(resolveIcon(name, registry));
	const toneClass = $derived(tone != null ? TONE_CLASSES[tone] : undefined);
	const circleBgClass = $derived(circle && tone != null ? CIRCLE_BG_CLASSES[tone] : undefined);
</script>

{#if circle}
	<!-- Circle variant: fixed 40px rounded-full container with tone-matched background. -->
	<div
		class={cn("flex h-10 w-10 shrink-0 items-center justify-center rounded-full", circleBgClass)}
	>
		<IconComponent
			class={cn(
				"shrink-0",
				SIZE_CLASSES[size],
				rotate90 ? "rotate-90 transition-transform" : "transition-transform",
				toneClass,
			)}
		/>
	</div>
{:else}
	<IconComponent
		class={cn(
			"shrink-0",
			SIZE_CLASSES[size],
			rotate90 ? "rotate-90 transition-transform" : "transition-transform",
			toneClass,
		)}
	/>
{/if}
