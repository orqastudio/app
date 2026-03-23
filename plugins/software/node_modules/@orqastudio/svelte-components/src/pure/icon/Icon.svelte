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

	let {
		name,
		size = "md",
		registry,
	}: {
		/** Icon key to resolve (e.g. "target", "circle-dot", "shield") */
		name: string;
		/** Icon size */
		size?: keyof typeof SIZE_CLASSES;
		/** Optional custom icon registry to check before defaults */
		registry?: Record<string, Component>;
	} = $props();

	const IconComponent = $derived(resolveIcon(name, registry));
</script>

<IconComponent class="shrink-0 {SIZE_CLASSES[size]}" />
