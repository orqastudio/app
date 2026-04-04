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
		children,
	}: {
		ref?: HTMLElement | null;
		viewportRef?: HTMLElement | null;
		orientation?: "vertical" | "horizontal" | "both";
		/** When true, expands to fill the full height of the parent. */
		full?: boolean;
		/** Constrains the scroll area to a named height token. */
		maxHeight?: "sm" | "md" | "lg" | "xl" | "viewport";
		children?: import("svelte").Snippet;
	} = $props();

	const heightClass = $derived(
		full ? "h-full" : maxHeight ? maxHeightMap[maxHeight] : undefined,
	);
</script>

<ScrollAreaPrimitive.Root
	bind:ref
	data-slot="scroll-area"
	class="relative {heightClass ?? ''}"
>
	<ScrollAreaPrimitive.Viewport
		bind:ref={viewportRef}
		data-slot="scroll-area-viewport"
		class="size-full rounded-[inherit]"
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
