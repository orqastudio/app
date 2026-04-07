<!-- scroll-area — a cross-browser scrollable region using bits-ui primitives.
     Use full={true} to fill the parent height, maxHeight for a token-based cap,
     or heightPx for an explicit pixel height when the container size is computed
     at runtime (e.g. from a parent dashboard card allocation). -->
<script lang="ts">
	import { ScrollArea as ScrollAreaPrimitive } from "bits-ui";
	import Scrollbar from "./scroll-area-scrollbar.svelte";

	// Maps maxHeight token to Tailwind max-h classes.
	const maxHeightMap = {
		sm: "max-h-40",
		md: "max-h-64",
		lg: "max-h-96",
		xl: "max-h-[32rem]",
		viewport: "max-h-screen",
	} as const;

	let {
		ref = $bindable(null),
		viewportRef = $bindable(null),
		orientation = "vertical",
		full = false,
		maxHeight,
		heightPx,
		onscroll,
		children,
	}: {
		ref?: HTMLElement | null;
		viewportRef?: HTMLElement | null;
		orientation?: "vertical" | "horizontal" | "both";
		/** When true, expands to fill the full height of the parent. */
		full?: boolean;
		/** Constrains the scroll area to a named height token. */
		maxHeight?: "sm" | "md" | "lg" | "xl" | "viewport";
		/**
		 * Explicit pixel height for data-driven containers (e.g. dashboard card allocations).
		 * Takes precedence over maxHeight. Use full={true} for percentage-based height.
		 */
		heightPx?: number;
		/** Fired when the scroll viewport is scrolled. */
		onscroll?: (e: Event) => void;
		children?: import("svelte").Snippet;
	} = $props();

	const heightClass = $derived(full ? "h-full" : maxHeight ? maxHeightMap[maxHeight] : undefined);
</script>

<ScrollAreaPrimitive.Root
	bind:ref
	data-slot="scroll-area"
	class="relative overflow-hidden {heightClass ?? ''}"
	style={heightPx != null ? `height: ${heightPx}px` : undefined}
>
	<ScrollAreaPrimitive.Viewport
		bind:ref={viewportRef}
		data-slot="scroll-area-viewport"
		class="size-full rounded-[inherit]"
		{onscroll}
	>
		{@render children?.()}
	</ScrollAreaPrimitive.Viewport>
	{#if orientation === "vertical" || orientation === "both"}
		<Scrollbar orientation="vertical" />
	{/if}
	{#if orientation === "horizontal" || orientation === "both"}
		<Scrollbar orientation="horizontal" />
	{/if}
	<ScrollAreaPrimitive.Corner />
</ScrollAreaPrimitive.Root>
