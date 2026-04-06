<script lang="ts">
	import type { Component } from "svelte";
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
		success: "text-success",
		warning: "text-warning",
		destructive: "text-destructive",
		foreground: "text-foreground",
	};

	let {
		name,
		size = "md",
		registry,
		rotate90 = false,
		tone,
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
		tone?: "muted" | "success" | "warning" | "destructive" | "foreground";
	} = $props();

	const IconComponent = $derived(resolveIcon(name, registry));
	const toneClass = $derived(tone != null ? TONE_CLASSES[tone] : undefined);
</script>

<IconComponent
	class="shrink-0 {SIZE_CLASSES[size]} {rotate90
		? 'rotate-90 transition-transform'
		: 'transition-transform'} {toneClass ?? ''}"
/>
